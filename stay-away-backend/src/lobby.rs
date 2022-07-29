use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use rand::Rng;

pub struct Lobby;

pub type Lobbies = Arc<RwLock<HashMap<String, RwLock<Lobby>>>>;

impl Lobby {
  pub fn new() -> Self {
    Self
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
