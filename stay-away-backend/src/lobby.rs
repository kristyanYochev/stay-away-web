use std::{collections::HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::client::ServerEvent;

pub struct Lobby {
    id: String,
    users: Vec<User>,
}

pub type Lobbies = Arc<RwLock<HashMap<String, LobbyHandle>>>;
pub type LobbyHandle = Sender<LobbyCommand>;

#[derive(Debug)]
pub enum LobbyCommand {
    Join {
        username: String,
        user_handle: Sender<ServerEvent>
    },
}

impl Lobby {
    pub fn new(id: String) -> Self {
        Self {
            id,
            users: Vec::with_capacity(12),
        }
    }

    pub async fn manage(mut self, mut rx: Receiver<LobbyCommand>) {
        use LobbyCommand::*;

        while let Some(command) = rx.recv().await {
            match command {
                Join { username, user_handle } => {
                    self.users.push(User::new(username.clone(), user_handle));

                    for user in &self.users {
                        user.handle.send(ServerEvent::UserJoined { username: username.clone() }).await.unwrap();
                    }
                }
            }
        }
    }
}

struct User {
    username: String,
    handle: UserHandle,
}

type UserHandle = Sender<ServerEvent>;

impl User {
    fn new(username: String, handle: UserHandle) -> Self {
        Self { username, handle }
    }
}

pub async fn generate_id(lobbies: &Lobbies) -> String {
    let mut id = random_id();

    while lobbies.read().await.contains_key(&id) {
        id = random_id();
    }

    id
}

fn random_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::thread_rng;

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
