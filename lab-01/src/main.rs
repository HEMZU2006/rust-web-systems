use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
struct MessageRes {
    message: String,
}

#[derive(Serialize)]
struct HealthRes {
    status: &'static str,
}

async fn health() -> Json<HealthRes> {
    info!("health request Handling");
    Json(HealthRes { status: "ok" })
}

async fn hello() -> Json<MessageRes> {
    info!("hello request Handling");
    Json(MessageRes {
        message: String::from("Hello, World!"),
    })
}

async fn hello_name(Path(name): Path<String>) -> Json<MessageRes> {
    info!(name = %name, "Hello request Handling");
    Json(MessageRes {
        message: format!("Hello, {name}!"),
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/hello/{name}", get(hello_name))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server is here to Serve");
    axum::serve(listener, app).await.unwrap();
}
