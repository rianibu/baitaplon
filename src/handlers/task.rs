use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::models::task::Task;
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct CreateTaskPayload {
    pub user_id: i64,
    pub title: String,
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct GetTasksPayload {
    pub user_id: i64,
}

#[derive(serde::Deserialize)]
pub struct DeleteTaskPayload {
    pub user_id: i64,
}

#[derive(serde::Deserialize)]
pub struct UpdateTaskStatusPayload {
    pub user_id: i64,
    pub status: String,
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    match Task::create(&state.db, payload.user_id, &payload.title, &payload.content).await {
        Ok(task) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "task": task
            })),
        ),
        Err(e) => {
            eprintln!("Failed to create task: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "Failed to create task"
                })),
            )
        }
    }
}

pub async fn get_tasks(
    State(state): State<AppState>,
    Query(payload): Query<GetTasksPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    match Task::get_by_user_id(&state.db, payload.user_id).await {
        Ok(tasks) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "tasks": tasks
            })),
        ),
        Err(e) => {
            eprintln!("Failed to fetch tasks: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "Failed to fetch tasks"
                })),
            )
        }
    }
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
    Json(payload): Json<DeleteTaskPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    match Task::delete(&state.db, task_id, payload.user_id).await {
        Ok(affected) if affected > 0 => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Task deleted"
            })),
        ),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "Task not found or not authorized"
            })),
        ),
        Err(e) => {
            eprintln!("Failed to delete task: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "Failed to delete task"
                })),
            )
        }
    }
}

pub async fn update_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
    Json(payload): Json<UpdateTaskStatusPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    match Task::update_status(&state.db, task_id, payload.user_id, &payload.status).await {
        Ok(affected) if affected > 0 => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Task status updated"
            })),
        ),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "Task not found or not authorized"
            })),
        ),
        Err(e) => {
            eprintln!("Failed to update task status: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "Failed to update task status"
                })),
            )
        }
    }
}