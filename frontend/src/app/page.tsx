"use client";
import { useEffect, useState } from 'react';
import ThreeCanvas from "./components/ThreeCanvas"
import ControlPanel from './components/ControlPanel';

interface CraneState {
  swing_deg: number;
  lift_mm: number;
  elbow_deg: number;
  wrist_deg: number;
  gripper_mm: number;
}


export default function Home() {
  const [ws, setWs] = useState<any | null>(null);
  // default crane state before websocket connection is established
  const [craneState, setCraneState] = useState<CraneState>({
    swing_deg: 0,
    lift_mm: 1000,
    elbow_deg: 0,
    wrist_deg: 0,
    gripper_mm: 0,
  });

  useEffect(() => {
    const websocket = new WebSocket('ws://127.0.0.1:8080');

    websocket.onopen = () => {
      console.log('Connected to WebSocket server');
    };

    websocket.onmessage = (event) => {
      const data = event.data.split(" ").map(Number);
      setCraneState({
        swing_deg: data[0],
        lift_mm: data[1],
        elbow_deg: data[2],
        wrist_deg: data[3],
        gripper_mm: data[4]
      });
    };

    websocket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    websocket.onclose = () => {
      console.log('WebSocket connection closed');
    };

    setWs(websocket);
    return () => websocket.close();
  }, []);

  const sendMessage = (message: string) => {
    if (ws && message) {
      ws.send(message);
    }
  };

  const handleSpeedSubmit = (swing: number, lift: number, elbow: number, wrist: number, gripper: number) => {
    sendMessage(`setspeed ${swing} ${lift} ${elbow} ${wrist} ${gripper}`)
  }

  const handleActuatorSetpointSubmit = (swing: number, lift: number, elbow: number, wrist: number, gripper: number) => {
    sendMessage(`setactuatorsetpoints ${swing} ${lift} ${elbow} ${wrist} ${gripper}`)
  }

  const handlePositionSubmit = (x: number, y: number, z:number) => {
    sendMessage(`setpoint ${x} ${y} ${z}`)
  }

  const handleOriginChange = () => {
    // to do: origin change
  }

  return (
    <div className='grid h-screen grid-cols-1 grid-rows-1 md:grid-cols-5 md:grid-rows-1'>

      <div className='col-span-4 row-span-1'>
        <ThreeCanvas
          swing_deg={craneState.swing_deg}
          lift_mm={craneState.lift_mm}
          elbow_deg={craneState.elbow_deg}
          wrist_deg={craneState.wrist_deg}
          gripper_mm={craneState.gripper_mm}
        />
      </div>
      <div className='col-span-1 row-span-1 p-4'>
        <ControlPanel
          onSpeedSubmit={handleSpeedSubmit}
          onActuatorSetpointSubmit={handleActuatorSetpointSubmit}
          onPositionSubmit={handlePositionSubmit}
        />
      </div>
    </div>
  );
}
