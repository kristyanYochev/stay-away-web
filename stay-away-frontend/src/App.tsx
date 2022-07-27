import React, { useEffect, useRef } from 'react';
import './App.css';
import Home from './pages/Home';

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
    <Home />
  );
}

export default App;
