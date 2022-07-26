use warp::{self, Filter};
use futures_util::{StreamExt, FutureExt};

#[tokio::main]
async fn main() {
    // let hello_world = warp::any().map(|| "Hello World");

    let routes = warp::path("echo")
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

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
