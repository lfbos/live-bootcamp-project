#![allow(unused_variables)]

use crate::domain::{AuthAPIError, User};
use crate::services::hashmap_user_store::UserStoreError;
use crate::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde::Serialize;

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    if email.is_empty() || !email.contains('@') || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state
        .user_store
        .write()
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    user_store.add_user(&user).map_err(|e| match e {
        UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
        _ => AuthAPIError::UnexpectedError,
    })?;

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
