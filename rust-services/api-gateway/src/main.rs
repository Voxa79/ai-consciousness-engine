use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    middleware,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, instrument, warn};

#[derive(Clone)]
pub struct AppState {
    pub consciousness_engine_url: String,
    pub user_service_url: String,
    pub jwt_secret: String,
    pub client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct ConsciousnessRequest {
    pub content: String,
    pub user_id: Option<String>,
    pub context: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub services: HashMap<String, ServiceHealth>,
}

#[derive(Serialize)]
pub struct ServiceHealth {
    pub status: String,
    pub response_time_ms: Option<u64>,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct MetricsResponse {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub uptime_seconds: u64,
    pub active_connections: u32,
}

static mut METRICS: MetricsResponse = MetricsResponse {
    total_requests: 0,
    successful_requests: 0,
    failed_requests: 0,
    average_response_time_ms: 0.0,
    uptime_seconds: 0,
    active_connections: 0,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();

    let consciousness_engine_url = std::env::var("CONSCIOUSNESS_ENGINE_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    let user_service_url = std::env::var("USER_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8081".to_string());
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "super_secure_jwt_secret_key_123".to_string());

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let state = AppState {
        consciousness_engine_url,
        user_service_url,
        jwt_secret,
        client,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        .route("/api/v1/consciousness/process", post(process_consciousness))
        .route("/api/v1/consciousness/state", get(get_consciousness_state))
        .route("/api/v1/auth/register", post(proxy_auth_register))
        .route("/api/v1/auth/login", post(proxy_auth_login))
        .route("/api/v1/auth/me", get(proxy_auth_me))
        .route("/api/v1/conversations/:user_id", get(get_user_conversations))
        .route("/api/v1/agents", get(list_agents))
        .layer(middleware::from_fn_with_state(state.clone(), metrics_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("🌐 API Gateway running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn metrics_middleware<B>(
    State(state): State<Arc<AppState>>,
    request: axum::extract::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let start = std::time::Instant::now();
    
    unsafe {
        METRICS.total_requests += 1;
        METRICS.active_connections += 1;
    }
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    
    unsafe {
        METRICS.active_connections -= 1;
        
        if response.status().is_success() {
            METRICS.successful_requests += 1;
        } else {
            METRICS.failed_requests += 1;
        }
        
        // Update average response time
        let total_responses = METRICS.successful_requests + METRICS.failed_requests;
        if total_responses > 0 {
            METRICS.average_response_time_ms = 
                (METRICS.average_response_time_ms * (total_responses - 1) as f64 + duration.as_millis() as f64) 
                / total_responses as f64;
        }
    }
    
    response
}

#[instrument]
async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let mut services = HashMap::new();
    
    // Check Consciousness Engine
    let consciousness_health = check_service_health(&state.client, &state.consciousness_engine_url, "/health").await;
    services.insert("consciousness_engine".to_string(), consciousness_health);
    
    // Check User Service
    let user_health = check_service_health(&state.client, &state.user_service_url, "/health").await;
    services.insert("user_service".to_string(), user_health);
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        uptime_seconds: 3600, // TODO: Calculate real uptime
        services,
    })
}

async fn check_service_health(client: &reqwest::Client, base_url: &str, endpoint: &str) -> ServiceHealth {
    let start = std::time::Instant::now();
    
    match client.get(&format!("{}{}", base_url, endpoint)).send().await {
        Ok(response) if response.status().is_success() => {
            ServiceHealth {
                status: "healthy".to_string(),
                response_time_ms: Some(start.elapsed().as_millis() as u64),
                last_check: chrono::Utc::now(),
            }
        }
        _ => {
            ServiceHealth {
                status: "unhealthy".to_string(),
                response_time_ms: None,
                last_check: chrono::Utc::now(),
            }
        }
    }
}

#[instrument]
async fn get_metrics() -> Json<MetricsResponse> {
    unsafe {
        Json(MetricsResponse {
            total_requests: METRICS.total_requests,
            successful_requests: METRICS.successful_requests,
            failed_requests: METRICS.failed_requests,
            average_response_time_ms: METRICS.average_response_time_ms,
            uptime_seconds: METRICS.uptime_seconds,
            active_connections: METRICS.active_connections,
        })
    }
}

#[instrument(skip(state))]
async fn process_consciousness(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<ConsciousnessRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Extract user from token if present
    let user_id = extract_user_from_headers(&headers, &state.jwt_secret)
        .unwrap_or_else(|| request.user_id.unwrap_or_else(|| "anonymous".to_string()));
    
    let enhanced_request = serde_json::json!({
        "content": request.content,
        "user_id": user_id,
        "context": request.context.unwrap_or_else(|| serde_json::json!({}))
    });
    
    match state.client
        .post(&format!("{}/consciousness/process", state.consciousness_engine_url))
        .json(&enhanced_request)
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(e) => {
                    warn!("Failed to parse consciousness response: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(response) => {
            warn!("Consciousness engine returned error: {}", response.status());
            Err(StatusCode::BAD_GATEWAY)
        }
        Err(e) => {
            warn!("Failed to connect to consciousness engine: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

#[instrument(skip(state))]
async fn get_consciousness_state(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    match state.client
        .get(&format!("{}/consciousness/state", state.consciousness_engine_url))
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(e) => {
                    warn!("Failed to parse consciousness state: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        _ => Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[instrument(skip(state))]
async fn proxy_auth_register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    proxy_to_user_service(&state, "/auth/register", "POST", Some(request)).await
}

#[instrument(skip(state))]
async fn proxy_auth_login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    proxy_to_user_service(&state, "/auth/login", "POST", Some(request)).await
}

#[instrument(skip(state))]
async fn proxy_auth_me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let mut req_builder = state.client.get(&format!("{}/auth/me", state.user_service_url));
    
    if let Some(auth_header) = headers.get("authorization") {
        req_builder = req_builder.header("authorization", auth_header);
    }
    
    let request_id = uuid::Uuid::new_v4().to_string();
    
    match req_builder.send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn proxy_to_user_service(
    state: &AppState,
    endpoint: &str,
    method: &str,
    body: Option<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let url = format!("{}{}", state.user_service_url, endpoint);
    
    let req_builder = match method {
        "POST" => state.client.post(&url),
        "GET" => state.client.get(&url),
        _ => return Err(StatusCode::METHOD_NOT_ALLOWED),
    };
    
    let req_builder = if let Some(body) = body {
        req_builder.json(&body)
    } else {
        req_builder
    };
    
    match req_builder.send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Ok(response) => {
            let status_code = response.status();
            match response.text().await {
                Ok(error_text) => Ok(Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(error_text),
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(_) => Err(status_code)
            }
        }
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[instrument(skip(state))]
async fn get_user_conversations(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let mut req_builder = state.client.get(&format!("{}/conversations/{}", state.consciousness_engine_url, user_id));
    
    if let Some(auth_header) = headers.get("authorization") {
        req_builder = req_builder.header("authorization", auth_header);
    }
    
    let request_id = uuid::Uuid::new_v4().to_string();
    
    match req_builder.send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(Json(ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                    timestamp: chrono::Utc::now(),
                    request_id,
                })),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

#[instrument]
async fn list_agents() -> Json<ApiResponse<serde_json::Value>> {
    let agents = serde_json::json!([
        {
            "id": "consciousness-engine",
            "name": "Consciousness Engine",
            "description": "IA consciente avec raisonnement éthique et intelligence émotionnelle",
            "status": "active",
            "capabilities": ["conversation", "reasoning", "ethics", "creativity", "empathy"],
            "model": "qwen2.5:3b-instruct-q4_k_m"
        }
    ]);
    
    Json(ApiResponse {
        success: true,
        data: Some(agents),
        error: None,
        timestamp: chrono::Utc::now(),
        request_id: uuid::Uuid::new_v4().to_string(),
    })
}

fn extract_user_from_headers(headers: &HeaderMap, _jwt_secret: &str) -> Option<String> {
    // TODO: Implement JWT token validation
    headers.get("x-user-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}
