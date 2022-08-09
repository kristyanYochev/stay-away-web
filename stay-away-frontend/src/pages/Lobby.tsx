import React, { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";

const Lobby: React.FC = () => {
  const [username, setUsername] = useState("");
  const ws = useRef<WebSocket | null>(null);

  const { lobbyId } = useParams();

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);
    socket.onopen = () => console.log(`Socket opened to ${lobbyId}`);
    socket.onclose = () => console.log(`Socket to ${lobbyId} is closed`);
    socket.onmessage = console.log;

    ws.current = socket;

    return () => socket.close();
  }, [lobbyId]);

  const joinRoom = () => {
    console.log(ws.current);
    ws.current?.send(JSON.stringify({
      event: "Join",
      data: {
        username
      }
    }));
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
