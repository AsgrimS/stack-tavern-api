use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Error, FromRow};

use crate::db::{get_connection_pool, Crud};

use super::TableModel;

#[derive(FromRow, Serialize)]
pub struct Stack {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
}

impl TableModel for Stack {
    const TABLE_NAME: &'static str = "stacks";
}

impl Stack {
    pub async fn create(payload: &CreateStack, user_uuid: &Uuid) -> Result<(), Error> {
        let pool = get_connection_pool().await;

        let user_id: i32 = sqlx::query!("SELECT id FROM users WHERE identity_uuid = $1", user_uuid)
            .fetch_one(pool)
            .await?
            .id;

        sqlx::query!(
            "INSERT INTO stacks (name, description, user_id) VALUES ($1, $2, $3)",
            payload.name,
            payload.description,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_stacks(user_id: &i32) -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;
        let stacks = sqlx::query_as!(Stack, "SELECT * FROM stacks WHERE user_id = $1", user_id)
            .fetch_all(pool)
            .await?;

        Ok(stacks)
    }
}

impl Crud for Stack {}

#[derive(Deserialize)]
pub struct CreateStack {
    pub name: String,
    pub description: Option<String>,
}
