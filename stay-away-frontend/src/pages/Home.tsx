import React, { useState } from 'react';

interface LobbiesProps {
  onJoinLobby?: (lobbyId: string) => void
}

const Lobbies: React.FC<LobbiesProps> = ({ onJoinLobby = () => {} }) => {
  const [lobbyId, setLobbyId] = useState('');

  const onCreateLobby = async () => {
    const response = await fetch('http://localhost:8080/lobbies', {
      method: 'POST'
    });

    const id = await response.text();

    console.log('Created lobby with id ' + id);

    setLobbyId(id);
  }

  return (
    <div>
      <button onClick={onCreateLobby}>Create a lobby</button>
      <input
        type='text'
        placeholder='Lobby Id'
        value={lobbyId}
        onChange={e => setLobbyId(e.target.value)}
      />
      <button onClick={() => onJoinLobby(lobbyId)}>Join Lobby</button>
    </div>
  )
}

const Home: React.FC = () => {
  const [username, setUsername] = useState('');
  const [loggedIn, setLoggedIn] = useState(false);

  const onLogin: React.FormEventHandler<HTMLFormElement> = (event) => {
    event.preventDefault();
    setLoggedIn(true);
  }

  return (
    <>
      <h1>Home Page</h1>
      <form onSubmit={onLogin}>
        <label htmlFor='username'>
          Your username:
          <br />
          <input
            type='text'
            placeholder='Definetly the thing'
            id='username'
            value={username}
            onChange={e => setUsername(e.target.value)}
            disabled={loggedIn}
          />
        </label>
        <br />
        <button type='submit'>Login</button>
        {loggedIn && <Lobbies />}
      </form>
    </>
  )
};

export default Home;
