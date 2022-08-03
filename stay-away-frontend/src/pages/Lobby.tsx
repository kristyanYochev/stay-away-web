import React from "react";
import { useParams } from "react-router-dom";

const Lobby: React.FC = () => {
  const { lobbyId } = useParams();

  return (
    <h1>Lobby {lobbyId}</h1>
  );
};

export default Lobby;
