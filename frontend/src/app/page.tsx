"use client";
import { useEffect, useState } from 'react';
import ThreeCanvas from "./components/ThreeCanvas"

export default function Home() {
  const [craneState, setCraneState] = useState([0, 1000, 0, 0, 0]);
  const [inputValue, setInputValue] = useState('');
  const [ws, setWs] = useState<any | null>(null);

  useEffect(() => {
    const websocket = new WebSocket('ws://127.0.0.1:8080');

    websocket.onopen = () => {
      console.log('Connected to WebSocket server');
    };

    websocket.onmessage = (event) => {
      // possible improvement: communicate with a mapping instead of a list so that we
      // don't have to rely on the ordering
      console.log('Received:', event.data);
      setCraneState(event.data.split(" ").map(Number));
    };

    websocket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    websocket.onclose = () => {
      console.log('WebSocket connection closed');
    };

    setWs(websocket);

    return () => {
      websocket.close();
    };
  }, []);

  const sendMessage = () => {
    if (ws && inputValue) {
      ws.send(inputValue);
      setInputValue('');
    }
  };

  return (
    <div className='flex flex-col h-screen'>
      <div className='flex-grow flex justify-center items-center'>
        <ThreeCanvas swing_deg={craneState[0]} lift_mm={craneState[1]} elbow_deg={craneState[2]} wrist_deg={craneState[3]} gripper_mm={craneState[4]} />
      </div>
      <div className='h-1/5 flex justify-center items-center bg-green-500 text-white'>
        <h1 className="text-3xl font-bold underline">WebSocket Communication</h1>
        <p>Message from server: {" ".concat(craneState)}</p>
        <input
          type="text"
          className="text font-bold text-black"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          placeholder="Type a message"
        />
        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold px-5 py-2 mx-2 my-5 rounded" onClick={sendMessage}>Send Message</button>
      </div>
    </div>
  );
}
