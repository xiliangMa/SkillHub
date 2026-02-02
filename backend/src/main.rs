use axum::{routing::get, Router, Json, extract::State, extract::Path, http::StatusCode};
use tokio::net::TcpListener;
use tracing_subscriber::fmt;
use serde::Serialize;
use std::env;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

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

#[derive(Serialize)]
struct StatsResponse {
    total_users: i64,
    total_skills: i64,
    total_payments: i64,
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

async fn list_skills(State(pool): State<Pool<Postgres>>) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT json_build_object('id', id::text, 'github_owner', github_owner, 'github_repo', github_repo, 'name', name, 'description', description, 'language', language, 'stars', stars, 'price', price)::text FROM skills ORDER BY stars DESC LIMIT 50"
    )
    .fetch_all(&pool).await
    .unwrap_or_default();

    let items: Vec<serde_json::Value> = rows.into_iter()
        .filter_map(|(json_str,)| serde_json::from_str(&json_str).ok())
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(items),
        message: None,
    })
}

async fn get_skill(State(pool): State<Pool<Postgres>>, Path(id): Path<String>) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let uuid_id:Uuid = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Invalid UUID".to_string()),
            }));
        }
    };

    let row: Option<(String,)> = sqlx::query_as(
        "SELECT json_build_object('id', id::text, 'github_owner', github_owner, 'github_repo', github_repo, 'name', name, 'description', description, 'language', language, 'stars', stars, 'price', price, 'created_at', created_at::text)::text FROM skills WHERE id = $1"
    )
    .bind(uuid_id)
    .fetch_optional(&pool).await.unwrap_or(None);

    match row {
        Some((json_str,)) => {
            match serde_json::from_str::<serde_json::Value>(&json_str) {
                Ok(data) => (StatusCode::OK, Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    message: None,
                })),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Failed to parse skill data".to_string()),
                }))
            }
        }
        None => (StatusCode::NOT_FOUND, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Skill not found".to_string()),
        }))
    }
}

async fn admin_stats(State(pool): State<Pool<Postgres>>) -> Json<ApiResponse<StatsResponse>> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&pool).await.unwrap_or(0);
    let skills: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM skills").fetch_one(&pool).await.unwrap_or(0);
    let payments: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM payments").fetch_one(&pool).await.unwrap_or(0);

    Json(ApiResponse {
        success: true,
        data: Some(StatsResponse { total_users: users, total_skills: skills, total_payments: payments }),
        message: None,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://skillhub:skillhub123@localhost:5432/skillhub".to_string()
    });
    let pool = sqlx::Pool::connect(&database_url).await?;
    tracing::info!("Connected to database");

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
        .route("/api/skills", get(list_skills))
        .route("/api/skills/:id", get(get_skill))
        .route("/api/admin/stats", get(admin_stats))
        .with_state(pool);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
