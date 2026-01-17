use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::{fs, net::TcpListener, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GoogleAuthError {
    #[error("OAuth error: {0}")]
    OAuth(String),
    #[error("IO error: {0}")]
    Io(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
}

fn token_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    PathBuf::from(format!("{}/.mj_bench/google_token.json", home))
}

pub async fn get_token(
    client_id: &str,
    client_secret: &str,
) -> Result<StoredToken, GoogleAuthError> {
    // If token exists and valid → return it
    if let Ok(tok) = load_token() {
        if tok.expires_at > now() + 60 {
            return Ok(tok);
        }
        // else refresh
        return refresh_token(tok, client_id, client_secret).await;
    }

    // Otherwise perform full OAuth flow
    oauth_flow(client_id, client_secret).await
}

fn load_token() -> Result<StoredToken, GoogleAuthError> {
    let path = token_path();
    let data = fs::read_to_string(path).map_err(|e| GoogleAuthError::Io(e.to_string()))?;
    serde_json::from_str(&data).map_err(|e| GoogleAuthError::Io(e.to_string()))
}

fn save_token(tok: &StoredToken) -> Result<(), GoogleAuthError> {
    let path = token_path();
    fs::create_dir_all(path.parent().unwrap()).ok();
    fs::write(path, serde_json::to_string_pretty(tok).unwrap())
        .map_err(|e| GoogleAuthError::Io(e.to_string()))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

async fn oauth_flow(
    client_id: &str,
    client_secret: &str,
) -> Result<StoredToken, GoogleAuthError> {
    let redirect = "http://localhost:8085/callback";

    let client = BasicClient::new(
        ClientId::new(client_id.into()),
        Some(ClientSecret::new(client_secret.into())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".into()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect.into()).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let auth_url = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/drive.file".into()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/spreadsheets".into()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open this URL in your browser:\n{}", auth_url);

    // Listen for redirect
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();
    let (mut stream, _) = listener.accept().unwrap();

    let mut buf = [0; 2048];
    let n = stream.read(&mut buf).unwrap();
    let req = String::from_utf8_lossy(&buf[..n]);

    let code = req
        .split("code=")
        .nth(1)
        .unwrap()
        .split('&')
        .next()
        .unwrap()
        .to_string();

    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| GoogleAuthError::OAuth(e.to_string()))?;

    let expires_at = now() + token.expires_in().unwrap().as_secs();

    let stored = StoredToken {
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().unwrap().secret().to_string(),
        expires_at,
    };

    save_token(&stored)?;
    Ok(stored)
}

async fn refresh_token(
    tok: StoredToken,
    client_id: &str,
    client_secret: &str,
) -> Result<StoredToken, GoogleAuthError> {
    let client = BasicClient::new(
        ClientId::new(client_id.into()),
        Some(ClientSecret::new(client_secret.into())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".into()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".into()).unwrap()),
    );

    let token = client
        .exchange_refresh_token(&oauth2::RefreshToken::new(tok.refresh_token.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|e| GoogleAuthError::OAuth(e.to_string()))?;

    let expires_at = now() + token.expires_in().unwrap().as_secs();

    let new_tok = StoredToken {
        access_token: token.access_token().secret().to_string(),
        refresh_token: tok.refresh_token,
        expires_at,
    };

    save_token(&new_tok)?;
    Ok(new_tok)
}