use serde::{Serialize, Deserialize};

/// A representation of events originating from the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", content = "data")]
pub enum ServerEvent {
    /// Fired whenever a user connects.
    UsersUpdated {
        users: Vec<User>,
    },
    Welcome {
        users: Vec<User>,
        id: usize,
    },
    /// Fired when the server encounters an error
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub id: usize
}