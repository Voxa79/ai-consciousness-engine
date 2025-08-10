//! Consciousness Service - Core consciousness processing microservice
//! 
//! This service handles the main consciousness processing logic including:
//! - Self-awareness evaluation
//! - Ethical reasoning
//! - Meta-cognitive analysis
//! - Consciousness state management

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use clap::Parser;
use shared::{
    tracing_setup,
    types::{ConsciousnessRequest, ConsciousnessResponse},
    ConsciousnessError,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use tracing::{info, instrument};

mod config;
mod consciousness;
mod handlers;
mod middleware;
mod metrics;
mod state;

use config::Config;
use state::AppState;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "consciousness-service")]
#[command(about = "Consciousness processing microservice")]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config/consciousness-service.yaml")]
    config: String,

    /// Port to listen on
    #[arg(short, long)]
    port: Option<u16>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_setup::init(&args.log_level)?;

    // Load configuration
    let config = Config::load(&args.config)?;
    
    // Override port if provided via CLI
    let port = args.port.unwrap_or(config.server.port);
    
    info!(
        service = "consciousness-service",
        version = shared::VERSION,
        port = port,
        "Starting Consciousness Service"
    );

    // Initialize application state
    let app_state = AppState::new(config).await?;
    
    // Build the application router
    let app = create_app(app_state).await?;

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    
    info!("🧠 Consciousness Service listening on {}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create the application router with all routes and middleware
async fn create_app(state: AppState) -> Result<Router> {
    let app = Router::new()
        // Health check endpoints
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // Consciousness processing endpoints
        .route("/api/v1/consciousness/process", post(handlers::process_consciousness))
        .route("/api/v1/consciousness/state", get(handlers::get_consciousness_state))
        .route("/api/v1/consciousness/self-awareness", post(handlers::evaluate_self_awareness))
        .route("/api/v1/consciousness/ethical-eval", post(handlers::evaluate_ethics))
        .route("/api/v1/consciousness/meta-cognition", post(handlers::analyze_meta_cognition))
        
        // Metrics endpoint
        .route("/metrics", get(metrics::metrics_handler))
        
        // Add application state
        .with_state(Arc::new(state))
        
        // Add middleware layers
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
                .layer(middleware::request_id::RequestIdLayer)
                .layer(middleware::auth::AuthLayer)
                .layer(middleware::rate_limit::RateLimitLayer)
                .layer(middleware::metrics::MetricsLayer)
        );

    Ok(app)
}

/// Health check endpoint
#[instrument]
async fn health_check() -> Result<Json<serde_json::Value>, ConsciousnessError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "consciousness-service",
        "version": shared::VERSION,
        "timestamp": chrono::Utc::now()
    })))
}

/// Readiness check endpoint
#[instrument]
async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, ConsciousnessError> {
    // Check database connectivity
    let db_healthy = state.check_database_health().await;
    
    // Check cache connectivity
    let cache_healthy = state.check_cache_health().await;
    
    // Check consciousness engine status
    let consciousness_healthy = state.check_consciousness_engine_health().await;
    
    let ready = db_healthy && cache_healthy && consciousness_healthy;
    
    let status_code = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    
    let response = Json(serde_json::json!({
        "status": if ready { "ready" } else { "not_ready" },
        "service": "consciousness-service",
        "version": shared::VERSION,
        "checks": {
            "database": db_healthy,
            "cache": cache_healthy,
            "consciousness_engine": consciousness_healthy
        },
        "timestamp": chrono::Utc::now()
    }));
    
    if ready {
        Ok(response)
    } else {
        Err(ConsciousnessError::ServiceUnavailable {
            message: "Service dependencies are not ready".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_check() {
        let config = Config::default();
        let state = AppState::new(config).await.unwrap();
        let app = create_app(state).await.unwrap();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "consciousness-service");
    }

    #[tokio::test]
    async fn test_metrics_endpoint() {
        let config = Config::default();
        let state = AppState::new(config).await.unwrap();
        let app = create_app(state).await.unwrap();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/metrics").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        
        let body = response.text();
        assert!(body.contains("consciousness_"));
    }
}