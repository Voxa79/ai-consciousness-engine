//! API Gateway pour Consciousness Engine
//! 
//! Gateway unifié pour tous les services consciousness avec authentification,
//! rate limiting, monitoring, et routing intelligent.

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    middleware,
    response::{Json, sse::{Sse, Event}},
    routing::{get, post, put, delete},
    Router,
};
use utoipa::{OpenApi, Modify};
use utoipa::openapi::{self, security::{SecurityScheme, HttpAuthScheme, HttpBuilder}, SecurityRequirement};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}, time::{Duration, Instant}};
use std::convert::Infallible;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::StreamExt; // filter_map, map pour BroadcastStream
use tokio_stream::wrappers::ReceiverStream;
use tower::ServiceBuilder;
use tower_http::{
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    cors::CorsLayer,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

// Regrouper les modèles OpenAPI exposés dans un module dédié
mod models;
use crate::models::{
    HealthResponse,
    MetricsResponse,
    ConsciousnessRequest,
    ConsciousnessResult,
    ConsciousnessState,
    AgentSummary,
    AgentDetail,
    AgentListResponse,
    AgentExecutionResult,
    PoliciesResponse,
    AuditLogsResponse,
    ComplianceStatus,
    PolicySummary,
    ErrorResponse,
};

// Documentation OpenAPI: déplacée en bas du fichier pour garantir que tous les schémas soient définis avant son expansion.

// ------------------------
// Config, State et Helpers
// ------------------------

#[derive(Clone, Debug)]
struct GatewayConfig {
    port: u16,
    consciousness_engine_url: String,
    agent_orchestrator_url: String,
    ai_governance_url: String,
    jwt_secret: String,
    rate_limit_requests_per_minute: u32,
    request_timeout_seconds: u64,
    auth_optional: bool,
}

/// Décrire une image via un VLM (proxifié vers le mvp-server)
#[utoipa::path(
    post,
    path = "/api/v1/vision/describe",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Description d'image (proxifiée)", body = serde_json::Value)),
    security(())
)]
async fn vision_describe(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("{}/vision/describe", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Transcrire de l'audio (ASR) proxifié vers le mvp-server
#[utoipa::path(
    post,
    path = "/api/v1/asr/transcribe",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Transcription (proxifiée)", body = serde_json::Value)),
    security(())
)]
async fn asr_transcribe(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("{}/asr/transcribe", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Synthèse vocale (TTS) proxifiée vers le mvp-server
#[utoipa::path(
    post,
    path = "/api/v1/tts/synthesize",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Audio encodé (proxifié)", body = serde_json::Value)),
    security(())
)]
async fn tts_synthesize(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("{}/tts/synthesize", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

// ------------------
// LLM Proxy Endpoints
// ------------------

#[utoipa::path(
    post,
    path = "/api/v1/llm/generate",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Réponse Ollama generate (proxifiée)", body = serde_json::Value))
)]
async fn llm_generate(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("{}/llm/generate", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/llm/chat",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Réponse Ollama chat (proxifiée)", body = serde_json::Value))
)]
async fn llm_chat(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let url = format!("{}/llm/chat", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/evaluate",
    tag = "evaluation",
    request_body = serde_json::Value,
    responses((status = 200, description = "Réponse /evaluate proxifiée", body = serde_json::Value))
)]
async fn evaluate_proxy(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Proxy vers le mvp-server (agent_orchestrator_url pointe sur http://mvp-server:4000)
    let url = format!("{}/evaluate", state.config.agent_orchestrator_url);
    match state.http_client.post(&url).json(&body).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/llm/stream",
    tag = "orchestration",
    request_body = serde_json::Value,
    responses((status = 200, description = "Flux SSE proxifié", body = String))
)]
async fn llm_stream(
    State(state): State<GatewayState>,
    Json(body): Json<serde_json::Value>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
    let url = format!("{}/llm/stream", state.config.agent_orchestrator_url);
    let client = state.http_client.clone();
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(64);

    tokio::spawn(async move {
        match client.post(&url).json(&body).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let _ = tx
                        .send(Ok(Event::default()
                            .event("error")
                            .data(serde_json::json!({"status": resp.status().as_u16()}).to_string())))
                        .await;
                    return;
                }
                let mut stream = resp.bytes_stream();
                let mut buf: Vec<u8> = Vec::new();
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(bytes) => {
                            buf.extend_from_slice(bytes.as_ref());
                            // découper par frames SSE (\n\n)
                            while let Some(pos) = buf.windows(2).position(|w| w == b"\n\n") {
                                let frame = buf.drain(..pos+2).collect::<Vec<u8>>();
                                if let Ok(text) = String::from_utf8(frame) {
                                    let mut event_name: Option<String> = None;
                                    for line in text.lines() {
                                        if let Some(v) = line.strip_prefix("event: ") {
                                            event_name = Some(v.trim().to_string());
                                        } else if let Some(v) = line.strip_prefix("data: ") {
                                            let name = event_name.clone().unwrap_or_else(|| "message".to_string());
                                            let _ = tx
                                                .send(Ok(Event::default()
                                                    .event(name)
                                                    .data(v.to_string())))
                                                .await;
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
            Err(_) => {
                let _ = tx
                    .send(Ok(Event::default()
                        .event("error")
                        .data(serde_json::json!({"message": "upstream unreachable"}).to_string())))
                    .await;
            }
        }
    });

    Sse::new(ReceiverStream::new(rx))
}

/// Liste les modèles disponibles côté Ollama via le MVP server
#[utoipa::path(
    get,
    path = "/api/v1/llm/models",
    tag = "orchestration",
    responses((status = 200, description = "Liste des modèles disponibles", body = serde_json::Value)),
    security(())
)]
async fn llm_models(
    State(state): State<GatewayState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // On interroge le MVP server qui expose /llm/health (inclut la liste des modèles)
    let url = format!("{}/llm/health", state.config.agent_orchestrator_url);
    match state.http_client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // Normalise en { models: [...] }
                        let models = json.get("models").cloned().unwrap_or_else(|| serde_json::json!([]));
                        Ok(Json(serde_json::json!({ "models": models })))
                    }
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY))
            }
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/orchestrator/mock-run",
    tag = "orchestrator",
    responses((status = 200, description = "Démarrage d'un run mock orchestrator (SSE proxifié)", body = String)),
    security(())
)]
/// Déclenche l'orchestrateur mock (mvp-server) et proxifie son flux SSE vers le bus local
async fn orchestrator_mock_run(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    let base = state.config.agent_orchestrator_url.clone();
    let url = format!("{}/orchestrator/mock-sse?tokens=40&interval_ms=50", base);
    let client = state.http_client.clone();
    let tx = state.event_tx.clone();

    // On lit le flux SSE en tâche de fond et on republie chaque ligne `data: {json}`
    tokio::spawn(async move {
        match client.get(&url).send().await {
            Ok(resp) => {
                if !resp.status().is_success() { return; }
                let mut stream = resp.bytes_stream();
                let mut buffer: Vec<u8> = Vec::new();
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(bytes) => {
                            buffer.extend_from_slice(bytes.as_ref());
                            // Un événement SSE est séparé par une ligne vide (\n\n)
                            while let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
                                let frame = buffer.drain(..pos+2).collect::<Vec<u8>>();
                                if let Ok(text) = String::from_utf8(frame) {
                                    for line in text.lines() {
                                        if let Some(rest) = line.strip_prefix("data: ") {
                                            // rest devrait être un JSON déjà structuré {topic, type, ...}
                                            let _ = tx.send(rest.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
            Err(_) => { /* ignore */ }
        }
    });

    Json(serde_json::json!({ "status": "started" }))
}

#[derive(Clone)]
struct GatewayState {
    config: GatewayConfig,
    http_client: reqwest::Client,
    rate_limiter: Arc<Mutex<RateLimiter>>, 
    metrics: Arc<GatewayMetrics>,
    event_tx: broadcast::Sender<String>,
}

#[derive(Default)]
struct GatewayMetrics {
    total_requests: std::sync::atomic::AtomicU64,
    successful_requests: std::sync::atomic::AtomicU64,
    failed_requests: std::sync::atomic::AtomicU64,
    consciousness_requests: std::sync::atomic::AtomicU64,
}

impl GatewayMetrics {
    fn increment_total(&self) {
        self.total_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn increment_success(&self) {
        self.successful_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn increment_failure(&self) {
        self.failed_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    fn increment_consciousness(&self) {
        self.consciousness_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

struct RateLimiter {
    window_start: Instant,
    counts: HashMap<String, u32>,
}

impl RateLimiter {
    fn new() -> Self {
        Self { window_start: Instant::now(), counts: HashMap::new() }
    }

    fn check_rate_limit(&mut self, client_id: &str, limit_per_minute: u32) -> bool {
        // reset window every minute
        if self.window_start.elapsed() >= Duration::from_secs(60) {
            self.window_start = Instant::now();
            self.counts.clear();
        }
        let entry = self.counts.entry(client_id.to_string()).or_insert(0);
        if *entry >= limit_per_minute { return false; }
        *entry += 1;
        true
    }
}

/// Créer le routeur principal
 pub fn create_gateway_router(state: GatewayState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        // Server-Sent Events (SSE)
        .route("/events", get(sse_events))
        // Mock streaming (démo)
        .route("/api/v1/mock/stream", post(mock_stream))
        // Orchestrator: déclenche un run mock et proxifie le SSE (mvp-server)
        .route("/api/v1/orchestrator/mock-run", post(orchestrator_mock_run))
        // LLM proxy -> mvp-server
        .route("/api/v1/llm/models", get(llm_models))
        .route("/api/v1/llm/generate", post(llm_generate))
        .route("/api/v1/llm/chat", post(llm_chat))
        .route("/api/v1/llm/stream", post(llm_stream))
        // Multimodal proxies
        .route("/api/v1/vision/describe", post(vision_describe))
        .route("/api/v1/asr/transcribe", post(asr_transcribe))
        .route("/api/v1/tts/synthesize", post(tts_synthesize))
        .route("/api/v1/evaluate", post(evaluate_proxy))
        // Swagger UI + OpenAPI via utoipa
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        
        // Consciousness Engine routes
        .route("/api/v1/consciousness/process", post(process_consciousness))
        .route("/api/v1/consciousness/state", get(get_consciousness_state))
        .route("/api/v1/consciousness/reflection", post(generate_reflection))
        .route("/api/v1/agents/:agent_id", put(update_agent))
        .route("/api/v1/agents/:agent_id", delete(delete_agent))
        .route("/api/v1/agents/:agent_id/execute", post(execute_agent))
        
        // AI Governance routes
        .route("/api/v1/governance/policies", get(list_policies))
        .route("/api/v1/governance/audit", get(get_audit_logs))
        .route("/api/v1/governance/compliance", get(check_compliance))
        
        .with_state(state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CorsLayer::permissive())
        )
        // Auth + Rate limit (après couches HTTP)
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

/// Middleware d'authentification + rate limiting (Axum 0.7)
async fn auth_middleware(
    State(state): State<GatewayState>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let path = request.uri().path();
    // Routes publiques
    if matches!(path, "/health" | "/metrics" | "/docs" | "/openapi.json" | "/events" | "/api/v1/mock/stream" | "/api/v1/orchestrator/mock-run" | "/api/v1/llm/generate" | "/api/v1/llm/chat" | "/api/v1/llm/stream" | "/api/v1/llm/models" | "/api/v1/evaluate" | "/api/v1/vision/describe" | "/api/v1/asr/transcribe" | "/api/v1/tts/synthesize") {
        return Ok(next.run(request).await);
    }

    // Rate limiting simple par x-client-id (ou "anonymous")
    let headers = request.headers().clone();
    let client_id = headers
        .get("x-client-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("anonymous");
    {
        let mut limiter = state
            .rate_limiter
            .lock()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if !limiter.check_rate_limit(client_id, state.config.rate_limit_requests_per_minute) {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    // Auth JWT (optionnelle en dev)
    if state.config.auth_optional {
        return Ok(next.run(request).await);
    }

    let auth = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let token = auth.strip_prefix("Bearer ");
    let Some(token) = token else { return Err(StatusCode::UNAUTHORIZED) };

    // Validation basique (signature HMAC), claims ignorés
    let dec_key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
    let validation = Validation::default();
    let _ = decode::<serde_json::Value>(token, &dec_key, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}

/// Health check endpoint

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses((status = 200, description = "Etat du service", body = HealthResponse)),
    security(())
)]
async fn health_check(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    let payload = serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "services": {
            "consciousness_engine": "available",
            "agent_orchestrator": "available",
            "ai_governance": "available"
        }
    });

    // Publier un petit event pour valider le flux SSE
    let _ = state.event_tx.send(
        serde_json::json!({
            "topic": "system",
            "type": "health",
            "timestamp": chrono::Utc::now(),
            "data": payload
        }).to_string()
    );

    Json(payload)
}

#[utoipa::path(
    get,
    path = "/events",
    tag = "system",
    params(("topic" = Option<String>, Query, description = "Filtrer par topic (ex: system, agent:{id}, execution:{id})")),
    responses((status = 200, description = "Flux SSE d'événements en temps réel", body = String)),
    security(())
)]
/// SSE: diffuse les événements en temps réel (topic facultatif)
async fn sse_events(
    State(state): State<GatewayState>,
    Query(params): Query<HashMap<String, String>>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut rx: broadcast::Receiver<String> = state.event_tx.subscribe();
    let topic_filter = params.get("topic").cloned();

    let stream = BroadcastStream::new(rx)
        .filter_map(move |msg: Result<String, BroadcastStreamRecvError>| {
            let topic_filter = topic_filter.clone();
            match msg {
                Ok(text) => {
                    if let Some(ref tf) = topic_filter {
                        // Si un topic est demandé, on filtre côté client
                        // Ici on attend un champ JSON "topic"
                        let pass = serde_json::from_str::<serde_json::Value>(&text)
                            .ok()
                            .and_then(|v| v.get("topic").and_then(|t| t.as_str()).map(|s| s == tf))
                            .unwrap_or(true);
                        if !pass { return None; }
                    }
                    Some(Event::default().data(text))
                }
                Err(_) => None,
            }
        })
        .map(|evt| Ok::<Event, Infallible>(evt));

    Sse::new(stream)
}

#[utoipa::path(
    post,
    path = "/api/v1/mock/stream",
    tag = "system",
    responses((status = 200, description = "Démarrage d'un flux simulé", body = String)),
    security(())
)]
/// Démarre un flux simulé qui publie des événements sur SSE (topic: execution:{id})
async fn mock_stream(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    let exec_id = uuid::Uuid::new_v4().to_string();
    let topic = format!("execution:{}", exec_id);
    let tx = state.event_tx.clone();
    let topic_clone = topic.clone();

    // publish start synchronously
    let _ = tx.send(serde_json::json!({
        "topic": topic,
        "type": "start",
        "timestamp": chrono::Utc::now(),
        "data": {"message": "Mock stream started"}
    }).to_string());

    // spawn async sequence
    tokio::spawn(async move {
        for i in 1..=5 {
            sleep(Duration::from_millis(400)).await;
            let _ = tx.send(serde_json::json!({
                "topic": topic_clone,
                "type": "token",
                "timestamp": chrono::Utc::now(),
                "data": {"index": i, "content": format!("token-{}", i)}
            }).to_string());
        }
        sleep(Duration::from_millis(300)).await;
        let _ = tx.send(serde_json::json!({
            "topic": topic_clone,
            "type": "complete",
            "timestamp": chrono::Utc::now(),
            "data": {"message": "Mock stream completed"}
        }).to_string());
    });

    Json(serde_json::json!({
        "id": exec_id,
        "status": "started",
        "topic": topic
    }))
}

/// Get gateway metrics
/// Métriques du gateway

#[utoipa::path(
    get,
    path = "/metrics",
    tag = "system",
    responses((status = 200, description = "Métriques du gateway", body = MetricsResponse)),
    security(())
)]
async fn get_metrics(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "total_requests": state.metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed),
        "successful_requests": state.metrics.successful_requests.load(std::sync::atomic::Ordering::Relaxed),
        "failed_requests": state.metrics.failed_requests.load(std::sync::atomic::Ordering::Relaxed),
        "consciousness_requests": state.metrics.consciousness_requests.load(std::sync::atomic::Ordering::Relaxed),
    }))
}

// Consciousness Engine Endpoints

/// Process consciousness request
#[utoipa::path(
    post,
    path = "/api/v1/consciousness/process",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    request_body = ConsciousnessRequest,
    responses(
        (status = 200, description = "Requête traitée avec succès", body = ConsciousnessResult, example = json!({
            "request_id": "req_01HZYZABCDEF",
            "label": "self-aware",
            "score": 0.82,
            "reasoning": "Le contenu montre des indications d'introspection et de cohérence contextuelle.",
            "created_at": "2025-08-08T13:45:00Z"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn process_consciousness(
    State(state): State<GatewayState>,
    Json(request): Json<ConsciousnessRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    state.metrics.increment_consciousness();
    
    let url = format!("{}/consciousness/process", state.config.consciousness_engine_url);
    
    match state.http_client.post(&url).json(&request).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Get consciousness state
#[utoipa::path(
    get,
    path = "/api/v1/consciousness/state",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "État courant retourné", body = ConsciousnessState, example = json!({
            "status": "ready",
            "last_updated": "2025-08-08T13:40:00Z",
            "metrics": {"throughput_rps": 12.5, "avg_latency_ms": 85.3}
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn get_consciousness_state(State(state): State<GatewayState>) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    
    let url = format!("{}/consciousness/state", state.config.consciousness_engine_url);
    
    match state.http_client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Generate reflection
#[utoipa::path(
    post,
    path = "/api/v1/consciousness/reflection",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Réflexion générée", body = serde_json::Value))
)]
async fn generate_reflection(State(state): State<GatewayState>) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    
    let url = format!("{}/consciousness/reflection", state.config.consciousness_engine_url);
    
    match state.http_client.post(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Get growth opportunities
#[utoipa::path(
    get,
    path = "/api/v1/consciousness/growth",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Opportunités de croissance", body = serde_json::Value))
)]
async fn get_growth_opportunities(State(state): State<GatewayState>) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    
    let url = format!("{}/consciousness/growth", state.config.consciousness_engine_url);
    
    match state.http_client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Explain response
#[utoipa::path(
    get,
    path = "/api/v1/consciousness/explain/{response_id}",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    params(("response_id" = String, Path, description = "Identifiant de la réponse à expliquer")),
    responses((status = 200, description = "Explication retournée", body = serde_json::Value))
)]
async fn explain_response(
    State(state): State<GatewayState>,
    Path(response_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    
    let mut url = format!("{}/consciousness/explain/{}", state.config.consciousness_engine_url, response_id);
    
    if !params.is_empty() {
        let query_string = params.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        url.push_str(&format!("?{}", query_string));
    }
    
    match state.http_client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Reset consciousness
#[utoipa::path(
    post,
    path = "/api/v1/consciousness/reset",
    tag = "consciousness",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Réinitialisation effectuée", body = serde_json::Value))
)]
async fn reset_consciousness(State(state): State<GatewayState>) -> Result<Json<serde_json::Value>, StatusCode> {
    state.metrics.increment_total();
    
    let url = format!("{}/consciousness/reset", state.config.consciousness_engine_url);
    
    match state.http_client.post(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                state.metrics.increment_success();
                match response.json::<serde_json::Value>().await {
                    Ok(json) => Ok(Json(json)),
                    Err(_) => {
                        state.metrics.increment_failure();
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            } else {
                state.metrics.increment_failure();
                Err(StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            }
        },
        Err(_) => {
            state.metrics.increment_failure();
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

// Agent Orchestrator Endpoints (Mock implementations)

#[utoipa::path(
    get,
    path = "/api/v1/agents",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Liste des agents", body = AgentListResponse, example = json!({
            "agents": [],
            "total": 0
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn list_agents(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "agents": [],
        "total": 0
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/agents",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Agent créé", body = AgentDetail, example = json!({
            "id": "agent_123",
            "status": "created",
            "config": {"type": "reactive"}
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn create_agent(
    State(state): State<GatewayState>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "id": uuid::Uuid::new_v4().to_string(),
        "status": "created",
        "config": _request
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/agents/{agent_id}",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    params(("agent_id" = String, Path, description = "Identifiant de l'agent")),
    responses(
        (status = 200, description = "Détails de l'agent", body = AgentDetail, example = json!({
            "id": "agent_123",
            "status": "active",
            "consciousness_level": 0.85
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn get_agent(
    State(state): State<GatewayState>,
    Path(agent_id): Path<String>,
) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "id": agent_id,
        "status": "active",
        "consciousness_level": 0.85
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/agents/{agent_id}",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    params(("agent_id" = String, Path, description = "Identifiant de l'agent")),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Agent mis à jour", body = AgentDetail, example = json!({
            "id": "agent_123",
            "status": "updated",
            "config": {"type": "reactive", "policy": "safe"}
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn update_agent(
    State(state): State<GatewayState>,
    Path(agent_id): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "id": agent_id,
        "status": "updated",
        "config": request
    }))
}

#[utoipa::path(
    delete,
    path = "/api/v1/agents/{agent_id}",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    params(("agent_id" = String, Path, description = "Identifiant de l'agent")),
    responses(
        (status = 200, description = "Agent supprimé", body = serde_json::Value, example = json!({
            "id": "agent_123",
            "status": "deleted"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn delete_agent(
    State(state): State<GatewayState>,
    Path(agent_id): Path<String>,
) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "id": agent_id,
        "status": "deleted"
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/agents/{agent_id}/execute",
    tag = "orchestration",
    security(("bearerAuth" = [])),
    params(("agent_id" = String, Path, description = "Identifiant de l'agent")),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Exécution réalisée", body = AgentExecutionResult, example = json!({
            "agent_id": "agent_123",
            "execution_id": "exe_01HZYY...",
            "status": "completed",
            "result": {"ok": true}
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn execute_agent(
    State(state): State<GatewayState>,
    Path(agent_id): Path<String>,
    Json(_request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "agent_id": agent_id,
        "execution_id": uuid::Uuid::new_v4().to_string(),
        "status": "completed",
        "result": "Agent execution successful"
    }))
}

// AI Governance Endpoints (Mock implementations)

#[utoipa::path(
    get,
    path = "/api/v1/governance/policies",
    tag = "governance",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Liste des politiques", body = PoliciesResponse, example = json!({
            "policies": [
                {"id": "ethical-guidelines", "name": "Ethical AI Guidelines", "status": "active"},
                {"id": "privacy-protection", "name": "Privacy Protection Policy", "status": "active"}
            ]
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn list_policies(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "policies": [
            {
                "id": "ethical-guidelines",
                "name": "Ethical AI Guidelines",
                "status": "active"
            },
            {
                "id": "privacy-protection",
                "name": "Privacy Protection Policy",
                "status": "active"
            }
        ]
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/governance/audit",
    tag = "governance",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Journaux d'audit", body = AuditLogsResponse, example = json!({
            "logs": [],
            "total": 0
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn get_audit_logs(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "logs": [],
        "total": 0
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/governance/compliance",
    tag = "governance",
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Statut de conformité", body = ComplianceStatus, example = json!({
            "compliance_score": 0.95,
            "status": "compliant",
            "last_check": "2025-08-08T13:30:00Z"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 429, description = "Rate limited", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse),
    )
)]
async fn check_compliance(State(state): State<GatewayState>) -> Json<serde_json::Value> {
    state.metrics.increment_total();
    state.metrics.increment_success();
    
    Json(serde_json::json!({
        "compliance_score": 0.95,
        "status": "compliant",
        "last_check": chrono::Utc::now()
    }))
}

// Request/Response Types

/// Documentation OpenAPI générée automatiquement (placée après les schémas)
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_metrics,
        process_consciousness,
        get_consciousness_state,
        generate_reflection,
        update_agent,
        delete_agent,
        execute_agent,
        list_policies,
        get_audit_logs,
        check_compliance,
        sse_events,
        mock_stream,
        llm_generate,
        llm_chat,
        llm_stream,
        vision_describe,
        asr_transcribe,
        tts_synthesize,
    ),
    tags(
        (name = "health", description = "Endpoints de vivacité et statut du service API Gateway"),
        (name = "metrics", description = "Métriques runtime de l'API Gateway"),
        (name = "consciousness", description = "Routes d'accès au Consciousness Engine: traitement, état, introspection"),
        (name = "orchestration", description = "Gestion et exécution des agents (création, mise à jour, exécution)"),
        (name = "governance", description = "Politiques, audit et conformité (AI governance)"),
        (name = "system", description = "Événements système et flux SSE"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

/// Ajout dynamique du schéma de sécurité Bearer JWT et d'une sécurité globale
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());
        components.security_schemes.insert(
            "bearerAuth".to_string(),
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );

        // Sécurité globale: Bearer requis par défaut
        openapi.security = Some(vec![SecurityRequirement::new("bearerAuth", Vec::<String>::new())]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_contains_core_schemas() {
        let doc = ApiDoc::openapi();
        let spec = serde_json::to_value(&doc).expect("serialize openapi");
        let schemas = spec
            .get("components")
            .and_then(|c| c.get("schemas"))
            .cloned()
            .unwrap_or_default();

        // Vérifie quelques schémas clés (les noms doivent correspondre aux types)
        for key in [
            "ConsciousnessRequest",
            "ErrorResponse",
            "ConsciousnessState",
            "AgentSummary",
        ] {
            assert!(schemas.get(key).is_some(), "schema manquant: {}", key);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Configuration
    let config = GatewayConfig {
        port: std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000),
        consciousness_engine_url: std::env::var("CONSCIOUSNESS_ENGINE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        agent_orchestrator_url: std::env::var("AGENT_ORCHESTRATOR_URL")
            .unwrap_or_else(|_| "http://localhost:8081".to_string()),
        ai_governance_url: std::env::var("AI_GOVERNANCE_URL")
            .unwrap_or_else(|_| "http://localhost:8082".to_string()),
        jwt_secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-key".to_string()),
        rate_limit_requests_per_minute: std::env::var("RATE_LIMIT_PER_MINUTE")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(100),
        request_timeout_seconds: 30,
        auth_optional: std::env::var("AUTH_OPTIONAL")
            .ok()
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true),
    };
    
    // Create shared state
    let state = GatewayState {
        config: config.clone(),
        http_client: reqwest::Client::new(),
        rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
        metrics: Arc::new(GatewayMetrics::default()),
        event_tx: {
            let (tx, _rx) = broadcast::channel(100);
            tx
        },
    };
    
    // Create router
    let app = create_gateway_router(state);
    
    // Start server
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    
    println!("🌐 API Gateway starting on port {}", config.port);
    println!("📡 Health check: http://localhost:{}/health", config.port);
    println!("🧠 Consciousness API: http://localhost:{}/api/v1/consciousness/", config.port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}