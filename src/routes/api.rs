use axum::Router;
use axum::routing::{post, get, delete};
use crate::handlers::auth::{register, login};
use crate::handlers::task::{create_task, get_tasks, delete_task};

pub fn api_router() -> Router<crate::AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_tasks))
        .route("/tasks/:id", delete(delete_task))
}