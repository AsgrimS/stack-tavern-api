use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

use sqlx::{types::Uuid, Error, FromRow};

use crate::db::get_connection_pool;

use super::TableModel;

#[derive(FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stack", 4)?;
        state.serialize_field("uuid", &self.uuid.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.end()
    }
}

impl TableModel for User {
    const TABLE_NAME: &'static str = "users";
}

impl User {
    pub async fn create(payload: &CreateUser) -> Result<(), Error> {
        let pool = get_connection_pool().await;

        sqlx::query!(
            "INSERT INTO users (uuid, name, email, password) VALUES ($1, $2, $3, $4)",
            Uuid::from_str("a28f46ba-1ac4-4df9-bc28-f62bdaadf45d").unwrap(),
            payload.name,
            payload.email,
            "password"
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(user_uuid: &Uuid) -> Result<Self, Error> {
        let pool = get_connection_pool().await;
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE uuid = $1", user_uuid)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_all() -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(pool)
            .await?;

        Ok(users)
    }

    pub async fn delete(user_uuid: &Uuid) -> Result<u64, Error> {
        let pool = get_connection_pool().await;
        let deleted_rows = sqlx::query!("DELETE FROM users WHERE uuid = $1", user_uuid)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(deleted_rows)
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
