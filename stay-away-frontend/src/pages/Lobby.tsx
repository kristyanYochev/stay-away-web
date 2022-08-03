import React, { useState } from "react";
import { useParams } from "react-router-dom";

const Lobby: React.FC = () => {
  const [username, setUsername] = useState("");

  const { lobbyId } = useParams();

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
