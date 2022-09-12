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

        for (_id, user) in &self.users {
            user.handle.send(update_event.clone()).await.unwrap();
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
