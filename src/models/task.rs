use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::SqlitePool;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

impl Task {
    pub async fn create(pool: &SqlitePool, user_id: i64, title: &str, content: &str) -> Result<Self, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO tasks (user_id, title, content) VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(title)
        .bind(content)
        .execute(pool)
        .await?;

        let id = result.last_insert_rowid();

        let task = sqlx::query_as::<_, Task>(
            "SELECT id, user_id, title, content, status, created_at FROM tasks WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn get_by_user_id(pool: &SqlitePool, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT id, user_id, title, content, status, created_at FROM tasks WHERE user_id = ? ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64, user_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM tasks WHERE id = ? AND user_id = ?",
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn update_status(pool: &SqlitePool, id: i64, user_id: i64, status: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE tasks SET status = ? WHERE id = ? AND user_id = ?",
        )
        .bind(status)
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}