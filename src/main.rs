use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;

mod routers;
use crate::routers::users::users_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let format = fmt::format() // logging styling
        .with_level(true)
        .with_target(false)
        .compact();

    tracing_subscriber::fmt().event_format(format).init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/users", users_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

#[cfg(test)]
pub mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use rstest::*;
    use tower::ServiceExt;

    use super::app as app_router;

    #[fixture]
    pub fn app() -> Router {
        app_router()
    }

    #[rstest]
    #[tokio::test]
    async fn hello_world(app: Router) {
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_text = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(response_text, "Hello, world!");
    }
}
