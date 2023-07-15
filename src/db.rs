use std::env;

use axum::async_trait;
use sqlx::PgPool;
use sqlx::{postgres::PgRow, Error, FromRow};
use tokio::sync::OnceCell;

use crate::models::TableModel;

async fn initialize_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap()
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_connection_pool<'a>() -> &'a PgPool {
    POOL.get_or_init(initialize_pool).await
}

#[async_trait]
pub trait Get {
    /// Gets an item from the database by id.
    /// Returns a Result with a Boxed item or sqlx::Error.
    async fn get(item_id: &i32) -> Result<Box<Self>, Error>
    where
        Self: Send + Unpin + TableModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {} WHERE id = {}", Self::TABLE_NAME, item_id);
        let item: Self = sqlx::query_as(query.as_str()).fetch_one(pool).await?;

        Ok(Box::new(item))
    }
}

#[async_trait]
pub trait GetAll {
    /// Gets all items from the database.
    /// Returns a Result with a vector of Boxed items or sqlx::Error.
    async fn get_all() -> Result<Vec<Box<Self>>, Error>
    where
        Self: Send + Unpin + TableModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let items = sqlx::query_as(query.as_str()).fetch_all(pool).await?;

        Ok(items.into_iter().map(|item| Box::new(item)).collect())
    }
}

#[async_trait]
pub trait Delete {
    /// Deletes an item from the database by id.
    /// Returns a Result with the number of affected rows or sqlx::Error.
    async fn delete(item_id: &i32) -> Result<u64, Error>
    where
        Self: TableModel,
    {
        let pool = get_connection_pool().await;
        let query = format!("DELETE FROM {} WHERE id = {}", Self::TABLE_NAME, item_id);
        let rows = sqlx::query(query.as_str()).execute(pool).await?;
        Ok(rows.rows_affected())
    }
}
