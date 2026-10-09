use axum::Router;
use axum::routing::{post, get, delete, put};
use crate::handlers::auth::{register, login};
use crate::handlers::task::{create_task, get_tasks, delete_task, update_task_status};

pub fn api_router() -> Router<crate::AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_tasks))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks/:id/status", put(update_task_status))
}