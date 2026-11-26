use axum::{Json, Router, http::StatusCode, routing::{get, post}};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct AuthInput {
    username: String,
    password: String,
}

fn is_valid_credentials(username: &str, password: &str) -> bool {
    let user = username.trim();
    user.chars().count() > 2 && password.chars().count() > 7 && user != password && !user.is_empty() && !password.is_empty()
}

async fn register(Json(input): Json<AuthInput>) -> StatusCode {
    if !is_valid_credentials(&input.username, &input.password) {
        return StatusCode::BAD_REQUEST;
    }

    info!("User registered: {}", input.username.trim());
    StatusCode::CREATED
}

async fn login() -> StatusCode {
    info!("User logged in");
    StatusCode::OK
}

async fn logout() -> StatusCode {
    info!("User logged out");
    StatusCode::OK
}

async fn me() -> StatusCode {
    info!("Current user check");
    StatusCode::UNAUTHORIZED
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}