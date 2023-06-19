use axum::{http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;

use sqlx::Row;

pub fn users_router() -> Router {
    Router::new()
        .route("/", get(|| async { "ok" }).post(create_user))
        .route("/test", get(test_db))
}

struct UserDB {
    username: String,
}

async fn test_db() -> (StatusCode, Json<i32>) {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap();
    // sqlx::migrate!().run(&pool).await.unwrap();

    // let res = sqlx::query_as!(UserDB, "SELECT username FROM user")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await
        .unwrap();
    let sum: i32 = res.get("sum");
    (StatusCode::CREATED, Json(sum))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::tests::{get, AppGet};

    #[rstest::rstest]
    #[tokio::test]
    async fn get_user(get: AppGet<'_>) {
        let response = get("/users").await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_text = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(response_text, "ok");
    }
}
