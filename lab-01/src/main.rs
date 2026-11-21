use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct MessageRes {
    message: String,
}

#[derive(Serialize)]
struct HealthRes {
    status: &'static str,
}

async fn health() -> Json<HealthRes> {
    Json(HealthRes { status: "ok" })
}

async fn hello() -> Json<MessageRes> {
    Json(MessageRes {
        message: String::from("Hello, World!"),
    })
}

async fn hello_name(Path(name): Path<String>) -> Json<MessageRes> {
    Json(MessageRes {
        message: format!("Hello, {name}!"),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/hello/{name}", get(hello_name))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
