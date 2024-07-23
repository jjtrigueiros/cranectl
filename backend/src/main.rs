use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;

mod crane;
use crane::Crane;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    // Actuator parameters in meters (these should match in the frontend for IK to work)
    // We can add more parameters as needed for different operations.
    const D2_MAX: f64 = 2.0; // crane height
    const D3: f64 = -0.1; // elbow displacement
    const D4: f64 = -0.5; // wrist displacement
    const R3: f64 = 0.6; // upper arm length
    const R4: f64 = 0.6; // forearm length

    // our application state (separate mutexes for independent locking)
    let server_settings_arcmx = Arc::new(Mutex::new(ServerSettings {
        refresh_ms: 16, // for reference, 16ms = 60fps (excl. overhead)
    }));
    let crane_arcmx = Arc::new(Mutex::new(crane::Crane::new(
        D2_MAX, D3, D4, R3, R4,
    )));

    // thread to update the crane state in response to passage of time
    tokio::spawn(update_crane_state(
        crane_arcmx.clone(),
        16,
    ));

    // spawn new connection handler, passing atomic references to shared state
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(
            stream,
            server_settings_arcmx.clone(),
            crane_arcmx.clone()
        ));
    }
}

async fn handle_connection(stream: TcpStream, server_settings_arcmx: Arc<Mutex<ServerSettings>>, crane_arcmx: Arc<Mutex<Crane>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("An error occurred during the websocket handshake");
    println!("New WebSocket connection");

    let (ws_sender, ws_receiver) = ws_stream.split();
    tokio::spawn(handle_incoming_commands(ws_receiver, server_settings_arcmx.clone(), crane_arcmx.clone()));
    tokio::spawn(send_periodic_messages(ws_sender, server_settings_arcmx.clone(), crane_arcmx.clone()));
}

async fn handle_incoming_commands(
    mut receiver: SplitStream<WebSocketStream<TcpStream>>,
    server_settings_arcmx: Arc<Mutex<ServerSettings>>,
    crane_arcmx: Arc<Mutex<Crane>>,
) {
    while let Some(message) = receiver.next().await {
        let message = message.expect("Failed to get message");
        if message.is_text() {
            let incoming_message = message.to_text().unwrap();
            match parse_command(incoming_message) {
                Ok(cmd) => handle_command(cmd, &server_settings_arcmx, &crane_arcmx),
                Err(err_msg) => println!("Failed to parse '{}': {}", message, err_msg),
            };
        }
    }
}

async fn send_periodic_messages(
    mut ws_sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    server_settings_arcmx: Arc<Mutex<ServerSettings>>,
    crane_arcmx: Arc<Mutex<Crane>>,
) {
    loop {
        let interval = {
            let state = server_settings_arcmx.lock().unwrap();
            state.refresh_ms
        };
        time::sleep(Duration::from_millis(interval)).await;
        let msg = {
            let crane_state = crane_arcmx.lock().unwrap().get_state();
            Message::text(format!(
                "{} {} {} {} {}",
                crane_state.swing_deg, crane_state.lift_mm, crane_state.elbow_deg, crane_state.wrist_deg, crane_state.gripper_mm
            ))
        };
        if ws_sender.send(msg).await.is_err() {
            println!("Failed to send message");
            break;
        }
    }
}

async fn update_crane_state(crane_arcmx: Arc<Mutex<Crane>>, dt_ms: u16) {
    let mut interval = time::interval(Duration::from_millis(dt_ms.into()));

    loop {
        interval.tick().await;
        let mut crane = crane_arcmx.lock().unwrap();
        let dt_seconds = f64::from(dt_ms) / 1000.0;
        crane.update_state(dt_seconds);
    }
}

struct ServerSettings {
    refresh_ms: u64,
}

enum Command {
    SetCraneActuatorSetpoints { swing_deg: f64, lift_m: f64, elbow_deg: f64, wrist_deg: f64, gripper_m: f64 },
    SetCraneSetpoint { x: f64, y: f64, z: f64 },
    SetRefresh { ms: u32 },
}

fn parse_command(command: &str) -> Result<Command, String> {
    let command_and_args: Vec<&str> = command.trim().split_whitespace().collect();

    match command_and_args.as_slice() {
        ["setactuatorsetpoints", swing_deg, lift_mm, elbow_deg, wrist_deg, gripper_mm] => {
            let swing_deg = swing_deg.parse::<f64>().map_err(|_| "Invalid swing degrees (pos. 1)")?;
            let lift_mm = lift_mm.parse::<f64>().map_err(|_| "Invalid lift mm (pos. 2)")?;
            let elbow_deg = elbow_deg.parse::<f64>().map_err(|_| "Invalid elbow degrees (pos. 3)")?;
            let wrist_deg = wrist_deg.parse::<f64>().map_err(|_| "Invalid wrist degrees (pos. 4)")?;
            let gripper_mm = gripper_mm.parse::<f64>().map_err(|_| "Invalid gripper mm (pos. 5)")?;
            Ok(Command::SetCraneActuatorSetpoints {
                swing_deg,
                lift_m: lift_mm * 0.001,
                elbow_deg,
                wrist_deg,
                gripper_m: gripper_mm * 0.001,
            })
        }
        ["setpoint", x, y, z] => {
            Ok(Command::SetCraneSetpoint { 
                x: x.parse::<f64>().map_err(|_| "Invalid x coordinate")?,
                y: y.parse::<f64>().map_err(|_| "Invalid y coordinate")?,
                z: z.parse::<f64>().map_err(|_| "Invalid z coordinate")?
            })
        }
        ["refresh", ms] => {
            let ms = ms.parse::<u32>().map_err(|_| "Invalid ms refresh value")?;
            Ok(Command::SetRefresh { ms })
        }
        _ => Err("Invalid command".to_string()),
    }
}

fn handle_command(command: Command, server_settings: &Arc<Mutex<ServerSettings>>, crane_arcmx: &Arc<Mutex<Crane>>) {
    match command {
        Command::SetCraneActuatorSetpoints { swing_deg, lift_m, elbow_deg, wrist_deg, gripper_m } => {
            let mut crane = crane_arcmx.lock().unwrap();
            crane.set_actuator_setpoints(swing_deg, lift_m, elbow_deg, wrist_deg, gripper_m)
        }
        Command::SetCraneSetpoint { x, y, z } => {
            let mut crane = crane_arcmx.lock().unwrap();
            crane.set_crane_setpoint(x, y, z)
        }
        Command::SetRefresh { ms } => {
            let mut state = server_settings.lock().unwrap();
            state.refresh_ms = ms.into();
        }
    }
}
