use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;

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
