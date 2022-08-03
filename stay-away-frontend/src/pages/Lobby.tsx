import React, { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";

const Lobby: React.FC = () => {
  const [username, setUsername] = useState("");
  const ws = useRef<WebSocket>(null);

  const { lobbyId } = useParams();

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);
    socket.onmessage = console.log;

    return () => socket.close();
  }, []);

  const joinRoom = () => {
    
  }

  return (
    <>
      <h1>Lobby {lobbyId}</h1>
      <input
        value={username}
        onChange={e => setUsername(e.target.value)}
        placeholder="Your awesome username"
      />
      <button onClick={joinRoom}>Join Room</button>
    </>
  );
};

export default Lobby;
