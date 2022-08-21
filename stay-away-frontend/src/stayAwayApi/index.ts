import { useEffect, useRef } from "react";
import { ServerEventMap } from "./serverEvents";
import { ClientEventMap } from "./clientEvents";

type ServerEventType = keyof ServerEventMap;
type ServerEvent<E extends ServerEventType> = ServerEventMap[E];
type ServerEventListener<E extends ServerEventType> = (event: ServerEvent<E>) => any;

type ClientEventType = keyof ClientEventMap;
type ClientEvent<E extends ClientEventType> = ClientEventMap[E];

interface StayAwayAPI {
  on<E extends ServerEventType>(eventType: E, listener: ServerEventListener<E>): void;
  send<E extends ClientEventType>(eventType: E, event: ClientEvent<E>): Promise<void>;
}

type EventListeners = {
  [E in ServerEventType]: ((event: ServerEventMap[E]) => any)[];
};

interface WebsocketMessage<E extends ServerEventType> {
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

    const emit = <E extends ServerEventType>(eventType: E, event: ServerEventMap[E]) => {
      eventListeners.current[eventType].forEach(handle => handle(event));
    };
  
    const handleWebsocketMessage = <E extends ServerEventType>(msg: MessageEvent<any>) => {
      const {event: eventType, data: event} = JSON.parse(msg.data) as WebsocketMessage<E>;
      emit(eventType, event);
    };

    socket.onmessage = handleWebsocketMessage;

    ws.current = socket;

    return () => socket.close();
  }, [lobbyId]);

  const subscribe = <E extends ServerEventType>(eventType: E, listener: (event: ServerEventMap[E]) => any) => {
    eventListeners.current[eventType].push(listener);
  }

  const send = async <E extends ClientEventType>(eventType: E, event: ClientEvent<E>) => {
    if (!ws.current) {
      console.error("No WebSocket");
      return;
    }

    if (ws.current.readyState !== ws.current.OPEN) {
      console.error("Websocket not connected");
      return;
    }

    ws.current.send(JSON.stringify({
      event: eventType,
      data: event
    }));
  }

  return {
    on: subscribe,
    send
  }
}
