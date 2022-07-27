import React from 'react';

const Home: React.FC = () => {
  return (
    <>
      <h1>Home Page</h1>
      <form>
        <label htmlFor='username'>
          Your username:
          <input type='text' placeholder='Definetly the thing' id='username'/>
        </label>
      </form>
    </>
  )
};

export default Home;
