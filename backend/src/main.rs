use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{self, Duration};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
mod crane;
use crane::{Crane, CraneState};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    // our application state (separate mutexes for independent locking)
    let crane = Arc::new(Mutex::new(crane::Crane::new()));
    let server_settings = Arc::new(Mutex::new(ServerSettings {
        refresh_ms: 1000, // for reference, 16ms = 60fps (excl. overhead)
    }));

    // spawn new connection handler, passing atomic references to shared state
    while let Ok((stream, _)) = listener.accept().await {
        let server_settings_clone = server_settings.clone();
        let crane_clone = crane.clone();
        tokio::spawn(handle_connection(stream, server_settings_clone, crane_clone));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, server_settings: Arc<Mutex<ServerSettings>>, crane: Arc<Mutex<Crane>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("New WebSocket connection");

    let (mut ws_sender, ws_receiver) = ws_stream.split();

    // Task to handle incoming commands
    let server_settings_clone = server_settings.clone();
    let shared_crane_state_clone = crane.clone();
    let mut receiver = ws_receiver;
    tokio::spawn(async move {
        while let Some(message) = receiver.next().await {
            let message = message.expect("Failed to get message");
            if message.is_text() {
                let incoming_message = message.to_text().unwrap();
                let cmd_res = parse_command(incoming_message);
                match cmd_res {
                    Ok(cmd) => {
                        handle_command(cmd, &server_settings_clone, &shared_crane_state_clone);
                    }
                    Err(err_msg) => println!("Failed to parse '{}': {}", message, err_msg),
                };
            }
        }
    });

    // Task to send periodic messages
    let server_settings_clone = server_settings.clone();
    let shared_crane_state_clone = crane.clone();
    tokio::spawn(async move {
        loop {
            let interval = {
                let state = server_settings_clone.lock().unwrap();
                state.refresh_ms
            };
            time::sleep(Duration::from_millis(interval)).await;
            let msg = {
                let crane_state = shared_crane_state_clone.lock().unwrap().get_state();
                Message::text(format!("{} {} {} {} {}", crane_state.swing_deg, crane_state.lift_mm, crane_state.elbow_deg, crane_state.wrist_deg, crane_state.gripper_mm))
            };
            if ws_sender.send(msg).await.is_err() {
                println!("Failed to send message");
                break;
            }
        }
    });
}

struct ServerSettings {
    refresh_ms: u64,
}


enum Command {
    SetPosition { x: f64, y: f64, z: f64 },
    SetCraneState (CraneState),
    SetRefresh { ms: u32 },
}

fn parse_command(command: &str) -> Result<Command, String> {
    let command_and_args: Vec<&str> = command.trim().split_whitespace().collect();

    match command_and_args.as_slice() {
        ["position", x, y, z] => {
            let x = x.parse::<f64>().map_err(|_| "Invalid x coordinate")?;
            let y = y.parse::<f64>().map_err(|_| "Invalid y coordinate")?;
            let z = z.parse::<f64>().map_err(|_| "Invalid z coordinate")?;
            Ok(Command::SetPosition { x, y, z })
        }
        ["cranestate", swing_deg, lift_mm, elbow_deg, wrist_deg, gripper_mm] => {
            let swing_deg = swing_deg.parse::<f64>().map_err(|_| "Invalid swing degrees (pos. 1)")?;
            let lift_mm = lift_mm.parse::<f64>().map_err(|_| "Invalid lift mm (pos. 2)")?;
            let elbow_deg = elbow_deg.parse::<f64>().map_err(|_| "Invalid elbow degrees (pos. 3)")?;
            let wrist_deg = wrist_deg.parse::<f64>().map_err(|_| "Invalid wrist degrees (pos. 4)")?;
            let gripper_mm = gripper_mm.parse::<f64>().map_err(|_| "Invalid gripper mm (pos. 5)")?;
            Ok(Command::SetCraneState(CraneState{
                swing_deg,
                lift_mm,
                elbow_deg,
                wrist_deg,
                gripper_mm,
        }))
        }
        ["refresh", ms] => {
            let ms = ms.parse::<u32>().map_err(|_| "Invalid ms refresh value")?;
            Ok(Command::SetRefresh { ms })
        }
        _ => Err("Invalid command".to_string()),
    }
}

fn handle_command(command: Command, server_settings: &Arc<Mutex<ServerSettings>>, crane: &Arc<Mutex<Crane>>) {
    match command {
        Command::SetPosition { x, y, z } => {
            todo!()
        }
        Command::SetRefresh { ms } => {
            let mut state = server_settings.lock().unwrap();
            state.refresh_ms = ms.into();
        }
        Command::SetCraneState(cs) => {
            let mut crane = crane.lock().unwrap();
            crane.set_state(cs)
        }
    }
}
