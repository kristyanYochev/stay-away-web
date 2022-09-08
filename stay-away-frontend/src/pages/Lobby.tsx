import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useStayAway, StayAwayProvider } from "../stayAwayApi/index";
import { User } from "../stayAwayApi/serverEvents";

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

  useEffect(() => {
    stayAway.on("Welcome", ({ id }) => {
      console.log(`My id is: ${id}`);
    });
  }, [stayAway]);

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
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    stayAway.on("UsersUpdated", evt => {
      console.log("Users Updated!");
      setUsers(evt.users);
    });
  }, [stayAway])

  return (
    <ul>
      {users.map((user) => (
        <li key={user.id}>{user.username}</li>
      ))}
    </ul>
  )
}

export default Lobby;
