use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token, &state.banned_token_store).await {
        Ok(_) => (),
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    }

    // Add token to the banned token store
    let mut banned_token_store = state.banned_token_store.write().await;
    banned_token_store.add_token(token).await;

    // Remove the JWT cookie from the `CookieJar`
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}
