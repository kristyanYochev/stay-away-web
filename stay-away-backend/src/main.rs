mod lobby;

use std::convert::Infallible;

use lobby::{Lobbies, Lobby, LobbyHandle};
use tokio::sync::mpsc;
use warp::{self, Filter, ws::Message};
use futures_util::{StreamExt, FutureExt, SinkExt, TryFutureExt};

#[tokio::main]
async fn main() {
    let lobbies = Lobbies::default();

    let ws_echo = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket wrror: {:?}", e);
                    }
                })
            })
        });

    let create_lobby = warp::path("lobbies")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_lobbies(lobbies.clone()))
        .and_then(handle_create_lobby);
    
    let user_connect = warp::path!("lobbies" / String)
        .and(with_lobbies(lobbies.clone()))
        .and_then(with_lobby_handle)
        .and(warp::ws())
        .map(|lobby_handle, ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| handle_user_connected(lobby_handle, socket))
        });

    let routes = ws_echo
        .or(create_lobby)
        .or(user_connect)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["*"])
            .allow_methods(vec!["GET", "POST"])
        );

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn with_lobbies(lobbies: Lobbies) -> impl Filter<Extract = (Lobbies,), Error = Infallible> + Clone {
    warp::any().map(move || lobbies.clone())
}

async fn handle_create_lobby(lobbies: Lobbies) -> Result<impl warp::Reply, Infallible> {
    let id = lobby::generate_id(&lobbies).await;

    let lobby = Lobby::new(id.clone());

    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        lobby.manage(rx).await;
    });

    lobbies.write().await.insert(id.clone(), tx.clone());

    Ok(id)
}

async fn with_lobby_handle(lobby_id: String, lobbies: Lobbies) -> Result<LobbyHandle, warp::Rejection> {
    if let Some(handle) = lobbies.read().await.get(&lobby_id) {
        Ok(handle.clone())
    } else {
        Err(warp::reject::not_found())
    }
}

async fn handle_user_connected(lobby: LobbyHandle, socket: warp::ws::WebSocket) {
    lobby.send(lobby::LobbyCommand::UserConnected).await
        .unwrap_or_else(|e| { eprintln!("mpsc err: {}", e)});
    let (mut tx, mut rx) = socket.split();

    tx.send(Message::text("Been nice having ya!"))
        .unwrap_or_else(|e| { eprintln!("ws error: {}", e) }).await;

    while let Some(res) = rx.next().await {
        let message = match res {
            Ok(msg) => msg.to_str().unwrap().to_string(),
            Err(e) => {
                eprintln!("ws err: {}", e);
                break;
            }
        };

        lobby.send(lobby::LobbyCommand::UserMessage(message))
            .unwrap_or_else(|e| { eprintln!("mpsc big oof'd {}", e) }).await;
    }
}