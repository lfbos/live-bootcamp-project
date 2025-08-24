#![allow(unused_variables)]

use crate::domain::User;
use crate::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde::Serialize;

pub async fn signup_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = app_state.user_store.write().unwrap();

    user_store.add_user(&user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
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
