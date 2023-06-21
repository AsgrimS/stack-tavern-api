use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

use crate::db::{get_connection_pool, Crud};

use super::TableModel;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl TableModel for User {
    const TABLE_NAME: &'static str = "users";
}

impl User {
    pub async fn create(payload: &CreateUser) -> Result<(), Error> {
        let pool = get_connection_pool().await;

        sqlx::query!(
            "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
            payload.name,
            payload.email,
            "password"
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Crud for User {}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
