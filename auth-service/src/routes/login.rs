use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.read().await;

    if !user_store.validate_user(&email, &password).await.is_ok() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    user_store.get_user(&email).await.map_err(|_| AuthAPIError::IncorrectCredentials)?;

    let response = Json(LoginResponse {
        message: "Login successful!".to_string(),
    });

    Ok((StatusCode::OK, response.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}
