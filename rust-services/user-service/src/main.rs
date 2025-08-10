use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/consciousness".to_string());
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "super_secure_jwt_secret_key_123".to_string());

    let db = sqlx::PgPool::connect(&database_url).await?;

    let state = AppState {
        db,
        jwt_secret,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_current_user))
        .route("/users/:id", get(get_user))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    info!("👤 User Service running on http://0.0.0.0:8081");
    
    axum::serve(listener, app).await?;
    Ok(())
}

#[instrument]
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "user-service",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now()
    }))
}

#[instrument(skip(state))]
async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Validate email format
    if !request.email.contains('@') {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash = hash(request.password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user
    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, $3, $4)",
        user_id,
        request.email,
        password_hash,
        request.name
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate JWT token
    let token = generate_token(&state.jwt_secret, &user_id.to_string(), &request.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = UserResponse {
        id: user_id.to_string(),
        email: request.email,
        name: request.name,
        created_at: chrono::Utc::now(),
    };

    Ok(Json(AuthResponse { token, user }))
}

#[instrument(skip(state))]
async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Find user
    let user = sqlx::query!(
        "SELECT id, email, password_hash, name, created_at FROM users WHERE email = $1 AND is_active = true",
        request.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = verify(request.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT token
    let token = generate_token(&state.jwt_secret, &user.id.to_string(), &user.email)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_response = UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.unwrap_or_else(chrono::Utc::now),
    };

    Ok(Json(AuthResponse { token: token, user: user_response }))
}

fn generate_token(secret: &str, user_id: &str, email: &str) -> anyhow::Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

async fn extract_user_from_token(headers: &HeaderMap, secret: &str) -> Result<Claims, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data.claims)
}

#[instrument(skip(state))]
async fn get_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, StatusCode> {
    let claims = extract_user_from_token(&headers, &state.jwt_secret).await?;
    
    let user = sqlx::query!(
        "SELECT id, email, name, created_at FROM users WHERE id = $1 AND is_active = true",
        Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.unwrap_or_else(chrono::Utc::now),
    }))
}

#[instrument(skip(state))]
async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<UserResponse>, StatusCode> {
    let user = sqlx::query!(
        "SELECT id, email, name, created_at FROM users WHERE id = $1 AND is_active = true",
        Uuid::parse_str(&user_id).map_err(|_| StatusCode::BAD_REQUEST)?
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
        created_at: user.created_at.unwrap_or_else(chrono::Utc::now),
    }))
}
