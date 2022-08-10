import { useEffect, useRef } from "react";

export default function useStayAway(lobbyId: string) {
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);
    socket.onmessage = console.log;

    ws.current = socket;

    return () => socket.close();
  }, [lobbyId]);

  return ws.current;
}
