use axum::{http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

pub fn users_router() -> Router {
    Router::new().route("/", get(|| async { "ok" }).post(create_user))
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
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use tower::ServiceExt;

    use crate::tests::app;

    #[rstest::rstest]
    #[tokio::test]
    async fn get_user(app: Router) {
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_text = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(response_text, "ok");
    }
}
