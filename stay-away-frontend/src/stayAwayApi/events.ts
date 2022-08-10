interface UsersUpdatedEvent {
  users: string[];
}

interface ErrorEvent {}

export interface EventMap {
  "UsersUpdated": UsersUpdatedEvent;
  "Error": ErrorEvent
}
