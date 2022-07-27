import React, { useEffect, useRef } from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080/echo");
    socket.onopen = () => console.log("Socket open");
    socket.onmessage = ({ data }) => console.log("Message received: " + data);

    ws.current = socket;

    return () => socket.close();
  }, []);

  const sendWsMessage = () => {
    ws.current?.send("Test message, should echo");
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <button onClick={sendWsMessage}>Test message</button>
      </header>
    </div>
  );
}

export default App;
