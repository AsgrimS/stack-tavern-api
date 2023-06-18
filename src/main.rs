use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;

mod routers;
use crate::routers::users::get_users_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let format = fmt::format() // logging styling
        .with_level(true)
        .with_target(false)
        .compact();

    tracing_subscriber::fmt().event_format(format).init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .nest("/users", get_users_router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
