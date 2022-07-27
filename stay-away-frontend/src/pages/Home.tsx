import React, { useState } from 'react';

const Lobbies: React.FC = ({ onCreateLobby }: { onCreateLobby: () => void }) => {
  return (
    <div>
      <button onClick={() => onCreateLobby()}>Create a lobby</button>
    </div>
  )
}

const Home: React.FC = () => {
  const [username, setUsername] = useState('');

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
        <button type='submit'>Login</button>
      </form>
    </>
  )
};

export default Home;
