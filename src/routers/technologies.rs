use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

use crate::db::{Get, GetAll};
use crate::models::technology::Technology;

pub fn technologies_router() -> Router {
    Router::new()
        .route("/:technology_id", get(get_technology))
        .route("/", get(get_technologies))
        .route(
            "/fuzzy/:technology_name",
            get(get_fuzzy_search_technologies),
        )
}

async fn get_technology(Path(technology_id): Path<i32>) -> Response {
    let Ok(technology) = Technology::get(&technology_id).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(technology).into_response()
}

async fn get_technologies() -> Response {
    let Ok(technologies) = Technology::get_all()
        .await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(technologies).into_response()
}

async fn get_fuzzy_search_technologies(Path(technology_name): Path<String>) -> Response {
    let Ok(technologies) = Technology::fuzzy_search(&technology_name).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    Json(technologies).into_response()
}
