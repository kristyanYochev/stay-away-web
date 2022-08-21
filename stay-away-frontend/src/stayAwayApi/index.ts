import { useEffect, useRef } from "react";
import { ServerEventMap } from "./serverEvents";

interface StayAwayAPI {
  on<E extends keyof ServerEventMap>(eventType: E, listener: (event: ServerEventMap[E]) => any): void;
}

type EventListeners = {
  [E in keyof ServerEventMap]: ((event: ServerEventMap[E]) => any)[];
};

interface WebsocketMessage<E extends keyof ServerEventMap> {
  event: E,
  data: ServerEventMap[E],
}

export default function useStayAway(lobbyId: string): StayAwayAPI {
  const ws = useRef<WebSocket | null>(null);
  let eventListeners = useRef<EventListeners>({
    "Error": [],
    "UsersUpdated": [],
  });

  useEffect(() => {
    const socket = new WebSocket(`ws://localhost:8080/lobbies/${lobbyId}`);

    const emit = <E extends keyof ServerEventMap>(eventType: E, event: ServerEventMap[E]) => {
      eventListeners.current[eventType].forEach(handle => handle(event));
    };
  
    const handleWebsocketMessage = <E extends keyof ServerEventMap>(msg: MessageEvent<any>) => {
      console.log(`Received message: ${msg}`);
      const {event: eventType, data: event} = JSON.parse(msg.data) as WebsocketMessage<E>;
      emit(eventType, event);
    };

    socket.onmessage = handleWebsocketMessage;

    ws.current = socket;

    return () => socket.close();
  }, [lobbyId]);

  const subscribe = <E extends keyof ServerEventMap>(eventType: E, listener: (event: ServerEventMap[E]) => any) => {
    eventListeners.current[eventType].push(listener);
  }

  return {
    on: subscribe
  }
}
