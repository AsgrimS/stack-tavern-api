use chrono::{DateTime, Utc};
use serde::{ser::Serializer, Serialize};

use sqlx::{types::Uuid, Error, FromRow};

use crate::db::{get_connection_pool, Get, GetAll};

use super::TableModel;

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i32,
    #[serde(serialize_with = "uuid_serialize")]
    pub identity_uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UserOut {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

fn uuid_serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

impl TableModel for User {
    const TABLE_NAME: &'static str = "users";
}

impl User {
    pub async fn create(name: &str, identity_uuid: &Uuid) -> Result<(), Error> {
        let pool = get_connection_pool().await;

        sqlx::query!(
            "INSERT INTO users (identity_uuid, name) VALUES ($1, $2)",
            identity_uuid,
            name,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_uuid(identity_uuid: &Uuid) -> Result<Self, Error> {
        let pool = get_connection_pool().await;
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE identity_uuid = $1",
            identity_uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

impl Get for User {}
impl GetAll for User {}
