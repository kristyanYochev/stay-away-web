interface UsersUpdatedEvent {
  users: string[];
}

interface ErrorEvent {}

export interface ServerEventMap {
  "UsersUpdated": UsersUpdatedEvent;
  "Error": ErrorEvent;
}
