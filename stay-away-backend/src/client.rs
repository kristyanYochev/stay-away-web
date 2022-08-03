use futures_util::{StreamExt, SinkExt, TryFutureExt};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use warp::ws::Message;

use crate::lobby::LobbyHandle;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum UserEvent {
    UserJoined {
        username: String
    }
}

pub async fn handle_user_connected(lobby: LobbyHandle, socket: warp::ws::WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = socket.split();
    let (mut tx, mut rx) = mpsc::channel::<UserEvent>(32);

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
    }
}
