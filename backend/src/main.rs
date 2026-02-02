use axum::{routing::get, Router, Json, response::IntoResponse};
use tokio::net::TcpListener;
use tracing_subscriber::fmt;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

async fn root() -> Json<ApiResponse<()>> {
    Json(ApiResponse {
        success: true,
        data: None,
        message: Some("SkillHub API v0.1.0".to_string()),
    })
}

async fn not_found() -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, Json(ApiResponse::<()> {
        success: false,
        data: None,
        message: Some("Not found".to_string()),
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    fmt::init();

    let host = env::var("SKILLHUB_APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("SKILLHUB_APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr = format!("{}:{}", host, port);
    tracing::info!("Starting SkillHub API on {}", addr);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/health", get(health))
        .fallback(not_found);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
