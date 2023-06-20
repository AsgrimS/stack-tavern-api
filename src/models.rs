use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::db::get_connection_pool;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn get(user_id: &i32) -> Result<Self, Error> {
        let pool = get_connection_pool().await;

        sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(&pool)
            .await
    }

    pub async fn get_all() -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;

        sqlx::query_as!(Self, "SELECT * FROM users")
            .fetch_all(&pool)
            .await
    }

    pub async fn create(payload: &CreateUser) -> Result<(), Error> {
        let pool = get_connection_pool().await;

        sqlx::query!(
            "INSERT INTO users (username, email) VALUES ($1, $2)",
            payload.username,
            payload.email
        )
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub async fn delete(user_id: &i32) -> Result<u64, Error> {
        let pool = get_connection_pool().await;

        let rows = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await?;
        Ok(rows.rows_affected())
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
