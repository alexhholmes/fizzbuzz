use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use tracing::instrument;

use crate::resources::Resources;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200),
        (status = 401),
        (status = 500)
    )
)]
#[instrument(skip(res, body))]
pub async fn post_login(
    State(res): State<Resources>,
    Json(body): Json<LoginRequest>,
) -> StatusCode {
    let user = match res.db.find_user_by_email(&body.email).await {
        Ok(Some(u)) => u,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let password = body.password.clone();
    let stored_hash = user.password_hash.clone();
    let valid = tokio::task::spawn_blocking(move || {
        use argon2::password_hash::{PasswordHash, PasswordVerifier};
        let parsed = PasswordHash::new(&stored_hash).map_err(|_| ())?;
        argon2::Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| ())
    })
    .await
    .unwrap_or(Err(()));

    match valid {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
