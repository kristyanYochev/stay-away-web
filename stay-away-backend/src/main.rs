mod lobby;
mod client;
mod events;

use std::convert::Infallible;

use lobby::{Lobbies, Lobby, LobbyHandle};
use tokio::sync::mpsc;
use warp::{self, Filter};
use futures_util::{StreamExt, FutureExt};

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
            ws.on_upgrade(move |socket| client::handle_user_connected(lobby_handle, socket))
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

/// Adds the lobbies into the filter chain to be picked up by other filters
fn with_lobbies(lobbies: Lobbies) -> impl Filter<Extract = (Lobbies,), Error = Infallible> + Clone {
    warp::any().map(move || lobbies.clone())
}

/// Generates an id using lobby::generate_id, creates a new lobby with that id
/// and spawns a task to manage the lobby. After spawining the task, the lobby is
/// added to the lobbies map and the id is returned.
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

/// Takes the lobbies map and resolves with a specific lobby.
/// Returns an Err() with a not_found rejection if the lobby does not exist.
async fn with_lobby_handle(lobby_id: String, lobbies: Lobbies) -> Result<LobbyHandle, warp::Rejection> {
    if let Some(handle) = lobbies.read().await.get(&lobby_id) {
        Ok(handle.clone())
    } else {
        Err(warp::reject::not_found())
    }
}
