import React, { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import useStayAway from "../stayAwayApi";

const Lobby: React.FC = () => {
  const [username, setUsername] = useState("");
  const [users, setUsers] = useState<string[]>([]);

  const { lobbyId } = useParams();

  const ws = useStayAway(lobbyId || "");

  const joinRoom = () => {
    console.log(ws);
    ws?.send(JSON.stringify({
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
      <br />
      <ul>
        {users.map((user, i) => (
          <li key={i}>
            {user}
          </li>
        ))}
      </ul>
    </>
  );
};

export default Lobby;
