use futures_util::{StreamExt, SinkExt, TryFutureExt};
use tokio::sync::mpsc::{self, Sender};
use serde::{Serialize, Deserialize};
use warp::ws::Message;

use crate::lobby::{LobbyHandle, LobbyCommand};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ServerEvent {
    UserJoined {
        username: String
    },
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum ClientEvent {
    Join {
        username: String
    }
}

pub async fn handle_user_connected(lobby: LobbyHandle, socket: warp::ws::WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = socket.split();
    let (mut tx, mut rx) = mpsc::channel::<ServerEvent>(32);

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
            Ok(msg) => msg.to_str().unwrap().to_string(),
            Err(e) => {
                eprintln!("ws err: {}", e);
                break;
            }
        };

        let client_event_result = serde_json::from_str::<ClientEvent>(message.as_str());

        match client_event_result {
            Ok(client_event) => {
                lobby.send(client_event.generate_lobby_command(tx.clone())).await.unwrap();
            },
            Err(e) => {
                eprintln!("Deserialization error: {}", e);
                tx.send(ServerEvent::Error).await.unwrap();
            }
        }
    }
}

impl ClientEvent {
    fn generate_lobby_command(self, my_handle: Sender<ServerEvent>) -> LobbyCommand {
        match self {
            Self::Join { username } => LobbyCommand::Join { username, user_handle: my_handle }
        }
    }
}