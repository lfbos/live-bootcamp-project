use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json,
    Router
};
use std::error::Error;
use std::sync::{Arc, RwLock};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use domain::AuthAPIError;

pub mod domain;
pub mod routes;
pub mod services;

use crate::services::hashmap_user_store::HashmapUserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };

        let body = Json(ErrorResponse { 
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub struct Application {
    listener: tokio::net::TcpListener,
    pub address: String,
    app: Router,
}

impl Application {
    pub async fn build(
        app_state: AppState,
        address: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Define your routes
        let app = Router::new()
            .route("/signup", post(routes::signup_handler))
            .route("/login", post(routes::login_handler))
            .route("/logout", post(routes::logout_handler))
            .route("/verify-2fa", post(routes::verify_2fa_handler))
            .route("/verify-token", post(routes::verify_token_handler))
            .fallback_service(ServeDir::new("assets"))
            .with_state(app_state);

        // Bind the TCP listener
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        Ok(Self {
            listener,
            address,
            app,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);

        // Start the server here, no need to store the Serve type
        axum::serve(self.listener, self.app)
            .with_graceful_shutdown(shutdown_signal())
            .await
    }
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
