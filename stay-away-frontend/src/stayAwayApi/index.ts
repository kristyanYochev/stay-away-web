import { useEffect, useRef } from "react";
import { EventMap } from "./events";

interface StayAwayAPI {
  on<E extends keyof EventMap>(eventType: E, listener: (event: EventMap[E]) => any): void;
}

type EventListeners = {
  [E in keyof EventMap]: ((event: EventMap[E]) => any)[];
};

interface WebsocketMessage<E extends keyof EventMap> {
  event: E,
  data: EventMap[E],
}

export default function useStayAway(lobbyId: string): StayAwayAPI {
  const ws = useRef<WebSocket | null>(null);

  let eventListeners: EventListeners = {
    "Error": [],
    "UsersUpdated": [],
  };

  const emit = <E extends keyof EventMap>(eventType: E, event: EventMap[E]) => {
    eventListeners[eventType].forEach(handle => handle(event));
  }

  const handleWebsocketMessage = <E extends keyof EventMap>(msg: MessageEvent<any>) => {
    const {event: eventType, data: event} = JSON.parse(msg.data) as WebsocketMessage<E>;
    emit(eventType, event);
  }

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);
    socket.onmessage = handleWebsocketMessage;

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
