import { useEffect, useRef } from "react";
import { EventMap } from "./events";

interface StayAwayAPI {
  on<E extends keyof EventMap>(eventType: E, listener: (event: EventMap[E]) => any): void;
}

type EventListeners = {
  [E in keyof EventMap]: ((event: EventMap[E]) => any)[];
};

export default function useStayAway(lobbyId: string): StayAwayAPI {
  const ws = useRef<WebSocket | null>(null);

  let eventListeners: EventListeners = {
    "Error": [],
    "UsersUpdated": [],
  };

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);
    socket.onmessage = console.log;

    ws.current = socket;

    return () => socket.close();
  }, [lobbyId]);

  const subscribe = <E extends keyof EventMap>(eventType: E, listener: (event: EventMap[E]) => any) => {
    eventListeners[eventType].push(listener);
  }

  return {
    on: subscribe
  }
}
