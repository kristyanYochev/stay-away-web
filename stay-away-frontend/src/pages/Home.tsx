import React, { useState } from 'react';

const Home: React.FC = () => {
  const [username, setUsername] = useState('');
  const [lobbyId, setLobbyId] = useState('');

  return (
    <>
      <h1>Home Page</h1>
      <form>
        <label htmlFor='username'>
          Your username:
          <br />
          <input
            type='text'
            placeholder='Definetly the thing'
            id='username'
            value={username}
            onChange={e => setUsername(e.target.value)}
          />
        </label>
        <br />
        <button type='button'>Create lobby</button>
        or
        <label htmlFor='lobyId'>
          <button type='button'>Join an existing lobby</button>
          <input
            type='text'
            placeholder='Lobby Id'
            id="lobbyId"
            value={lobbyId}
            onChange={e => setLobbyId(e.target.value)}
          />
        </label>
      </form>
    </>
  )
};

export default Home;
