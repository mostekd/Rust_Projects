use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello from Axum!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/ping", get(|| async { "pong" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
