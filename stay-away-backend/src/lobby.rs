use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Lobby {
    id: String,
    users: Vec<String>,
}

pub type Lobbies = Arc<RwLock<HashMap<String, LobbyHandle>>>;
pub type LobbyHandle = Sender<LobbyCommand>;

#[derive(Debug)]
pub enum LobbyCommand {
    Join { username: String },
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
                Join { username } => {
                    self.users.push(username);
                }
            }
        }
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
