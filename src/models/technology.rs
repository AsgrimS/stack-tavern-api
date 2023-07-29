use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::{Error, FromRow};

use crate::db::{get_connection_pool, Delete, Get, GetAll};

use super::TableModel;

#[derive(FromRow, Serialize)]
pub struct Technology {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTechnology {
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
}

impl TableModel for Technology {
    const TABLE_NAME: &'static str = "technologies";
}

impl Technology {
    pub async fn fuzzy_search(name: &str) -> Result<Vec<Self>, Error> {
        let pool = get_connection_pool().await;

        let technologies = sqlx::query_as!(
            Technology,
            r"
            SELECT 
	            *
            FROM technologies
            WHERE name % $1
            ORDER BY SIMILARITY(name,$1) 
            DESC
            LIMIT 5
            ",
            name
        )
        .fetch_all(pool)
        .await?;

        Ok(technologies)
    }
}

impl Get for Technology {}
impl Delete for Technology {}
impl GetAll for Technology {}
