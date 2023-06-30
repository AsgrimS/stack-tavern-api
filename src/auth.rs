use std::env;

use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AccessToken, ClientId, ClientSecret, IntrospectionUrl, IssuerUrl, TokenIntrospectionResponse,
};
use tokio::sync::OnceCell;

async fn initalize_openid_client() -> CoreClient {
    let client_id =
        ClientId::new(env::var("CLIENT_ID").expect("Missing the CLIENT_ID environment variable."));

    let client_secret = ClientSecret::new(
        env::var("CLIENT_SECRET").expect("Missing the CLIENT_SECRET environment variable."),
    );

    let issuer = env::var("ISSUER").expect("Missing the ISSUER environment variable.");
    let issuer_url = IssuerUrl::new(issuer.clone())
        .expect("Could not parse the ISSUER environment variable as a IssuerURL.");

    let introspection_url = IntrospectionUrl::new(format!(
        "{}/protocol/openid-connect/token/introspect",
        issuer.clone()
    ))
    .expect("Could not parse url as IntrospectionUrl.");

    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client)
        .await
        .unwrap();

    CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
        .set_introspection_uri(introspection_url)
}

static AUTH_CLIENT: OnceCell<CoreClient> = OnceCell::const_new();

pub async fn get_openid_client<'a>() -> &'a CoreClient {
    AUTH_CLIENT.get_or_init(initalize_openid_client).await
}

pub async fn require_token<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    if verify_token(auth.token()).await {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn verify_token(token: &str) -> bool {
    let token = AccessToken::new(token.to_string());

    let client = get_openid_client().await;

    let introspec_request = client.introspect(&token).unwrap();
    let introspect = introspec_request
        .request_async(async_http_client)
        .await
        .unwrap();

    introspect.active()
}
