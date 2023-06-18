use axum::{
    http::StatusCode,
    routing::{get, post},
    // response::IntoResponse,
    Json,
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    dotenv().ok(); // load env variables from .env

    let format = fmt::format()
        .with_level(true) // include levels in formatted output
        .with_target(false) // don't include targets
        .compact(); // use the `Compact` formatting style.

    // initialize tracing
    tracing_subscriber::fmt().event_format(format).init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
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
