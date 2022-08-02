use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use rand::Rng;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Lobby {
  id: String
}

pub type Lobbies = Arc<RwLock<HashMap<String, LobbyHandle>>>;
pub type LobbyHandle = Sender<LobbyCommand>;

#[derive(Debug)]
pub enum LobbyCommand {
  UserConnected,
  UserMessage(String)
}

impl Lobby {
  pub fn new(id: String) -> Self {
    Self { id }
  }

  pub async fn manage(self, mut rx: Receiver<LobbyCommand>) {
    use LobbyCommand::*;

    while let Some(command) = rx.recv().await {
      match command {
        UserConnected => println!("User has connected!"),
        UserMessage(msg) => println!("User sent message: {}", msg),
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
  use rand::thread_rng;
  use rand::distributions::Alphanumeric;

  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect()
}
