use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response, Extension,
};
use shuttle_runtime::SecretStore;

pub async fn api_key_auth(Extension(secret): Extension<SecretStore>, req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    let x_api_key = secret.get("x-api-key").unwrap();

    match auth_header {
        Some(key) if key == x_api_key => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
