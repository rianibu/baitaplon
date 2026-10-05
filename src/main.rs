use axum::{
    Router,
    routing::get,
    middleware::from_fn,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum::extract::State;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use crate::routes::api::api_router;

// Define our application state
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

// Define a simple health check handler
async fn health_check() -> &'static str {
    "OK"
}

// Define a 404 handler for SPA fallback
async fn fallback_handler() -> impl IntoResponse {
    // Serve index.html for any other GET request (for client-side routing)
    // But note: we are serving static files via ServeDir, so we don't need this if we want to serve index.html only for root.
    // Instead, we can rely on ServeDir's fallback: if we set `fallback: true`, it will serve index.html for missing files.
    // We'll handle that in the service layer.
    (StatusCode::NOT_FOUND, "Not Found")
}

#[tokio::main]
async fn main() {
    // Initialize the database connection pool
    let db = SqlitePool::connect("sqlite:task_manager.db")
        .await
        .expect("Failed to connect to database");

    // Run migrations: create tables if they don't exist
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
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(&db)
    .await
    .expect("Failed to create tasks table");

    // Build our application state
    let state = AppState { db };

    // Build the API router
    let api_router = api_router().with_state(state.clone());

    // Serve static files from the "static" directory
    // We'll use ServeDir with fallback to index.html for SPA
    let static_service = ServeDir::new("static")
        .fallback(serve_dir::fallback::Fallback::index("index.html"));

    // Combine routers: API under /api, and static files for everything else
    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(static_service)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .fallback(fallback_handler); // This fallback is for any route not matched by the above (should be rare due to static fallback)

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}