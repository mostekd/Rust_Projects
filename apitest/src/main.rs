use axum::{
    extract::{Json, State},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Router,
};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use axum::body::Body;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use sqlx::Row;

// Struktura przechowująca wiadomości w pamięci
#[derive(Clone)]
struct AppState {
    messages: Arc<Mutex<Vec<Message>>>,
    db: SqlitePool,
}

#[derive(Deserialize, Serialize, Clone)]
struct Message {
    content: String,
}

// Template HTML
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    messages: &'a [Message],
}

async fn hello() -> &'static str {
    "Hello from Axum!"
}

// Dodawanie wiadomości z walidacją
async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<Message>,
) -> impl IntoResponse {
    if payload.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Content cannot be empty").into_response();
    }
    // Zapis do bazy
    if let Err(e) = sqlx::query("INSERT INTO messages (content) VALUES (?)")
        .bind(&payload.content)
        .execute(&state.db)
        .await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    let mut messages = state.messages.lock().await;
    messages.push(payload.clone());
    (StatusCode::OK, payload.content).into_response()
}

// Usuwanie wszystkich wiadomości
async fn delete_message(State(state): State<AppState>) -> impl IntoResponse {
    let mut messages = state.messages.lock().await;
    messages.clear();
    (StatusCode::OK, "All messages deleted")
}

// Wyświetlanie wiadomości jako HTML
async fn list_messages(State(state): State<AppState>) -> impl IntoResponse {
    // Pobierz z bazy (wersja bez makra query_as!)
    let db_messages: Vec<Message> = sqlx::query("SELECT content FROM messages")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| Message {
            content: row.get::<String, _>("content"),
        })
        .collect();
    let template = IndexTemplate { messages: &db_messages };
    Html(template.render().unwrap())
}

// Middleware logowania
async fn log_middleware(req: axum::http::Request<Body>, next: middleware::Next) -> impl IntoResponse {
    println!("{} {}", req.method(), req.uri().path());
    next.run(req).await
}

#[tokio::main]
async fn main() {
    // Inicjalizacja bazy danych
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:messages.db").await.unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY, content TEXT NOT NULL)"
    ).execute(&db).await.unwrap();

    // Inicjalizacja stanu aplikacji
    let state = AppState {
        messages: Arc::new(Mutex::new(Vec::new())),
        db,
    };

    let app = Router::new()
        .route("/", get(hello))
        .route("/ping", get(|| async { "pong" }))
        .route("/message", post(create_message))
        .route("/message", delete(delete_message))
        .route("/messages", get(list_messages))
        .nest_service("/static", ServeDir::new("static"))
        .layer(middleware::from_fn(log_middleware))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
