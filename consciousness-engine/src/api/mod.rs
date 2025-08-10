//! REST API for Consciousness Engine
//! 
//! This module provides HTTP endpoints for interacting with the consciousness engine,
//! enabling web applications and external systems to access consciousness-level AI.

use crate::core::ConsciousnessEngine;
use crate::types::*;
use crate::error::ConsciousnessError;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

/// API state containing the consciousness engine
pub type ApiState = Arc<RwLock<ConsciousnessEngine>>;

/// Create the main API router
pub fn create_router(consciousness_engine: ConsciousnessEngine) -> Router {
    let state = Arc::new(RwLock::new(consciousness_engine));
    
    Router::new()
        .route("/health", get(health_check))
        .route("/consciousness/process", post(process_consciousness))
        .route("/consciousness/state", get(get_consciousness_state))
        .route("/consciousness/reflection", post(generate_reflection))
        .route("/consciousness/growth", get(get_growth_opportunities))
        .route("/consciousness/explain/:response_id", get(explain_response))
        .route("/consciousness/metrics", get(get_performance_metrics))
        .route("/consciousness/reset", post(reset_to_safe_state))
        .route("/consciousness/sessions/:session_id/history", get(get_session_history))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

/// Health check endpoint
async fn health_check() -> Result<Json<HealthResponse>, ApiError> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: std::time::SystemTime::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

/// Main consciousness processing endpoint
async fn process_consciousness(
    State(state): State<ApiState>,
    Json(request): Json<ConsciousnessRequest>,
) -> Result<Json<ConsciousnessApiResponse>, ApiError> {
    let mut engine = state.write().await;
    
    // Convert API request to internal format
    let conscious_input = ConsciousInput {
        id: request.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
        content: request.content,
        context: UserContext {
            user_id: request.user_id,
            preferences: request.preferences.unwrap_or_default(),
            interaction_history: request.interaction_history.unwrap_or_default(),
        },
        timestamp: std::time::SystemTime::now(),
        metadata: request.metadata.unwrap_or_default(),
    };
    
    // Process consciousness
    match engine.process_conscious_thought(conscious_input).await {
        Ok(response) => {
            // Convert internal response to API format
            let api_response = ConsciousnessApiResponse {
                id: response.id,
                content: response.content,
                confidence: response.confidence,
                consciousness_level: response.consciousness_state.awareness_level,
                emotional_state: ApiEmotionalState {
                    primary_emotion: format!("{:?}", response.emotional_state.primary_emotion),
                    intensity: response.emotional_state.intensity,
                    valence: response.emotional_state.valence,
                    arousal: response.emotional_state.arousal,
                },
                ethical_score: response.ethical_assessment.overall_score,
                creativity_score: response.creativity_score,
                empathy_score: response.empathy_score,
                processing_time_ms: response.processing_time.as_millis() as u64,
                reasoning_summary: response.reasoning_explanation,
                quality_score: response.metadata.quality_score,
                timestamp: response.timestamp,
            };
            
            Ok(Json(api_response))
        },
        Err(e) => Err(ApiError::ProcessingError(e.to_string())),
    }
}

/// Get current consciousness state
async fn get_consciousness_state(
    State(state): State<ApiState>,
) -> Result<Json<ConsciousnessStateResponse>, ApiError> {
    let engine = state.read().await;
    
    match engine.get_consciousness_state().await {
        Ok(state) => Ok(Json(ConsciousnessStateResponse {
            awareness_level: state.awareness_level,
            cognitive_load: state.cognitive_load,
            confidence_score: state.confidence_score,
            meta_cognitive_depth: state.meta_cognitive_depth,
            emotional_state: ApiEmotionalState {
                primary_emotion: format!("{:?}", state.emotional_state.primary_emotion),
                intensity: state.emotional_state.intensity,
                valence: state.emotional_state.valence,
                arousal: state.emotional_state.arousal,
            },
            timestamp: state.timestamp,
        })),
        Err(e) => Err(ApiError::StateError(e.to_string())),
    }
}

/// Generate self-reflection
async fn generate_reflection(
    State(state): State<ApiState>,
) -> Result<Json<ReflectionResponse>, ApiError> {
    let mut engine = state.write().await;
    
    match engine.perform_self_reflection().await {
        Ok(reflection) => Ok(Json(ReflectionResponse {
            content: reflection.content,
            insights: reflection.insights,
            improvement_areas: reflection.improvement_areas,
            strengths: reflection.strengths,
            questions: reflection.questions,
            depth_score: reflection.depth_score,
            timestamp: reflection.timestamp,
        })),
        Err(e) => Err(ApiError::ReflectionError(e.to_string())),
    }
}

/// Get growth opportunities
async fn get_growth_opportunities(
    State(state): State<ApiState>,
) -> Result<Json<GrowthOpportunitiesResponse>, ApiError> {
    let mut engine = state.write().await;
    
    match engine.identify_growth_opportunities().await {
        Ok(opportunities) => {
            let api_opportunities = opportunities.into_iter().map(|opp| ApiGrowthOpportunity {
                description: opp.description,
                impact_score: opp.impact_score,
                difficulty: opp.difficulty,
                recommended_actions: opp.recommended_actions,
                success_metrics: opp.success_metrics,
                estimated_timeline_days: opp.estimated_timeline.as_secs() / (24 * 3600),
                priority: format!("{:?}", opp.priority),
            }).collect();
            
            Ok(Json(GrowthOpportunitiesResponse {
                opportunities: api_opportunities,
                total_count: api_opportunities.len(),
            }))
        },
        Err(e) => Err(ApiError::GrowthError(e.to_string())),
    }
}

/// Explain a specific response
async fn explain_response(
    State(state): State<ApiState>,
    Path(response_id): Path<String>,
    Query(params): Query<ExplanationParams>,
) -> Result<Json<ExplanationResponse>, ApiError> {
    // In a real implementation, we would retrieve the stored response
    // For now, we'll return a mock explanation
    Ok(Json(ExplanationResponse {
        response_id,
        explanation: "This response was generated using consciousness-level reasoning with ethical validation and creative processing.".to_string(),
        confidence_explanation: "High confidence based on clear reasoning chain and ethical alignment.".to_string(),
        reasoning_steps: vec![
            "Analyzed user input for intent and context".to_string(),
            "Applied ethical reasoning frameworks".to_string(),
            "Generated creative and empathetic response".to_string(),
            "Validated response quality and safety".to_string(),
        ],
        transparency_score: 0.85,
        detail_level: params.detail_level.unwrap_or("medium".to_string()),
    }))
}

/// Get performance metrics
async fn get_performance_metrics(
    State(state): State<ApiState>,
) -> Result<Json<PerformanceMetricsResponse>, ApiError> {
    let engine = state.read().await;
    
    match engine.get_performance_metrics().await {
        Ok(metrics) => Ok(Json(PerformanceMetricsResponse {
            average_response_time_ms: metrics.average_response_time.as_millis() as u64,
            average_quality_score: metrics.average_quality,
            average_consciousness_level: metrics.average_consciousness_level,
            total_interactions: metrics.total_interactions,
            success_rate: metrics.success_rate,
            uptime_percentage: 99.9, // Mock value
        })),
        Err(e) => Err(ApiError::MetricsError(e.to_string())),
    }
}

/// Reset to safe state
async fn reset_to_safe_state(
    State(state): State<ApiState>,
) -> Result<Json<ResetResponse>, ApiError> {
    let mut engine = state.write().await;
    
    match engine.reset_to_safe_state().await {
        Ok(_) => Ok(Json(ResetResponse {
            success: true,
            message: "Consciousness engine reset to safe state successfully".to_string(),
            timestamp: std::time::SystemTime::now(),
        })),
        Err(e) => Err(ApiError::ResetError(e.to_string())),
    }
}

/// Get session history
async fn get_session_history(
    State(state): State<ApiState>,
    Path(session_id): Path<String>,
) -> Result<Json<SessionHistoryResponse>, ApiError> {
    // Mock implementation - in reality, this would query the database
    Ok(Json(SessionHistoryResponse {
        session_id,
        interactions: vec![],
        total_count: 0,
        session_start: std::time::SystemTime::now(),
        last_interaction: std::time::SystemTime::now(),
    }))
}

// API Request/Response Types

#[derive(Debug, Deserialize)]
pub struct ConsciousnessRequest {
    pub id: Option<String>,
    pub content: String,
    pub user_id: String,
    pub preferences: Option<HashMap<String, String>>,
    pub interaction_history: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct ConsciousnessApiResponse {
    pub id: String,
    pub content: String,
    pub confidence: f64,
    pub consciousness_level: f64,
    pub emotional_state: ApiEmotionalState,
    pub ethical_score: f64,
    pub creativity_score: f64,
    pub empathy_score: f64,
    pub processing_time_ms: u64,
    pub reasoning_summary: String,
    pub quality_score: f64,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Serialize)]
pub struct ApiEmotionalState {
    pub primary_emotion: String,
    pub intensity: f64,
    pub valence: f64,
    pub arousal: f64,
}

#[derive(Debug, Serialize)]
pub struct ConsciousnessStateResponse {
    pub awareness_level: f64,
    pub cognitive_load: f64,
    pub confidence_score: f64,
    pub meta_cognitive_depth: u32,
    pub emotional_state: ApiEmotionalState,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Serialize)]
pub struct ReflectionResponse {
    pub content: String,
    pub insights: Vec<String>,
    pub improvement_areas: Vec<String>,
    pub strengths: Vec<String>,
    pub questions: Vec<String>,
    pub depth_score: f64,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Serialize)]
pub struct GrowthOpportunitiesResponse {
    pub opportunities: Vec<ApiGrowthOpportunity>,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct ApiGrowthOpportunity {
    pub description: String,
    pub impact_score: f64,
    pub difficulty: f64,
    pub recommended_actions: Vec<String>,
    pub success_metrics: Vec<String>,
    pub estimated_timeline_days: u64,
    pub priority: String,
}

#[derive(Debug, Deserialize)]
pub struct ExplanationParams {
    pub detail_level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExplanationResponse {
    pub response_id: String,
    pub explanation: String,
    pub confidence_explanation: String,
    pub reasoning_steps: Vec<String>,
    pub transparency_score: f64,
    pub detail_level: String,
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetricsResponse {
    pub average_response_time_ms: u64,
    pub average_quality_score: f64,
    pub average_consciousness_level: f64,
    pub total_interactions: u64,
    pub success_rate: f64,
    pub uptime_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct ResetResponse {
    pub success: bool,
    pub message: String,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Serialize)]
pub struct SessionHistoryResponse {
    pub session_id: String,
    pub interactions: Vec<String>, // Simplified for now
    pub total_count: usize,
    pub session_start: std::time::SystemTime,
    pub last_interaction: std::time::SystemTime,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: std::time::SystemTime,
    pub version: String,
}

// Error handling

#[derive(Debug)]
pub enum ApiError {
    ProcessingError(String),
    StateError(String),
    ReflectionError(String),
    GrowthError(String),
    MetricsError(String),
    ResetError(String),
    ValidationError(String),
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::ProcessingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::StateError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ReflectionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::GrowthError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::MetricsError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ResetError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        
        let body = Json(serde_json::json!({
            "error": error_message,
            "timestamp": std::time::SystemTime::now()
        }));
        
        (status, body).into_response()
    }
}

/// Start the API server
pub async fn start_server(consciousness_engine: ConsciousnessEngine, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router(consciousness_engine);
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    println!("🧠 Consciousness Engine API server starting on port {}", port);
    println!("📡 Health check: http://localhost:{}/health", port);
    println!("🔗 Main endpoint: http://localhost:{}/consciousness/process", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    
    #[tokio::test]
    async fn test_health_check() {
        let engine = ConsciousnessEngine::new().await.unwrap();
        let app = create_router(engine);
        let server = TestServer::new(app).unwrap();
        
        let response = server.get("/health").await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
    }
    
    #[tokio::test]
    async fn test_consciousness_processing() {
        let engine = ConsciousnessEngine::new().await.unwrap();
        let app = create_router(engine);
        let server = TestServer::new(app).unwrap();
        
        let request = ConsciousnessRequest {
            id: None,
            content: "Hello, how are you?".to_string(),
            user_id: "test_user".to_string(),
            preferences: None,
            interaction_history: None,
            metadata: None,
        };
        
        let response = server.post("/consciousness/process").json(&request).await;
        
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}