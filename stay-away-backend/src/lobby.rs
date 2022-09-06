use std::{collections::HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::client::ServerEvent;

/// Represents a single game lobby
pub struct Lobby {
    id: String,
    /// A collection of all users currently joined in the lobby
    users: Vec<User>,
}

pub type Lobbies = Arc<RwLock<HashMap<String, LobbyHandle>>>;
pub type LobbyHandle = Sender<LobbyCommand>;

/// Represents a command that can be sent to the lobby via its handle
#[derive(Debug)]
pub enum LobbyCommand {
    /// A request for a user to join the lobby
    Join {
        username: String,
        user_handle: UserHandle
    },
}

impl Lobby {
    /// Creates a new Lobby. The users vec has an initial capacity of 12,
    /// since that is the maximum player count of the original board game.
    pub fn new(id: String) -> Self {
        Self {
            id,
            users: Vec::with_capacity(12),
        }
    }

    /// An infinite loop handling all the commands for that lobby.
    pub async fn manage(mut self, mut rx: Receiver<LobbyCommand>) {
        use LobbyCommand::*;

        while let Some(command) = rx.recv().await {
            match command {
                Join { username, user_handle } => {
                    self.users.push(User::new(username.clone(), user_handle.clone()));

                    let update_event = ServerEvent::UsersUpdated {
                        users: self.users.iter()
                            .map(|u| u.username.clone()).collect()
                    };

                    for user in &self.users {
                        user.handle.send(update_event.clone()).await.unwrap();
                    }
                }
            }
        }
    }
}

/// A representation of a single user joined in a lobby
struct User {
    username: String,
    handle: UserHandle,
}

type UserHandle = Sender<ServerEvent>;

impl User {
    /// Creates new user
    fn new(username: String, handle: UserHandle) -> Self {
        Self { username, handle }
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
