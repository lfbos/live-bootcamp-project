use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token, &state.banned_token_store).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}
