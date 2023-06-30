use axum::{
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use sqlx::types::Uuid;

use crate::auth::require_token;
use crate::models::user::{CreateUser, User};

pub fn users_router() -> Router {
    Router::new()
        .route(
            "/info",
            get(get_user)
                .delete(delete_user)
                .route_layer(middleware::from_fn(require_token)),
        )
        .route("/", get(get_users).post(create_user))
}

async fn get_user(Extension(user_uuid): Extension<Uuid>) -> Response {
    let Ok(user) = User::get(&user_uuid).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(user).into_response()
}

async fn create_user(Json(payload): Json<CreateUser>) -> Response {
    let Ok(_) = User::create(&payload)
    .await else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    StatusCode::CREATED.into_response()
}

async fn delete_user(Extension(user_uuid): Extension<Uuid>) -> Response {
    let Ok(affected_rows) = User::delete(&user_uuid)
    .await else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    if affected_rows == 0 {
        return StatusCode::NOT_FOUND.into_response();
    };

    StatusCode::NO_CONTENT.into_response()
}

async fn get_users() -> Response {
    let Ok(users) = User::get_all()
        .await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(users).into_response()
}
