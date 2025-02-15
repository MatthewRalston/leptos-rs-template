use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::env;


/// TODO: FIXME
/// Creates and configures the OAuth2 client
pub fn oauth_client() -> BasicClient {
    let client_id = ClientId::new(env::var("OAUTH_CLIENT_ID").expect("Missing OAUTH_CLIENT_ID"));
    let client_secret = ClientSecret::new(env::var("OAUTH_CLIENT_SECRET").expect("Missing OAUTH_CLIENT_SECRET"));
    
    let auth_url = AuthUrl::new("https://provider.com/oauth/authorize".to_string()).unwrap();
    let token_url = TokenUrl::new("https://provider.com/oauth/token".to_string()).unwrap();
    
    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/auth/callback".to_string()).unwrap())
}
