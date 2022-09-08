use futures_util::{StreamExt, SinkExt, TryFutureExt};
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot;
use warp::ws::Message;

use crate::lobby::{LobbyHandle, Lobby};
use crate::events::{server::ServerEvent, client::ClientEvent};

/// Handles the websocket connection. Spawns a task for sending events to the client.
pub async fn handle_user_connected(lobby: LobbyHandle, socket: warp::ws::WebSocket) {
    use crate::lobby::LobbyCommand::AssignId;

    let (mut websocket_tx, mut websocket_rx) = socket.split();
    let (tx, mut rx) = mpsc::channel(32);

    let (id_tx, id_rx) = oneshot::channel();
    lobby.send(AssignId { id_channel: id_tx }).await.unwrap();

    let id = id_rx.await.unwrap();

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

        client_message(message, tx.clone(), id, lobby.clone()).await;
    }

    user_disconnect(id, lobby).await;
}

/// Handler for a message from the client.
/// Each message is deserialized and, if all goes well, is sent to the lobby.
async fn client_message(msg: Message, my_handle: Sender<ServerEvent>, my_id: usize, lobby: LobbyHandle) {
    let message = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let client_event_result = serde_json::from_str::<ClientEvent>(message);

    match client_event_result {
        Ok(client_event) => {
            lobby.send(client_event.generate_lobby_command(my_handle.clone(), my_id)).await.unwrap();
        },
        Err(e) => {
            eprintln!("Deserialization error: {}", e);
            my_handle.send(ServerEvent::Error).await.unwrap();
        }
    }
}

async fn user_disconnect(my_id: usize, lobby: LobbyHandle) {
    use crate::lobby::LobbyCommand::Disconnect;

    lobby.send(Disconnect { user_id: my_id }).await.unwrap();

    println!("Farewell, {my_id}");
}