use serde::{Serialize, Deserialize};

/// A representation of events originating from the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", content = "data")]
pub enum ServerEvent {
    /// Fired whenever a user connects. TODO: implement disconnections
    UsersUpdated {
        users: Vec<String>
    },
    /// Fired when the server encounters an error
    Error,
}