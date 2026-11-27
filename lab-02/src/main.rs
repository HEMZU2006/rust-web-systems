use axum::{Json, Router, extract::State, http::StatusCode, routing::{get, post}};
use serde::Deserialize;
use tracing::{error,info};
use sqlx::PgPool;


#[derive(Deserialize)]
struct AuthInput {
    username: String,
    password: String,
}

fn is_valid_credentials(username: &str, password: &str) -> bool {
    let user = username.trim();
    user.chars().count() > 2 && password.chars().count() > 7 && user != password && !user.is_empty() && !password.is_empty()
}

async fn register(State(pool):State<PgPool>,Json(input): Json<AuthInput>) -> StatusCode {

    let username = input.username.trim();
    let password = input.password.trim()
    ;
    if !is_valid_credentials(username,password) {
        return StatusCode::BAD_REQUEST;
    }

     let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(username)
        .bind(&input.password)
        .execute(&pool)
        .await;

     match result {
        Ok(_) => {
            info!("User registered: {username}");
            StatusCode::CREATED
        }
        Err(e) if e.as_database_error().is_some_and(|db| db.is_unique_violation()) => {
            StatusCode::CONFLICT
        }
        Err(e) => {
            error!("Failed to insert user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
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

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Connected to PostgreSQL!");

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .with_state(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}  