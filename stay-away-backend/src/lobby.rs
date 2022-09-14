use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, oneshot};

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::events::{server::ServerEvent, self};

/// Represents a single game lobby
pub struct Lobby {
    id: String,
    /// A collection of all users currently joined in the lobby
    users: HashMap<UserId, User>,
    next_user_id: UserId
}

pub type Lobbies = Arc<RwLock<HashMap<String, LobbyHandle>>>;
pub type LobbyHandle = Sender<LobbyCommand>;

/// Represents a command that can be sent to the lobby via its handle
#[derive(Debug)]
pub enum LobbyCommand {
    /// A request for a user to join the lobby
    Join {
        username: String,
        user_id: UserId,
        user_handle: UserHandle
    },

    /// A command to assign an id to the connection.
    /// Fired as soon as a websocket is opened.
    AssignId {
        id_channel: oneshot::Sender<UserId>
    },

    /// A signal that a user has disconnected
    Disconnect {
        user_id: UserId
    },

    /// A signal to start the game
    StartGame,
}

impl Lobby {
    /// Creates a new Lobby. The users vec has an initial capacity of 12,
    /// since that is the maximum player count of the original board game.
    pub fn new(id: String) -> Self {
        Self {
            id,
            users: HashMap::default(),
            next_user_id: 1,
        }
    }

    /// An infinite loop handling all the commands for that lobby.
    pub async fn manage(mut self, mut rx: Receiver<LobbyCommand>) {
        use LobbyCommand::*;

        while let Some(command) = rx.recv().await {
            match command {
                Join { username, user_handle, user_id } => {
                    self.users.insert(
                        user_id,
                        User::new(
                            username.clone(),
                            user_handle.clone(),
                            user_id,
                        )
                    );

                    user_handle.send(ServerEvent::Welcome { id: user_id }).await.unwrap();

                    self.notify_user_list_update().await;
                },

                AssignId { id_channel } => {
                    id_channel.send(self.generate_user_id()).unwrap();
                },

                Disconnect { user_id } => {
                    self.users.remove(&user_id);

                    self.notify_user_list_update().await;
                },

                StartGame => {
                    let num_users = self.users.len();
                    if num_users >= 4 && num_users <= 12 {
                        self.broadcast(ServerEvent::StartGame).await;
                    }
                }
            }
        }
    }

    async fn notify_user_list_update(&self) {
        let update_event = ServerEvent::UsersUpdated {
            users: self.users.iter()
                .map(|(id, u)| events::server::User {
                    id: *id,
                    username: u.username.clone()
                }).collect()
        };

        self.broadcast(update_event).await;
    }

    async fn broadcast(&self, event: ServerEvent) {
        for (_id, user) in &self.users {
            user.handle.send(event.clone()).await.unwrap();
        }
    }

    /// Grabs the next_user_id and increments it for the next call.
    fn generate_user_id(&mut self) -> UserId {
        let next_id = self.next_user_id;
        self.next_user_id += 1;
        next_id
    }
}

/// A representation of a single user joined in a lobby
struct User {
    username: String,
    handle: UserHandle,
    id: UserId,
}

type UserHandle = Sender<ServerEvent>;
type UserId = usize;

impl User {
    /// Creates new user
    fn new(username: String, handle: UserHandle, id: UserId) -> Self {
        Self { username, handle, id }
    }
}

/// Generates a unique and random lobby id
pub async fn generate_id(lobbies: &Lobbies) -> String {
    let mut id = random_id();

    while lobbies.read().await.contains_key(&id) {
        id = random_id();
    }

    id
}

/// Generates a random string of 6 alphanumeric characters.
fn random_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::thread_rng;

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;
    use tokio::sync::oneshot;
    use tokio::task::JoinHandle;

    use crate::events::server::ServerEvent;

    use super::{LobbyHandle, Lobby};

    fn start_lobby() -> (LobbyHandle, JoinHandle<()>) {
        let lobby = Lobby::new("aaaaaa".to_string());
        let (tx, rx) = mpsc::channel(32);
        (tx, tokio::spawn(lobby.manage(rx)))
    }

    async fn teardown(lobby: LobbyHandle, join_handle: JoinHandle<()>) {
        drop(lobby);
        tokio::join!(join_handle).0.unwrap();
    }

    async fn get_id(lobby: LobbyHandle) -> usize {
        let (id_tx, id_rx) = oneshot::channel();
        lobby.send(super::LobbyCommand::AssignId { id_channel: id_tx }).await.unwrap();
        id_rx.await.unwrap()
    }

    async fn join_lobby(lobby: LobbyHandle, username: String) -> (usize, mpsc::Receiver<ServerEvent>) {
        let id = get_id(lobby.clone()).await;
        let (tx, rx) = mpsc::channel(32);
        lobby.send(super::LobbyCommand::Join { username, user_id: id, user_handle: tx }).await.unwrap();

        (id, rx)
    }

    #[tokio::test]
    async fn lobby_assigns_consecutive_ids() {
        let (lobby, join_handle) = start_lobby();

        let id1 = get_id(lobby.clone()).await;
        let id2 = get_id(lobby.clone()).await;

        assert!(id1 < id2);
        assert!(id2 - id1 == 1);

        teardown(lobby, join_handle).await;
    }

    #[tokio::test]
    async fn lobby_sends_welcome_message_to_joined_client() {
        let (lobby, join_handle) = start_lobby();

        let my_id = get_id(lobby.clone()).await;

        let (my_tx, mut my_rx) = mpsc::channel(32);

        lobby.send(super::LobbyCommand::Join {
            username: "test".to_string(),
            user_id: my_id,
            user_handle: my_tx.clone()
        }).await.unwrap();

        let received_message = my_rx.recv().await.unwrap();

        if let ServerEvent::Welcome { id: received_id } = received_message {
            assert!(received_id == my_id);
        } else {
            assert!(false);
        }

        teardown(lobby, join_handle).await;
    }

    #[tokio::test]
    async fn lobby_sends_user_updated_when_a_client_joins() {
        let (lobby, join_handle) = start_lobby();

        let (id1, mut rx1) = join_lobby(lobby.clone(), "user1".to_string()).await;
        let (id2, mut rx2) = join_lobby(lobby.clone(), "user2".to_string()).await;

        rx1.recv().await.unwrap(); // The welcome message
        rx2.recv().await.unwrap();

        let msg1 = rx1.recv().await.unwrap();
        if let ServerEvent::UsersUpdated { users } = msg1 {
            assert!(users.len() == 1);
            assert!(users[0].id == id1);
            assert!(users[0].username == "user1".to_string());
        } else {
            assert!(false);
        }

        let msg2 = rx1.recv().await.unwrap();
        if let ServerEvent::UsersUpdated { mut users } = msg2 {
            assert!(users.len() == 2);
            // TODO: Add additional checks for ids and usernames
        } else {
            assert!(false);
        }

        let msg3 = rx2.recv().await.unwrap();
        if let ServerEvent::UsersUpdated { mut users } = msg3 {
            assert!(users.len() == 2);
            // TODO: Add additional checks for ids and usernames
        } else {
            assert!(false);
        }

        teardown(lobby, join_handle).await;
    }
}