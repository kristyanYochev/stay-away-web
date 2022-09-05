import { useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import useStayAway from "../stayAwayApi";

const Lobby = () => {
  const [username, setUsername] = useState("");
  const [users, setUsers] = useState<string[]>([]);

  const { lobbyId } = useParams();

  const stayAway = useStayAway();

  useEffect(() => {
    stayAway.on("UsersUpdated", evt => {
      console.log("Users Updated!");
      setUsers(evt.users);
    });
  }, [])

  const joinRoom = () => {
    stayAway.send("Join", {username});
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
      <UserList users={users}/>
    </>
  );
};

const UserList = ({users}: {users: string[]}) => {
  return (
    <ul>
      {users.map((user, i) => (
        <li key={i}>{user}</li>
      ))}
    </ul>
  )
}

export default Lobby;
