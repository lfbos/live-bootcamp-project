use axum::{
    routing::{post},
    Router,
};
use std::error::Error;
use tower_http::services::ServeDir;

pub mod routes;

pub struct Application {
    listener: tokio::net::TcpListener,
    pub address: String,
    app: Router,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Define your routes
        let app = Router::new()
            .route("/signup", post(routes::signup_handler))
            .route("/login", post(routes::login_handler))
            .route("/logout", post(routes::logout_handler))
            .route("/verify-2fa", post(routes::verify_2fa_handler))
            .route("/verify-token", post(routes::verify_token_handler))
            .fallback_service(ServeDir::new("assets"));

        // Bind the TCP listener
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        Ok(Self { listener, address, app })
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
