use std::env;

use axum::async_trait;
use sqlx::PgPool;
use sqlx::{postgres::PgRow, Error, FromRow};

use crate::models::TableModel;

pub async fn get_connection_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap()
}

/// This trait is used to implement the CRUD operations for the models.
/// It does not implement create operation, so it must be implemented by the model.
#[async_trait]
pub trait Crud {
    async fn get(item_id: &i32) -> Result<Box<Self>, Error>
    where
        Self: Send + Unpin + TableModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {} WHERE id = {}", Self::TABLE_NAME, item_id);
        let item: Self = sqlx::query_as(query.as_str()).fetch_one(&pool).await?;

        Ok(Box::new(item))
    }
    async fn get_all() -> Result<Vec<Box<Self>>, Error>
    where
        Self: Send + Unpin + TableModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let items = sqlx::query_as(query.as_str()).fetch_all(&pool).await?;

        Ok(items.into_iter().map(|item| Box::new(item)).collect())
    }
    async fn delete(item_id: &i32) -> Result<u64, Error>
    where
        Self: TableModel,
    {
        let pool = get_connection_pool().await;
        let query = format!("DELETE FROM {} WHERE id = {}", Self::TABLE_NAME, item_id);
        let rows = sqlx::query(query.as_str()).execute(&pool).await?;
        Ok(rows.rows_affected())
    }
}
