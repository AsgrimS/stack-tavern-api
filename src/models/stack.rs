use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, Error, FromRow};

use crate::db::{get_connection_pool, Delete, Get};

use super::technology::{CreateTechnology, Technology};
use super::user::UserOut;
use super::TableModel;

#[derive(FromRow, Serialize)]
pub struct Stack {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct StackOut {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub author: UserOut,
    pub technologies: Vec<Technology>,
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

        let mut txn = pool.begin().await?;

        let new_stack_id = sqlx::query!(
            "INSERT INTO stacks (name, description, user_id) VALUES ($1, $2, $3) RETURNING id",
            payload.name,
            payload.description,
            user_id
        )
        .fetch_one(&mut txn)
        .await?
        .id;

        let mut technology_ids: Vec<i32> = Vec::new();
        for technology in payload.technologies.iter() {
            if let Some(existing_technology) = sqlx::query_as!(
                Technology,
                "SELECT * from technologies WHERE name = $1",
                technology.name.to_lowercase()
            )
            .fetch_optional(&mut txn)
            .await?
            {
                technology_ids.push(existing_technology.id);
            } else {
                technology_ids.push(sqlx::query!(
                "INSERT INTO technologies (name, description, purpose) VALUES ($1, $2, $3) RETURNING id",
                technology.name.to_lowercase(),
                technology.description,
                technology.purpose
            )
            .fetch_one(&mut txn)
            .await?.id);
            };
        }

        for technology_id in technology_ids.iter() {
            sqlx::query!(
                "INSERT INTO stack_technology (stack_id, technology_id) VALUES ($1, $2)",
                new_stack_id,
                technology_id
            )
            .execute(&mut txn)
            .await?;
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn get_user_stacks(user_id: &i32) -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;
        let stacks = sqlx::query_as!(Stack, "SELECT * FROM stacks WHERE user_id = $1", user_id)
            .fetch_all(pool)
            .await?;

        Ok(stacks)
    }

    pub async fn get_all() -> Result<Vec<StackOut>, Error> {
        let mut txn = get_connection_pool().await.begin().await?;

        let rows = sqlx::query!(
            r"
             SELECT 
                 s.id AS stack_id, 
                 s.name AS stack_name, 
                 s.description as stack_description, 
                 s.created_at as stack_created_at, 
                 u.id AS user_id, 
                 u.name AS user_name, 
                 u.created_at as user_created_at 
             FROM 
                 stacks AS s 
             INNER JOIN 
                 users AS u ON s.user_id = u.id
            "
        )
        .fetch_all(&mut txn)
        .await?;

        let mut stacks = Vec::new();
        for row in rows.into_iter() {
            stacks.push(StackOut {
                id: row.stack_id,
                name: row.stack_name,
                description: row.stack_description,
                created_at: row.stack_created_at,
                author: UserOut {
                    id: row.user_id,
                    name: row.user_name,
                    created_at: row.user_created_at,
                },
                technologies: sqlx::query_as!(
                    Technology,
                    r"
                    SELECT
                        t.*
                    FROM 
                        technologies AS t
                    INNER JOIN
                        stack_technology AS st ON t.id = st.technology_id
                    WHERE
                        st.stack_id = $1
                ",
                    row.stack_id
                )
                .fetch_all(&mut txn)
                .await?,
            })
        }

        txn.commit().await?;

        Ok(stacks)
    }
}

impl Get for Stack {}
impl Delete for Stack {}

#[derive(Deserialize)]
pub struct CreateStack {
    pub name: String,
    pub description: Option<String>,
    pub technologies: Vec<CreateTechnology>,
}
