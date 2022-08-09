use futures_util::{StreamExt, SinkExt, TryFutureExt};
use tokio::sync::mpsc::{self, Sender};
use serde::{Serialize, Deserialize};
use warp::ws::Message;

use crate::lobby::{LobbyHandle, LobbyCommand};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", content = "data")]
pub enum ServerEvent {
    UsersUpdated {
        users: Vec<String>
    },
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ClientEvent {
    Join {
        username: String
    },
}

pub async fn handle_user_connected(lobby: LobbyHandle, socket: warp::ws::WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = socket.split();
    let (tx, mut rx) = mpsc::channel::<ServerEvent>(32);

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            websocket_tx
                .send(Message::text(serde_json::to_string(&message).unwrap()))
                .unwrap_or_else(|err| {
                    eprintln!("Websocket send error: {}", err);
                })
                .await;
        }
    });

    while let Some(res) = websocket_rx.next().await {
        let message = match res {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("ws err: {}", e);
                break;
            }
        };

        client_message(message, tx.clone(), lobby.clone()).await;
    }
}

async fn client_message(msg: Message, my_handle: Sender<ServerEvent>, lobby: LobbyHandle) {
    let message = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let client_event_result = serde_json::from_str::<ClientEvent>(message);

    match client_event_result {
        Ok(client_event) => {
            lobby.send(client_event.generate_lobby_command(my_handle.clone())).await.unwrap();
        },
        Err(e) => {
            eprintln!("Deserialization error: {}", e);
            my_handle.send(ServerEvent::Error).await.unwrap();
        }
    }
}

impl ClientEvent {
    fn generate_lobby_command(self, my_handle: Sender<ServerEvent>) -> LobbyCommand {
        match self {
            Self::Join { username } => LobbyCommand::Join { username, user_handle: my_handle },
            _ => LobbyCommand::Join { username: "".to_string(), user_handle: my_handle } // Dead code to let the thing compile
        }
    }
}