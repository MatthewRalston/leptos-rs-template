use axum;
use axum::{routing::get, Router, response::Redirect, extract::Query};
use std::collections::HashMap;
use oauth2::{CsrfToken, AuthorizationCode};
use reqwest::async_http_client;
use crate::auth::oauth_client;

pub fn create_router() -> Router {
    Router::new()
        .route("/auth/login", get(auth_login))
        .route("/auth/callback", get(auth_callback));
}

async fn auth_login() -> Redirect {
    let client = oauth_client();
    let (auth_url, _csrf_token) = client.authorize_url(CsrfToken::new_random).url();
    Redirect::temporary(auth_url.as_str())
}

async fn auth_callback(Query(params): Query<HashMap<String, String>>) -> String {
    let code = params.get("code").expect("Missing code");
    let client = oauth_client();

    let token_response = client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(async_http_client)
        .await
        .expect("Failed to get token");

    format!("Access Token: {:?}", token_response.access_token().secret())
}
