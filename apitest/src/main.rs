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
use sqlx::FromRow;

// Struktura przechowująca wiadomości w pamięci
#[derive(Clone)]
struct AppState {
    messages: Arc<Mutex<Vec<Message>>>,
    db: SqlitePool,
}

#[derive(Deserialize, Serialize, Clone, FromRow)]
struct Message {
    id: i64,
    content: String,
}

// Template HTML
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    messages: &'a [Message],
    page: usize,
    total_pages: usize,
    sort: String,
    query: String,
}

// Dodawanie wiadomości z formularza HTML
use axum::Form;
#[derive(Deserialize)]
struct MessageForm {
    content: String,
}

async fn add_message_form(State(state): State<AppState>, Form(form): Form<MessageForm>) -> impl IntoResponse {
    if form.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Content cannot be empty").into_response();
    }
    if let Err(e) = sqlx::query("INSERT INTO messages (content) VALUES (?)")
        .bind(&form.content)
        .execute(&state.db)
        .await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    // Poprawka: Redirect na .into_response()
    axum::response::Redirect::to("/messages").into_response()
}

// Edycja wiadomości
#[derive(Deserialize)]
struct EditForm {
    id: i64,
    content: String,
}

async fn edit_message(State(state): State<AppState>, Form(form): Form<EditForm>) -> impl IntoResponse {
    if form.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Content cannot be empty").into_response();
    }
    if let Err(e) = sqlx::query("UPDATE messages SET content = ? WHERE id = ?")
        .bind(&form.content)
        .bind(form.id)
        .execute(&state.db)
        .await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    // Poprawka: Redirect na .into_response()
    axum::response::Redirect::to("/messages").into_response()
}

// Usuwanie pojedynczej wiadomości
async fn delete_message_id(State(state): State<AppState>, axum::extract::Path(id): axum::extract::Path<i64>) -> impl IntoResponse {
    if let Err(e) = sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    // Poprawka: Redirect na .into_response()
    axum::response::Redirect::to("/messages").into_response()
}

// Wyświetlanie wiadomości z paginacją, sortowaniem i wyszukiwaniem
use axum::extract::Query;
use std::collections::HashMap;

async fn list_messages(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let page = params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1);
    let per_page = 5;
    let sort = params.get("sort").cloned().unwrap_or_else(|| "desc".to_string());
    let query = params.get("q").cloned().unwrap_or_default();
    let offset = (page - 1) * per_page;
    let mut sql = "SELECT id, content FROM messages".to_string();
    let mut args: Vec<String> = vec![];
    // Poprawka formatowania LIKE
    if !query.is_empty() {
        sql.push_str(" WHERE content LIKE ?");
        args.push(format!("%{}%", query));
    }
    sql.push_str(&format!(" ORDER BY id {} LIMIT ? OFFSET ?", if sort == "asc" { "ASC" } else { "DESC" }));
    let mut q = sqlx::query_as::<_, Message>(&sql);
    for arg in &args {
        q = q.bind(arg);
    }
    q = q.bind(per_page as i64).bind(offset as i64);
    let db_messages = q.fetch_all(&state.db).await.unwrap_or_default();
    // Liczba stron
    let total: (i64,) = if !query.is_empty() {
        sqlx::query_as("SELECT COUNT(*) FROM messages WHERE content LIKE ?")
            .bind(format!("%{}%", query))
            .fetch_one(&state.db).await.unwrap_or((0,))
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM messages")
            .fetch_one(&state.db).await.unwrap_or((0,))
    };
    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as usize;
    let template = IndexTemplate {
        messages: &db_messages,
        page,
        total_pages,
        sort,
        query,
    };
    Html(template.render().unwrap())
}

// Middleware logowania
async fn log_middleware(req: axum::http::Request<Body>, next: middleware::Next) -> impl IntoResponse {
    println!("{} {}", req.method(), req.uri().path());
    next.run(req).await
}

// Przywróć hello
async fn hello() -> &'static str {
    "Hello from Axum!"
}

// Przywróć create_message i delete_message (POST/DELETE JSON, nie HTML)
async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<Message>,
) -> impl IntoResponse {
    if payload.content.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Content cannot be empty").into_response();
    }
    if let Err(e) = sqlx::query("INSERT INTO messages (content) VALUES (?)")
        .bind(&payload.content)
        .execute(&state.db)
        .await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    (StatusCode::OK, payload.content).into_response()
}

async fn delete_message(State(state): State<AppState>) -> impl IntoResponse {
    if let Err(e) = sqlx::query("DELETE FROM messages").execute(&state.db).await {
        eprintln!("DB error: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }
    (StatusCode::OK, "All messages deleted").into_response()
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
        .route("/messages/add", post(add_message_form))
        .route("/messages/edit", post(edit_message))
        .route("/messages/delete/{id}", post(delete_message_id))
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
