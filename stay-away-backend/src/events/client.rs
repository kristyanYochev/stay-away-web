use serde::{Serialize, Deserialize};
use tokio::sync::mpsc::Sender;

use crate::lobby::LobbyCommand;

use super::server::ServerEvent;

/// Represents events originating from the client
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ClientEvent {
    /// A request for joining the lobby
    Join {
        username: String
    },
}

impl ClientEvent {
  /// Basically a fancy Into<LobbyCommand>. Not the actual trait, since the handle is needed to generate the LobbyCommand
  pub fn generate_lobby_command(self, my_handle: Sender<ServerEvent>) -> LobbyCommand {
      match self {
          Self::Join { username } => LobbyCommand::Join { username, user_handle: my_handle },
          _ => LobbyCommand::Join { username: "".to_string(), user_handle: my_handle } // Dead code to let the thing compile
      }
  }
}