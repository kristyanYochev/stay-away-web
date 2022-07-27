import React, { useState } from 'react';

const Home: React.FC = () => {
  const [username, setUsername] = useState('');

  return (
    <>
      <h1>Home Page</h1>
      <form>
        <label htmlFor='username'>
          Your username:
          <input
            type='text'
            placeholder='Definetly the thing'
            id='username'
            value={username}
            onChange={e => setUsername(e.target.value)}
          />
        </label>
      </form>
    </>
  )
};

export default Home;
