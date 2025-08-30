use auth_service::{ErrorResponse, routes::LoginResponse, utils::constants::JWT_COOKIE_NAME};
use crate::helpers::{get_random_email, TestApp};
use uuid::Uuid;

#[tokio::test]
async fn login_should_return_200_for_a_valid_user() {
    let app = TestApp::new().await;

    // Create a user
    let email = format!("{}@example.com", Uuid::new_v4());
    let password = "password123";
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Attempt to login
    let login_body = serde_json::json!({
        "email": email,
        "password": password
    });
    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let expected_response = LoginResponse {
        message: "Login successful!".to_string(),
    };

    assert_eq!(
        response.json::<LoginResponse>().await.unwrap(),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_email_is_invalid() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": "invalid-email",
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid credentials".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_credentials_are_incorrect() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": "random@example.com",
        "password": "password123"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_owned()
    );
}

#[tokio::test]
async fn should_return_422_if_credentials_are_malformed() {
    let app = TestApp::new().await;

    let test_cases = vec![
        serde_json::json!({
            "password": "password123"
        }),
        serde_json::json!({
            "email": "test@example.com"
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(response.status().as_u16(), 422);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    // Create a user
    let email = format!("{}@example.com", Uuid::new_v4());
    let password = "password123";
    let signup_body = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Attempt to login with incorrect credentials
    let login_body = serde_json::json!({
        "email": email,
        "password": "wrongpassword"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_owned()
    );
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
