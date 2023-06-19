use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::env;

use crate::schemas::{CreateUser, User};

pub fn users_router() -> Router {
    Router::new()
        .route("/:user_id", get(get_user))
        .route("/", get(get_users).post(create_user))
}

async fn get_user(Path(user_id): Path<i32>) -> Response {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap();

    let Ok(user) = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(user).into_response()
}

async fn get_users() -> Response {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap();
    let Ok(users) = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await else {
        return StatusCode::NOT_FOUND.into_response();
    };
    Json(users).into_response()
}

async fn create_user(Json(payload): Json<CreateUser>) -> Response {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    let pool = sqlx::postgres::PgPool::connect(url.as_str()).await.unwrap();

    let Ok(_) = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2)",
        payload.username,
        payload.email
    )
    .execute(&pool)
    .await else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    Json("User Created").into_response()
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };
//
//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

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
