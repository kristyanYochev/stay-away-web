interface UsersUpdatedEvent {
  users: User[];
}

export type User = {
  username: string;
  id: number;
}

interface ErrorEvent {}

interface WelcomeEvent {
  id: number;
}

interface StartGameEvent {}

export interface ServerEventMap {
  "UsersUpdated": UsersUpdatedEvent;
  "Error": ErrorEvent;
  "Welcome": WelcomeEvent;
  "StartGame": StartGameEvent;
}
