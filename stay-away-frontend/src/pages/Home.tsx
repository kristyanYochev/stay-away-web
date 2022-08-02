import React, { useState } from 'react';

const Home: React.FC = () => {
  const [lobbyId, setLobbyId] = useState('');
  const [lobbyCreated, setLobbyCreated] = useState(false);

  const onCreateLobby = async () => {
    const response = await fetch('http://localhost:8080/lobbies', {
      method: 'POST'
    });

    const id = await response.text();

    console.log('Created lobby with id ' + id);

    setLobbyId(id);
    setLobbyCreated(true);
  }

  return (
    <div>
      <button onClick={onCreateLobby}>Create a lobby</button>
      <input
        type='text'
        placeholder='Lobby Id'
        value={lobbyId}
        onChange={e => setLobbyId(e.target.value)}
        disabled={lobbyCreated}
      />
      <button onClick={() => {}}>Join Lobby</button>
    </div>
  )
}

export default Home;
