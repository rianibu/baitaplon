use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use crate::models::user::User;
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Validate password contains '@'
    if !payload.password.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Password must contain @ character"
            })),
        );
    }

    // Hash password
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to hash password" }))),
    };

    // Create user in DB
    match User::create(&state.db, &payload.username, &password_hash).await {
        Ok(user) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "user_id": user.id
            })),
        ),
        Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => (
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Username already exists"
            })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create user"
            })),
        ),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Find user by username
    let user = match User::find_by_username(&state.db, &payload.username).await {
        Some(u) => u,
        None => return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "Invalid username or password"
            })),
        ),
    };

    // Verify password
    let password_valid = verify(&payload.password, &user.password_hash)
        .unwrap_or(false);

    if !password_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "error": "Invalid username or password"
            })),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "user_id": user.id
        })),
    )
}