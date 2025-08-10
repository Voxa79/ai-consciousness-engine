//! Regroupement des modèles exposés dans la documentation OpenAPI.
//! Toutes les structures de données documentées par OpenAPI via utoipa sont définies ici.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use serde_json::json;

/// Réponse d'état du service
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

/// Métriques du gateway
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct MetricsResponse {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub consciousness_requests: u64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "content": "Evaluate consciousness metrics for input...",
    "user_id": "user_123",
    "preferences": {"tone": "neutral"},
    "metadata": {"trace_id": "abc-123"}
}))]
pub struct ConsciousnessRequest {
    pub content: String,
    pub user_id: String,
    pub preferences: Option<HashMap<String, String>>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Résultat de traitement de conscience, incluant label, score et raisonnement.
#[derive(Debug, Serialize, ToSchema)]
pub struct ConsciousnessResult {
    /// Identifiant de corrélation de la requête
    pub request_id: String,
    /// Label interprété (ex: "self-aware", "non-self-aware")
    pub label: String,
    /// Score entre 0 et 1
    pub score: f32,
    /// Brève explication du modèle
    pub reasoning: String,
    /// Horodatage ISO8601
    pub created_at: String,
}

/// État courant du Consciousness Engine et ses métriques principales.
#[derive(Debug, Serialize, ToSchema)]
pub struct ConsciousnessState {
    /// Statut du moteur (ex: starting, ready, degraded)
    pub status: String,
    /// Dernière mise à jour ISO8601
    pub last_updated: String,
    /// Principales métriques runtime
    pub metrics: HashMap<String, f64>,
}

/// Résumé compact d'un agent géré par l'orchestrateur.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentSummary {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
}

/// Détails d'un agent, incluant configuration déclarative.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentDetail {
    pub id: String,
    pub status: String,
    #[schema(value_type = Object, example = json!({"type": "reactive"}))]
    pub config: serde_json::Value,
}

/// Réponse de liste d'agents avec pagination simple.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentListResponse {
    pub agents: Vec<AgentSummary>,
    pub total: u32,
}

/// Résultat d'une exécution d'agent, avec identifiants de corrélation et payload résultat.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentExecutionResult {
    pub agent_id: String,
    pub execution_id: String,
    pub status: String,
    #[schema(value_type = Object)]
    pub result: serde_json::Value,
}

/// Résumé d'une politique de gouvernance.
#[derive(Debug, Serialize, ToSchema)]
pub struct PolicySummary {
    pub id: String,
    pub name: String,
    pub status: String,
}

/// Collection de politiques retournées par le service de gouvernance.
#[derive(Debug, Serialize, ToSchema)]
pub struct PoliciesResponse {
    pub policies: Vec<PolicySummary>,
}

/// Réponse des journaux d'audit et total associé.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuditLogsResponse {
    #[schema(value_type = Vec<Object>)]
    pub logs: Vec<serde_json::Value>,
    pub total: u32,
}

/// Statut de conformité global et dernière date d'évaluation.
#[derive(Debug, Serialize, ToSchema)]
pub struct ComplianceStatus {
    pub compliance_score: f32,
    pub status: String,
    pub last_check: String,
}

/// Format standardisé pour les erreurs applicatives exposées par l'API Gateway.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
