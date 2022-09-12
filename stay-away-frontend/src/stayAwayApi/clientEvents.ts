interface JoinEvent {
  username: string
}

interface StartGameEvent {}

export interface ClientEventMap {
  "Join": JoinEvent;
  "StartGame": StartGameEvent
}