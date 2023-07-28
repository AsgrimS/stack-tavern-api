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

    // pub async fn get(user_uuid: &Uuid) -> Result<Self, Error> {
    //     let pool = get_connection_pool().await;
    //     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE uuid = $1", user_uuid)
    //         .fetch_one(pool)
    //         .await?;
    //
    //     Ok(user)
    // }

    // pub async fn get_all() -> Result<Vec<Self>, Error> {
    //     let pool = get_connection_pool().await;
    //     let users = sqlx::query_as!(User, "SELECT * FROM users")
    //         .fetch_all(pool)
    //         .await?;
    //
    //     Ok(users)
    // }

    // pub async fn delete(user_uuid: &Uuid) -> Result<u64, Error> {
    //     let pool = get_connection_pool().await;
    //     let deleted_rows = sqlx::query!("DELETE FROM users WHERE uuid = $1", user_uuid)
    //         .execute(pool)
    //         .await?
    //         .rows_affected();
    //
    //     Ok(deleted_rows)
    // }
}

impl Get for User {}
impl GetAll for User {}

// #[derive(Deserialize)]
// pub struct CreateUser {
//     pub name: String,
//     #[serde(deserialize_with = "uuid_deserialize")]
//     pub identity_uuid: Uuid,
// }
//
// fn uuid_deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     Uuid::from_str(&s).map_err(serde::de::Error::custom)
// }
