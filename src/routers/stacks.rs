use axum::{
    extract::Path,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::models::stack::{CreateStack, Stack};
use crate::{auth::require_token, db::Crud};

pub fn stacks_router() -> Router {
    Router::new()
        .route("/:stack_id", get(get_stack).delete(delete_stack))
        .route(
            "/",
            post(create_stack)
                .route_layer(middleware::from_fn(require_token))
                .get(get_stacks),
        )
        .route("/user/:user_id", get(get_user_stacks))
}

async fn get_stack(Path(stack_id): Path<i32>) -> Response {
    let Ok(stack) = Stack::get(&stack_id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(stack).into_response()
}

async fn create_stack(Json(payload): Json<CreateStack>) -> Response {
    let Ok(_) = Stack::create(&payload, &1).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    StatusCode::CREATED.into_response()
}

async fn delete_stack(Path(stack_id): Path<i32>) -> Response {
    let Ok(affected_rows) = Stack::delete(&stack_id).await else {
        return StatusCode::UNPROCESSABLE_ENTITY.into_response();
    };

    if affected_rows == 0 {
        return StatusCode::NOT_FOUND.into_response();
    };

    StatusCode::NO_CONTENT.into_response()
}

async fn get_stacks() -> Response {
    let Ok(stacks) = Stack::get_all().await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(stacks).into_response()
}

async fn get_user_stacks(Path(user_id): Path<i32>) -> Response {
    let Ok(stacks) = Stack::get_user_stacks(user_id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(stacks).into_response()
}
