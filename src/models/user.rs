use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

use crate::db::{get_connection_pool, Crud};

use super::TableModel;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl TableModel for User {
    const TABLE_NAME: &'static str = "users";
}

impl User {
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
}

impl Crud for User {}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}
