use warp::{self, Filter};

#[tokio::main]
async fn main() {
    let hello_world = warp::any().map(|| "Hello World");

    warp::serve(hello_world).run(([0, 0, 0, 0], 8080)).await;
}
