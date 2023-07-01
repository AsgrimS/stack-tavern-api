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
use sqlx::types::Uuid;
use tokio::sync::OnceCell;

use crate::models::user::User;

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

async fn get_openid_client<'a>() -> &'a CoreClient {
    AUTH_CLIENT.get_or_init(initalize_openid_client).await
}

async fn verify_token(token: &str) -> Option<Uuid> {
    let token = AccessToken::new(token.to_string());

    let client = get_openid_client().await;

    let introspec_request = client.introspect(&token).unwrap();
    let introspect = introspec_request
        .request_async(async_http_client)
        .await
        .unwrap();

    if !introspect.active() {
        return None;
    };

    let (Some(sub), Some(name)) = (introspect.sub(), introspect.username()) else {
        return None;
    };

    let Ok(user_uuid) = Uuid::try_parse(sub) else {
        return None;
    };

    if (User::get_by_uuid(&user_uuid).await).is_err() {
        User::create(name, &user_uuid).await.ok();
    };

    Some(user_uuid)
}

pub async fn require_token<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let Some(user_uuid) = verify_token(auth.token()).await else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    request.extensions_mut().insert(user_uuid);

    Ok(next.run(request).await)
}
