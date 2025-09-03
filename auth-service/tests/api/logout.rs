use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    // Create a user
    let email = get_random_email();
    let password = "password123";
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Extract token from the login response cookie
    let auth_cookie = response
        .cookies()
        .find(|c| c.name() == JWT_COOKIE_NAME)
        .expect("Auth cookie not found in login response");
    let token = auth_cookie.value().to_string();

    // Logout
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    // Check that the token was added to the banned token store
    let is_banned = app
        .banned_token_store
        .read()
        .await
        .is_token_banned(&token)
        .await;
    assert!(is_banned);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    // Create a user
    let email = get_random_email();
    let password = "password123";
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Attempt to logout (1st time)
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    // Attempt to logout (2nd time)
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response.json::<ErrorResponse>().await.unwrap().error,
        "Missing token".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response.json::<ErrorResponse>().await.unwrap().error,
        "Invalid token".to_owned()
    );
}
