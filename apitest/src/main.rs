use axum::{
    extract::Json,
    routing::{delete, get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello from Axum!"
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

async fn create_message(Json(payload): Json<Message>) -> String {
    format!("Received: {}", payload.content)
}

async fn delete_message() -> &'static str {
    "Message deleted"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/ping", get(|| async { "pong" }))
        .route("/message", post(create_message))
        .route("/message", delete(delete_message));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
