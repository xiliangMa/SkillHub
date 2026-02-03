use axum::{routing::{get, post, delete}, Router, Json, extract::{State, Path, Request, Extension, Query}, http::StatusCode, middleware};
use tokio::net::TcpListener;
use tracing_subscriber::fmt;
use serde::{Serialize, Deserialize};
use std::env;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, Output};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use hyper::HeaderMap;

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

const JWT_SECRET: &[u8] = b"skillhub-secret-key-change-in-production";
const JWT_EXPIRY_HOURS: i64 = 24;

#[derive(Serialize, Deserialize)]
struct UserClaims {
    sub: String,
    user_id: String,
    email: String,
    role: String,
    exp: usize,
    iat: usize,
}

#[derive(Serialize, Deserialize)]
struct UserResponse {
    id: String,
    email: String,
    name: Option<String>,
    role: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: UserResponse,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct AddFavoriteRequest {
    skill_id: String,
}

#[derive(Serialize, Deserialize)]
struct SkillQueryParams {
    search: Option<String>,
    language: Option<String>,
    sort: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct CreatePaymentRequest {
    skill_id: String,
    payment_method: String,
}

#[derive(Serialize)]
struct PaymentResponse {
    id: String,
    amount: f64,
    currency: String,
    status: String,
    qr_code_url: Option<String>,
    payment_url: Option<String>,
}

#[derive(Serialize)]
struct FavoriteResponse {
    id: String,
    skill_id: String,
    skill_name: String,
    skill_description: Option<String>,
    github_owner: String,
    github_repo: String,
    created_at: String,
}

async fn auth_middleware(mut req: Request, next: middleware::Next) -> axum::response::Response {
    let user_id_opt = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .and_then(|token| decode_jwt(token).ok())
        .map(|c| c.claims.user_id)
        .or(None);

    req.extensions_mut().insert(user_id_opt);
    next.run(req).await
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    name: Option<String>,
}

fn hash_password(password: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(rand::thread_rng());
    let hash = argon2.hash_password(password.as_bytes(), &salt.as_salt())
        .map_err(|e| e.to_string())?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::password_hash::PasswordHash;
    use argon2::PasswordVerifier;
    
    eprintln!("DEBUG: Starting password verification");
    eprintln!("DEBUG: Password length: {}", password.len());
    eprintln!("DEBUG: Hash: '{}' (len={})", hash, hash.len());
    
    match PasswordHash::new(hash) {
        Ok(ph) => {
            eprintln!("DEBUG: Successfully parsed hash");
            eprintln!("DEBUG algorithm: {:?}", ph.algorithm);
            eprintln!("DEBUG: Hash version: {:?}", ph.version);
            eprintln!("DEBUG: Hash params: {:?}", ph.params);
            eprintln!("DEBUG: Hash salt present: {}", ph.salt.is_some());
            
            let argon2 = Argon2::default();
            let result = argon2.verify_password(password.as_bytes(), &ph);
            eprintln!("DEBUG: Verification result: {:?}", result);
            result.is_ok()
        }
        Err(e) => {
            eprintln!("DEBUG: Failed to parse hash: {}", e);
            false
        }
    }
}

fn generate_jwt(user_id: &str, email: &str, role: &str) -> Result<String, String> {
    let now = Utc::now();
    let exp = now + Duration::hours(JWT_EXPIRY_HOURS);

    let claims = UserClaims {
        sub: user_id.to_string(),
        user_id: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| e.to_string())
}

fn decode_jwt(token: &str) -> Result<TokenData<UserClaims>, String> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default()
    ).map_err(|e| e.to_string())
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

async fn list_skills(State(pool): State<Pool<Postgres>>, Query(params): Query<SkillQueryParams>) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let limit = params.limit.unwrap_or(50).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let mut query = String::from("SELECT json_build_object('id', id::text, 'github_owner', github_owner, 'github_repo', github_repo, 'name', name, 'description', description, 'language', language, 'stars', stars, 'price', price)::text FROM skills WHERE 1=1");
    let mut param_count = 0;

    if let Some(ref search) = params.search {
        param_count += 1;
        query.push_str(&format!(" AND (name ILIKE ${} OR description ILIKE ${})", param_count, param_count));
    }

    if let Some(ref language) = params.language {
        if language != "all" {
            param_count += 1;
            query.push_str(&format!(" AND language = ${}", param_count));
        }
    }

    query.push_str(&format!(" ORDER BY stars DESC LIMIT {} OFFSET {}", limit, offset));

    let mut query_builder = sqlx::query_as::<_, (String,)>(&query);

    if let Some(search) = &params.search {
        query_builder = query_builder.bind(format!("%{}%", search));
    }

    if let Some(language) = &params.language {
        if language != "all" {
            query_builder = query_builder.bind(language);
        }
    }

    let rows: Vec<(String,)> = query_builder.fetch_all(&pool).await.unwrap_or_default();

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

async fn register(State(pool): State<Pool<Postgres>>, Json(req): Json<RegisterRequest>) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    if req.email.is_empty() || req.password.len() < 6 {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Email and password (min 6 chars) required".to_string()),
        }));
    }

    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT email FROM users WHERE email = $1"
    ).bind(&req.email).fetch_optional(&pool).await.unwrap_or(None);

    if existing.is_some() {
        return (StatusCode::CONFLICT, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Email already registered".to_string()),
        }));
    }

    let password_hash = match hash_password(&req.password) {
        Ok(h) => h,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Password hash failed: {}", e)),
        })),
    };

    let user_id = Uuid::new_v4();
    let role = "user".to_string();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role) VALUES ($1, $2, $3, $4, $5)"
    ).bind(user_id)
     .bind(&req.email)
     .bind(&password_hash)
     .bind(req.name.as_ref().unwrap_or(&req.email.split('@').next().unwrap_or("").to_string()))
     .bind(&role)
     .execute(&pool).await;

    match result {
        Ok(_) => {
            let token = match generate_jwt(&user_id.to_string(), &req.email, &role) {
                Ok(t) => t,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some(format!("Token generation failed: {}", e)),
                })),
            };

            (StatusCode::CREATED, Json(ApiResponse {
                success: true,
                data: Some(LoginResponse {
                    token,
                    user: UserResponse {
                        id: user_id.to_string(),
                        email: req.email.clone(),
                        name: req.name,
                        role,
                    },
                }),
                message: Some("Registration successful".to_string()),
            }))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Database error: {}", e)),
        }))
    }
}

async fn login(State(pool): State<Pool<Postgres>>, Json(req): Json<LoginRequest>) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    eprintln!("LOGIN DEBUG: Looking for email='{}' (len={})", req.email, req.email.len());
    
    let row_result = sqlx::query_as::<_, (String, String, String, String)>(
        "SELECT id::text, email, password_hash, name FROM users WHERE email = $1"
    ).bind(&req.email)
     .fetch_optional(&pool).await;
    
    match row_result {
        Ok(row) => {
            eprintln!("LOGIN DEBUG: row={:?}", row);
            
            match row {
                Some((user_id, email, password_hash, name)) => {
                    eprintln!("LOGIN DEBUG: Found user: email={}, password_hash='{}' (len={})", email, password_hash, password_hash.len());
                    if !verify_password(&req.password, &password_hash) {
                        return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
                            success: false,
                            data: None,
                            message: Some("Invalid credentials".to_string()),
                        }));
                    }

                    let role: String = sqlx::query_scalar(
                        "SELECT role FROM users WHERE id = $1::uuid"
                    ).bind(&user_id).fetch_one(&pool).await.unwrap_or_else(|_| "user".to_string());

                    let token = match generate_jwt(&user_id, &email, &role) {
                        Ok(t) => t,
                        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
                            success: false,
                            data: None,
                            message: Some(format!("Token generation failed: {}", e)),
                        })),
                    };

                    (StatusCode::OK, Json(ApiResponse {
                        success: true,
                        data: Some(LoginResponse {
                            token,
                            user: UserResponse {
                                id: user_id,
                                email,
                                name: Some(name),
                                role,
                            },
                        }),
                        message: Some("Login successful".to_string()),
                    }))
                }
                None => (StatusCode::UNAUTHORIZED, Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Invalid credentials".to_string()),
                }))
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Database error: {}", e)),
        }))
    }
}

async fn auth_me(State(pool): State<Pool<Postgres>>, req: Request) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let auth_header = req.headers().get("authorization");
    let token = match auth_header {
        Some(h) => h.to_str().ok().and_then(|s| s.strip_prefix("Bearer ")).unwrap_or(""),
        None => "",
    };

    if token.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("No token provided".to_string()),
        }));
    }

    let claims = match decode_jwt(token) {
        Ok(c) => c,
        Err(e) => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Invalid token: {}", e)),
        })),
    };

    let user_id = Uuid::parse_str(&claims.claims.user_id).unwrap_or_default();
    let row: Option<(String, String, String)> = sqlx::query_as(
        "SELECT id::text, email, COALESCE(name, '') FROM users WHERE id = $1"
    ).bind(user_id)
     .fetch_optional(&pool).await.unwrap_or(None);

    match row {
        Some((id, email, name)) => {
            let role: String = sqlx::query_scalar(
                "SELECT role FROM users WHERE id = $1::uuid"
            ).bind(user_id).fetch_one(&pool).await.unwrap_or_else(|_| "user".to_string());

            (StatusCode::OK, Json(ApiResponse {
                success: true,
                data: Some(UserResponse {
                    id,
                    email,
                    name: if name.is_empty() { None } else { Some(name) },
                    role,
                }),
                message: None,
            }))
        }
        None => (StatusCode::NOT_FOUND, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("User not found".to_string()),
        }))
    }
}

async fn list_favorites(State(pool): State<Pool<Postgres>>, Extension(user_id_opt): Extension<Option<String>>) -> (StatusCode, Json<ApiResponse<Vec<FavoriteResponse>>>) {
    let user_id_str = match user_id_opt {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Not authenticated".to_string()),
        })),
    };

    let user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
    let rows: Vec<(String, String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT f.id::text, f.skill_id::text, s.name, COALESCE(s.description, ''), s.github_owner, s.github_repo, f.created_at::text
         FROM favorites f
         JOIN skills s ON f.skill_id = s.id
         WHERE f.user_id = $1
         ORDER BY f.created_at DESC"
    ).bind(user_id).fetch_all(&pool).await.unwrap_or_default();

    let favorites: Vec<FavoriteResponse> = rows.into_iter().map(|(id, skill_id, name, desc, owner, repo, created)| {
        FavoriteResponse {
            id,
            skill_id,
            skill_name: name,
            skill_description: if desc.is_empty() { None } else { Some(desc) },
            github_owner: owner,
            github_repo: repo,
            created_at: created,
        }
    }).collect();

    (StatusCode::OK, Json(ApiResponse {
        success: true,
        data: Some(favorites),
        message: None,
    }))
}

async fn add_favorite(State(pool): State<Pool<Postgres>>, Extension(user_id_opt): Extension<Option<String>>, Json(req_body): Json<AddFavoriteRequest>) -> (StatusCode, Json<ApiResponse<FavoriteResponse>>) {
    let user_id_str = match user_id_opt {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Not authenticated".to_string()),
        })),
    };

    let user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
    let skill_id = match Uuid::parse_str(&req_body.skill_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid skill ID".to_string()),
        })),
    };

    let exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM favorites WHERE user_id = $1 AND skill_id = $2"
    ).bind(user_id).bind(skill_id).fetch_optional(&pool).await.unwrap_or(None);

    if exists.is_some() {
        return (StatusCode::CONFLICT, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Already favorited".to_string()),
        }));
    }

    let skill_exists: Option<(String,)> = sqlx::query_as(
        "SELECT name FROM skills WHERE id = $1"
    ).bind(skill_id).fetch_optional(&pool).await.unwrap_or(None);

    if skill_exists.is_none() {
        return (StatusCode::NOT_FOUND, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Skill not found".to_string()),
        }));
    }

    let favorite_id = Uuid::new_v4().to_string();
    let result = sqlx::query(
        "INSERT INTO favorites (id, user_id, skill_id) VALUES ($1, $2, $3)"
    ).bind(&favorite_id)
     .bind(user_id)
     .bind(skill_id)
     .execute(&pool).await;

    match result {
        Ok(_) => {
            let (name, desc, owner, repo, created): (String, String, String, String, String) = sqlx::query_as(
                "SELECT s.name, COALESCE(s.description, ''), s.github_owner, s.github_repo, f.created_at::text
                 FROM favorites f
                 JOIN skills s ON f.skill_id = s.id
                 WHERE f.id = $1"
            ).bind(&favorite_id).fetch_one(&pool).await.unwrap_or_default();

            (StatusCode::CREATED, Json(ApiResponse {
                success: true,
                data: Some(FavoriteResponse {
                    id: favorite_id,
                    skill_id: req_body.skill_id,
                    skill_name: name,
                    skill_description: if desc.is_empty() { None } else { Some(desc) },
                    github_owner: owner,
                    github_repo: repo,
                    created_at: created,
                }),
                message: Some("Added to favorites".to_string()),
            }))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Database error: {}", e)),
        }))
    }
}

async fn remove_favorite(State(pool): State<Pool<Postgres>>, Path(id): Path<String>, Extension(user_id_opt): Extension<Option<String>>) -> (StatusCode, Json<ApiResponse<()>>) {
    let user_id_str = match user_id_opt {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Not authenticated".to_string()),
        })),
    };

    let user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
    let favorite_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid favorite ID".to_string()),
        })),
    };

    let result = sqlx::query(
        "DELETE FROM favorites WHERE id = $1 AND user_id = $2"
    ).bind(favorite_id)
     .bind(user_id)
     .execute(&pool).await;

    match result {
        Ok(r) => {
            if r.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(ApiResponse {
                    success: false,
                    data: None,
                    message: Some("Favorite not found".to_string()),
                }))
            } else {
                (StatusCode::OK, Json(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("Removed from favorites".to_string()),
                }))
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Database error: {}", e)),
        }))
    }
}

async fn create_payment(State(pool): State<Pool<Postgres>>, Extension(user_id_opt): Extension<Option<String>>, Json(req): Json<CreatePaymentRequest>) -> (StatusCode, Json<ApiResponse<PaymentResponse>>) {
    let user_id_str = match user_id_opt {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Not authenticated".to_string()),
        })),
    };

    let user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
    let skill_id = match Uuid::parse_str(&req.skill_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid skill ID".to_string()),
        })),
    };

    let skill: Option<(f64,)> = sqlx::query_as(
        "SELECT price FROM skills WHERE id = $1"
    ).bind(skill_id).fetch_optional(&pool).await.unwrap_or(None);

    let (price,) = match skill {
        Some(p) => p,
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Skill not found".to_string()),
        })),
    };

    if price == 0.0 {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Skill is free, no payment required".to_string()),
        }));
    }

    let payment_id = Uuid::new_v4().to_string();
    let currency = "CNY".to_string();
    let status = "pending".to_string();

    let result = sqlx::query(
        "INSERT INTO payments (id, user_id, skill_id, amount, currency, status, payment_method) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    ).bind(&payment_id)
     .bind(user_id)
     .bind(skill_id)
     .bind(price)
     .bind(&currency)
     .bind(&status)
     .bind(&req.payment_method)
     .execute(&pool).await;

    match result {
        Ok(_) => {
            let (qr_code_url, payment_url): (Option<String>, Option<String>) = match req.payment_method.as_str() {
                "wechat" => (
                    Some(format!("weixin://wxpay/bizpayurl?pr={}", payment_id)),
                    None
                ),
                "alipay" => (
                    None,
                    Some(format!("https://alipay.com/pay?orderId={}", payment_id))
                ),
                "stripe" => (
                    None,
                    Some(format!("https://checkout.stripe.com/pay/{}", payment_id))
                ),
                _ => (None, None),
            };

            (StatusCode::CREATED, Json(ApiResponse {
                success: true,
                data: Some(PaymentResponse {
                    id: payment_id,
                    amount: price,
                    currency,
                    status,
                    qr_code_url,
                    payment_url,
                }),
                message: Some("Payment created".to_string()),
            }))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse {
            success: false,
            data: None,
            message: Some(format!("Database error: {}", e)),
        }))
    }
}

async fn payment_status(State(pool): State<Pool<Postgres>>, Extension(user_id_opt): Extension<Option<String>>, Path(id): Path<String>) -> (StatusCode, Json<ApiResponse<PaymentResponse>>) {
    let user_id_str = match user_id_opt {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Not authenticated".to_string()),
        })),
    };

    let _user_id = Uuid::parse_str(&user_id_str).unwrap_or_default();
    let payment_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid payment ID".to_string()),
        })),
    };

    let row: Option<(String, f64, String, String)> = sqlx::query_as(
        "SELECT id::text, amount, currency, status FROM payments WHERE id = $1"
    ).bind(payment_id).fetch_optional(&pool).await.unwrap_or(None);

    match row {
        Some((id, amount, currency, status)) => (StatusCode::OK, Json(ApiResponse {
            success: true,
            data: Some(PaymentResponse {
                id,
                amount,
                currency,
                status,
                qr_code_url: None,
                payment_url: None,
            }),
            message: None,
        })),
        None => (StatusCode::NOT_FOUND, Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Payment not found".to_string()),
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    fmt::init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://skillhub:skillhub@localhost:5432/skillhub".to_string()
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
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", get(auth_me))
        .route("/api/favorites", get(list_favorites).route_layer(middleware::from_fn(auth_middleware)))
        .route("/api/favorites", post(add_favorite).route_layer(middleware::from_fn(auth_middleware)))
        .route("/api/favorites/:id", delete(remove_favorite).route_layer(middleware::from_fn(auth_middleware)))
        .route("/api/payments", post(create_payment).route_layer(middleware::from_fn(auth_middleware)))
        .route("/api/payments/:id", get(payment_status).route_layer(middleware::from_fn(auth_middleware)))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([hyper::Method::GET, hyper::Method::POST, hyper::Method::PUT, hyper::Method::DELETE])
            .allow_headers(Any))
        .with_state(pool);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
