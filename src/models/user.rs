use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Option<Self> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
    }

    pub async fn create(pool: &SqlitePool, username: &str, password_hash: &str) -> Result<Self, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        )
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?;

        let id = result.last_insert_rowid();

        Ok(User {
            id,
            username: username.to_string(),
            password_hash: password_hash.to_string(),
        })
    }
}