mod lobby;

use std::convert::Infallible;

use lobby::{Lobbies, Lobby};
use tokio::sync::RwLock;
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
        .and(with_lobbies(lobbies))
        .and_then(handle_create_lobby);

    let routes = ws_echo.or(create_lobby)
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

    lobbies.write().await.insert(id.clone(), RwLock::new(Lobby::new()));

    Ok(id)
}
