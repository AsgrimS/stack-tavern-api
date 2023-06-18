use axum::Router;
use rstest::fixture;

use crate::app as app_router;

#[fixture]
pub fn app() -> Router {
    app_router()
}
