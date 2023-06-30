use chrono::{DateTime, Utc};
use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};
use sqlx::{types::Uuid, Error, FromRow};
// use uuid::Uuid;

use crate::db::{get_connection_pool, Crud};

use super::TableModel;

#[derive(FromRow)]
pub struct Stack {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub user_uuid: Uuid,
}

impl TableModel for Stack {
    const TABLE_NAME: &'static str = "stacks";
}

impl Stack {
    pub async fn create(payload: &CreateStack, user_uuid: &Uuid) -> Result<(), Error> {
        let pool = get_connection_pool().await;
        sqlx::query!(
            "INSERT INTO stacks (name, description, user_uuid) VALUES ($1, $2, $3)",
            payload.name,
            payload.description,
            user_uuid
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_stacks(user_uuid: &Uuid) -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;
        let stacks = sqlx::query_as!(
            Stack,
            "SELECT * FROM stacks WHERE user_uuid = $1",
            user_uuid
        )
        .fetch_all(pool)
        .await?;
        Ok(stacks)
    }
}

impl Crud for Stack {}

impl Serialize for Stack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stack", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("user_uuid", &self.user_uuid.to_string())?;
        state.end()
    }
}

#[derive(Deserialize)]
pub struct CreateStack {
    pub name: String,
    pub description: Option<String>,
}
