import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import useStayAway, { StayAwayProvider } from "../stayAwayApi";

const Lobby = () => {
  const { lobbyId } = useParams();

  if (!lobbyId) {
    throw new Error("Lobby is undefined");
  }

  return (
    <StayAwayProvider lobbyId={lobbyId}>
      <h1>Lobby {lobbyId}</h1>
      <UserJoin />
      <br />
      <UserList />
    </StayAwayProvider>
  );
};

const UserJoin = () => {
  const [username, setUsername] = useState("");

  const stayAway = useStayAway();

  const joinRoom = () => {
    stayAway.send("Join", {username});
  }

  return (
    <>
      <input
        value={username}
        onChange={e => setUsername(e.target.value)}
        placeholder="Your awesome username"
      />
      <button onClick={joinRoom}>Join Room</button>
    </>
  )
}

const UserList = () => {
  const stayAway = useStayAway();
  const [users, setUsers] = useState<string[]>([]);

  useEffect(() => {
    stayAway.on("UsersUpdated", evt => {
      console.log("Users Updated!");
      setUsers(evt.users);
    });
  }, [stayAway])

  return (
    <ul>
      {users.map((user, i) => (
        <li key={i}>{user}</li>
      ))}
    </ul>
  )
}

export default Lobby;
