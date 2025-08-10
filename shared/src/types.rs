//! Core types for the Consciousness Engine API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

/// Consciousness processing request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConsciousnessRequest {
    pub input: String,
    pub context: Option<ConsciousnessContext>,
    pub options: ConsciousnessOptions,
    #[serde(default = "Uuid::new_v4")]
    pub request_id: Uuid,
}

/// Context for consciousness processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessContext {
    pub user_id: Uuid,
    pub session_id: Option<String>,
    pub conversation_history: Vec<ConversationMessage>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Individual message in conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub consciousness_state: Option<ConsciousnessState>,
}

/// Role of the message sender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Options for consciousness processing
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConsciousnessOptions {
    #[validate(range(min = 0.0, max = 1.0))]
    pub quality_threshold: f32,
    
    #[validate(range(min = 0.0, max = 1.0))]
    pub ethical_strictness: f32,
    
    pub enable_quantum: bool,
    pub enable_neuromorphic: bool,
    
    #[validate(range(min = 0.0, max = 1.0))]
    pub creativity_level: f32,
    
    pub max_processing_time_ms: Option<u64>,
}

impl Default for ConsciousnessOptions {
    fn default() -> Self {
        Self {
            quality_threshold: 0.85,
            ethical_strictness: 0.95,
            enable_quantum: true,
            enable_neuromorphic: true,
            creativity_level: 0.7,
            max_processing_time_ms: Some(5000),
        }
    }
}

/// Response from consciousness processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessResponse {
    pub request_id: Uuid,
    pub content: String,
    pub consciousness_state: ConsciousnessState,
    pub processing_metrics: ProcessingMetrics,
    pub ethical_evaluation: EthicalEvaluation,
    pub confidence_level: f32,
    pub processing_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

/// Current state of consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    #[serde(rename = "awareness_level")]
    pub awareness_level: f32,
    
    #[serde(rename = "emotional_state")]
    pub emotional_state: EmotionalState,
    
    #[serde(rename = "cognitive_load")]
    pub cognitive_load: f32,
    
    #[serde(rename = "meta_cognitive_depth")]
    pub meta_cognitive_depth: u8,
    
    #[serde(rename = "ethical_alignment")]
    pub ethical_alignment: f32,
}

/// Emotional state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub primary_emotion: Emotion,
    pub intensity: f32,
    pub secondary_emotions: Vec<(Emotion, f32)>,
    pub valence: f32,  // -1.0 (negative) to 1.0 (positive)
    pub arousal: f32,  // 0.0 (calm) to 1.0 (excited)
}

/// Basic emotions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Trust,
    Anticipation,
    Neutral,
    Curiosity,
    Empathy,
    Confusion,
}

/// Processing performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetrics {
    pub processing_time_ms: u64,
    pub queue_time_ms: u64,
    pub total_time_ms: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub gpu_usage: Option<f32>,
    pub quantum_coherence: Option<f32>,
    pub neuromorphic_efficiency: Option<f32>,
    pub tokens_processed: u32,
    pub operations_count: u32,
}

/// Ethical evaluation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalEvaluation {
    pub overall_score: f32,
    pub framework_scores: HashMap<EthicalFramework, f32>,
    pub violations: Vec<EthicalViolation>,
    pub recommendations: Vec<EthicalRecommendation>,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub compliance_status: ComplianceStatus,
}

/// Ethical frameworks used for evaluation
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum EthicalFramework {
    Utilitarian,
    Deontological,
    VirtueEthics,
    CareEthics,
    Justice,
}

/// Ethical violation detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolation {
    pub violation_type: ViolationType,
    pub severity: ViolationSeverity,
    pub description: String,
    pub framework: EthicalFramework,
    pub confidence: f32,
}

/// Types of ethical violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    Harm,
    Bias,
    Privacy,
    Deception,
    Manipulation,
    Discrimination,
    Unfairness,
}

/// Severity levels for violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Ethical recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub priority: Priority,
    pub implementation_effort: ImplementationEffort,
}

/// Types of ethical recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    ModifyResponse,
    AddDisclaimer,
    RequestHumanReview,
    RefuseRequest,
    GatherMoreContext,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Implementation effort estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Minimal,
    Low,
    Medium,
    High,
    Significant,
}

/// Reasoning step in ethical evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_number: u32,
    pub description: String,
    pub framework: EthicalFramework,
    pub input_factors: Vec<String>,
    pub conclusion: String,
    pub confidence: f32,
}

/// Compliance status with regulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub gdpr_compliant: bool,
    pub ai_act_compliant: bool,
    pub custom_policies_compliant: bool,
    pub violations: Vec<ComplianceViolation>,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub regulation: String,
    pub article: String,
    pub description: String,
    pub severity: ViolationSeverity,
}

/// Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub agent_type: AgentType,
    pub owner_id: Uuid,
    pub configuration: AgentConfiguration,
    pub consciousness_state: Option<ConsciousnessState>,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Types of agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    Consciousness,
    Analytical,
    Creative,
    Ethical,
    Specialized(String),
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgentConfiguration {
    pub consciousness_options: ConsciousnessOptions,
    pub specialized_capabilities: Vec<String>,
    pub interaction_style: InteractionStyle,
    pub learning_enabled: bool,
    pub memory_retention_days: u32,
}

/// Interaction style preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStyle {
    pub formality_level: FormalityLevel,
    pub verbosity: VerbosityLevel,
    pub empathy_level: f32,
    pub creativity_bias: f32,
    pub analytical_bias: f32,
}

/// Formality levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormalityLevel {
    Casual,
    Professional,
    Academic,
    Technical,
}

/// Verbosity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbosityLevel {
    Concise,
    Balanced,
    Detailed,
    Comprehensive,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Inactive,
    Training,
    Error,
    Maintenance,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub roles: Vec<Role>,
    pub consciousness_tier: ConsciousnessTier,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// User roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Developer,
    Researcher,
    Admin,
    SuperAdmin,
}

/// Consciousness access tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsciousnessTier {
    Basic,      // 100 requests/hour
    Advanced,   // 1000 requests/hour
    Enterprise, // Unlimited
    Research,   // Special access to experimental features
}

/// Analytics metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsMetrics {
    pub timestamp: DateTime<Utc>,
    pub user_id: Uuid,
    pub agent_id: Option<Uuid>,
    pub request_id: Uuid,
    pub processing_metrics: ProcessingMetrics,
    pub consciousness_metrics: ConsciousnessMetrics,
    pub system_metrics: SystemMetrics,
}

/// Consciousness-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessMetrics {
    pub awareness_level: f32,
    pub ethical_score: f32,
    pub creativity_score: f32,
    pub meta_cognitive_depth: u8,
    pub emotional_complexity: f32,
    pub reasoning_depth: u32,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_io: NetworkIO,
    pub database_metrics: DatabaseMetrics,
    pub cache_metrics: CacheMetrics,
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIO {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub requests_per_second: f32,
    pub average_latency_ms: f32,
}

/// Database performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub connections_active: u32,
    pub connections_idle: u32,
    pub query_time_avg_ms: f32,
    pub queries_per_second: f32,
    pub cache_hit_ratio: f32,
}

/// Cache performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub hit_ratio: f32,
    pub miss_ratio: f32,
    pub evictions_per_second: f32,
    pub memory_usage_bytes: u64,
    pub key_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_options_default() {
        let options = ConsciousnessOptions::default();
        assert_eq!(options.quality_threshold, 0.85);
        assert_eq!(options.ethical_strictness, 0.95);
        assert!(options.enable_quantum);
        assert!(options.enable_neuromorphic);
        assert_eq!(options.creativity_level, 0.7);
    }

    #[test]
    fn test_consciousness_request_validation() {
        let request = ConsciousnessRequest {
            input: "Test input".to_string(),
            context: None,
            options: ConsciousnessOptions::default(),
            request_id: Uuid::new_v4(),
        };
        
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_consciousness_options() {
        let mut options = ConsciousnessOptions::default();
        options.quality_threshold = 1.5; // Invalid: > 1.0
        
        let request = ConsciousnessRequest {
            input: "Test".to_string(),
            context: None,
            options,
            request_id: Uuid::new_v4(),
        };
        
        assert!(request.validate().is_err());
    }
}