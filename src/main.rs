use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::services::{ServeDir, ServeFile};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;

pub mod models;
pub mod handlers;
pub mod routes;

use crate::routes::api::api_router;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite:task_manager.db")
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );
        "#,
    )
    .execute(&db)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            status TEXT DEFAULT 'pending',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(&db)
    .await
    .expect("Failed to create tasks table");

    let state = AppState { db };

    let api_router = api_router().with_state(state.clone());

    let static_service = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(static_service)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}