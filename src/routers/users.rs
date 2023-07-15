use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::db::{Get, GetAll};
use crate::models::user::User;

pub fn users_router() -> Router {
    Router::new()
        .route("/:user_id", get(get_user))
        .route("/", get(get_users))
}

async fn get_user(Path(user_id): Path<i32>) -> Response {
    let Ok(user) = User::get(&user_id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(user).into_response()
}

async fn get_users() -> Response {
    let Ok(users) = User::get_all()
        .await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(users).into_response()
}
