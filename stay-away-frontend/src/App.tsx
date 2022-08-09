import React, { useEffect, useRef } from 'react';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import './App.css';
import Home from './pages/Home';
import Lobby from './pages/Lobby';

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
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/lobby/:lobbyId" element={<Lobby />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
