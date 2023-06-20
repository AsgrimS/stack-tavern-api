use std::env;

use sqlx::PgPool;

pub async fn get_connection_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap()
}
