//! Error handling for the Consciousness Engine API

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Result type alias for the Consciousness Engine
pub type Result<T> = std::result::Result<T, ConsciousnessError>;

/// Main error type for the Consciousness Engine API
#[derive(Debug, Error)]
pub enum ConsciousnessError {
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },

    #[error("Authorization denied: {message}")]
    AuthorizationDenied { message: String },

    #[error("Consciousness processing failed: {message}")]
    ConsciousnessProcessingFailed { message: String },

    #[error("Ethical violation detected: {violation_type:?} - {message}")]
    EthicalViolation {
        violation_type: crate::types::ViolationType,
        message: String,
        severity: crate::types::ViolationSeverity,
    },

    #[error("Quality threshold not met: expected {threshold}, got {actual}")]
    QualityThresholdNotMet { threshold: f32, actual: f32 },

    #[error("Rate limit exceeded: {limit} requests per {window}")]
    RateLimitExceeded { limit: u32, window: String },

    #[error("Validation error: {errors:?}")]
    ValidationError { errors: HashMap<String, Vec<String>> },

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Cache error: {message}")]
    CacheError { message: String },

    #[error("External service error: {service} - {message}")]
    ExternalServiceError { service: String, message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Timeout error: operation timed out after {timeout_ms}ms")]
    TimeoutError { timeout_ms: u64 },

    #[error("Resource not found: {resource_type} with id {id}")]
    ResourceNotFound { resource_type: String, id: String },

    #[error("Resource conflict: {message}")]
    ResourceConflict { message: String },

    #[error("Internal server error: {message}")]
    InternalServerError { message: String },

    #[error("Service unavailable: {message}")]
    ServiceUnavailable { message: String },

    #[error("Bad request: {message}")]
    BadRequest { message: String },

    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation { operation: String },
}

impl ConsciousnessError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::AuthenticationFailed { .. } => StatusCode::UNAUTHORIZED,
            Self::AuthorizationDenied { .. } => StatusCode::FORBIDDEN,
            Self::EthicalViolation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::QualityThresholdNotMet { .. } => StatusCode::NOT_ACCEPTABLE,
            Self::RateLimitExceeded { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::ValidationError { .. } => StatusCode::BAD_REQUEST,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::ResourceNotFound { .. } => StatusCode::NOT_FOUND,
            Self::ResourceConflict { .. } => StatusCode::CONFLICT,
            Self::TimeoutError { .. } => StatusCode::REQUEST_TIMEOUT,
            Self::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Self::UnsupportedOperation { .. } => StatusCode::NOT_IMPLEMENTED,
            Self::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CacheError { .. } => StatusCode::SERVICE_UNAVAILABLE,
            Self::ExternalServiceError { .. } => StatusCode::BAD_GATEWAY,
            Self::ConfigurationError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConsciousnessProcessingFailed { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get the error code for this error (for API responses)
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed { .. } => "AUTHENTICATION_FAILED",
            Self::AuthorizationDenied { .. } => "AUTHORIZATION_DENIED",
            Self::EthicalViolation { .. } => "ETHICAL_VIOLATION",
            Self::QualityThresholdNotMet { .. } => "QUALITY_THRESHOLD_NOT_MET",
            Self::RateLimitExceeded { .. } => "RATE_LIMIT_EXCEEDED",
            Self::ValidationError { .. } => "VALIDATION_ERROR",
            Self::BadRequest { .. } => "BAD_REQUEST",
            Self::ResourceNotFound { .. } => "RESOURCE_NOT_FOUND",
            Self::ResourceConflict { .. } => "RESOURCE_CONFLICT",
            Self::TimeoutError { .. } => "TIMEOUT_ERROR",
            Self::ServiceUnavailable { .. } => "SERVICE_UNAVAILABLE",
            Self::UnsupportedOperation { .. } => "UNSUPPORTED_OPERATION",
            Self::DatabaseError { .. } => "DATABASE_ERROR",
            Self::CacheError { .. } => "CACHE_ERROR",
            Self::ExternalServiceError { .. } => "EXTERNAL_SERVICE_ERROR",
            Self::ConfigurationError { .. } => "CONFIGURATION_ERROR",
            Self::ConsciousnessProcessingFailed { .. } => "CONSCIOUSNESS_PROCESSING_FAILED",
            Self::InternalServerError { .. } => "INTERNAL_SERVER_ERROR",
        }
    }

    /// Check if this error should be retried
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::TimeoutError { .. }
                | Self::ServiceUnavailable { .. }
                | Self::ExternalServiceError { .. }
                | Self::CacheError { .. }
        )
    }

    /// Check if this error should be logged as an error (vs warning/info)
    pub fn should_log_as_error(&self) -> bool {
        matches!(
            self,
            Self::InternalServerError { .. }
                | Self::DatabaseError { .. }
                | Self::ConfigurationError { .. }
                | Self::ConsciousnessProcessingFailed { .. }
        )
    }
}

/// Error response structure for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
    pub request_id: Option<Uuid>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Detailed error information
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub suggestions: Vec<String>,
}

impl IntoResponse for ConsciousnessError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        let (details, suggestions) = match &self {
            ConsciousnessError::ValidationError { errors } => {
                let details = serde_json::to_value(errors).unwrap_or_default();
                let suggestions = vec![
                    "Check the request format and ensure all required fields are provided".to_string(),
                    "Validate input parameters against the API documentation".to_string(),
                ];
                (Some(details), suggestions)
            }
            ConsciousnessError::EthicalViolation { violation_type, severity, .. } => {
                let details = serde_json::json!({
                    "violation_type": violation_type,
                    "severity": severity
                });
                let suggestions = vec![
                    "Review the request content for potential ethical issues".to_string(),
                    "Consider rephrasing the request to align with ethical guidelines".to_string(),
                    "Contact support if you believe this is a false positive".to_string(),
                ];
                (Some(details), suggestions)
            }
            ConsciousnessError::QualityThresholdNotMet { threshold, actual } => {
                let details = serde_json::json!({
                    "threshold": threshold,
                    "actual": actual
                });
                let suggestions = vec![
                    "Lower the quality threshold in your request options".to_string(),
                    "Provide more context or clearer input".to_string(),
                    "Try again as consciousness quality can vary".to_string(),
                ];
                (Some(details), suggestions)
            }
            ConsciousnessError::RateLimitExceeded { limit, window } => {
                let details = serde_json::json!({
                    "limit": limit,
                    "window": window
                });
                let suggestions = vec![
                    format!("Wait before making more requests (limit: {} per {})", limit, window),
                    "Consider upgrading your consciousness tier for higher limits".to_string(),
                    "Implement exponential backoff in your client".to_string(),
                ];
                (Some(details), suggestions)
            }
            ConsciousnessError::AuthenticationFailed { .. } => {
                (None, vec![
                    "Check your authentication credentials".to_string(),
                    "Ensure your API key is valid and not expired".to_string(),
                    "Verify you're using the correct authentication method".to_string(),
                ])
            }
            ConsciousnessError::AuthorizationDenied { .. } => {
                (None, vec![
                    "Check if you have the required permissions".to_string(),
                    "Contact your administrator to request access".to_string(),
                    "Verify you're accessing the correct resource".to_string(),
                ])
            }
            _ => (None, vec!["Contact support if the problem persists".to_string()]),
        };

        let error_response = ErrorResponse {
            error: ErrorDetails {
                code: error_code.to_string(),
                message,
                details,
                suggestions,
            },
            request_id: None, // Will be set by middleware
            timestamp: chrono::Utc::now(),
        };

        (status, Json(error_response)).into_response()
    }
}

// Conversion from common error types
impl From<sqlx::Error> for ConsciousnessError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ConsciousnessError::ResourceNotFound {
                resource_type: "database record".to_string(),
                id: "unknown".to_string(),
            },
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    ConsciousnessError::ResourceConflict {
                        message: "Resource already exists".to_string(),
                    }
                } else {
                    ConsciousnessError::DatabaseError {
                        message: db_err.to_string(),
                    }
                }
            }
            _ => ConsciousnessError::DatabaseError {
                message: err.to_string(),
            },
        }
    }
}

impl From<redis::RedisError> for ConsciousnessError {
    fn from(err: redis::RedisError) -> Self {
        ConsciousnessError::CacheError {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for ConsciousnessError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ConsciousnessError::TimeoutError {
                timeout_ms: 30000, // Default timeout
            }
        } else {
            ConsciousnessError::ExternalServiceError {
                service: "http_client".to_string(),
                message: err.to_string(),
            }
        }
    }
}

impl From<validator::ValidationErrors> for ConsciousnessError {
    fn from(err: validator::ValidationErrors) -> Self {
        let mut errors = HashMap::new();
        
        for (field, field_errors) in err.field_errors() {
            let messages: Vec<String> = field_errors
                .iter()
                .map(|e| e.message.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "Invalid value".to_string()))
                .collect();
            errors.insert(field.to_string(), messages);
        }
        
        ConsciousnessError::ValidationError { errors }
    }
}

impl From<tokio::time::error::Elapsed> for ConsciousnessError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        ConsciousnessError::TimeoutError {
            timeout_ms: 30000,
        }
    }
}

impl From<serde_json::Error> for ConsciousnessError {
    fn from(err: serde_json::Error) -> Self {
        ConsciousnessError::BadRequest {
            message: format!("JSON parsing error: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            ConsciousnessError::AuthenticationFailed {
                message: "test".to_string()
            }.status_code(),
            StatusCode::UNAUTHORIZED
        );

        assert_eq!(
            ConsciousnessError::EthicalViolation {
                violation_type: crate::types::ViolationType::Harm,
                message: "test".to_string(),
                severity: crate::types::ViolationSeverity::High,
            }.status_code(),
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(
            ConsciousnessError::AuthenticationFailed {
                message: "test".to_string()
            }.error_code(),
            "AUTHENTICATION_FAILED"
        );
    }

    #[test]
    fn test_retryable_errors() {
        assert!(ConsciousnessError::TimeoutError { timeout_ms: 1000 }.is_retryable());
        assert!(!ConsciousnessError::AuthenticationFailed {
            message: "test".to_string()
        }.is_retryable());
    }

    #[test]
    fn test_should_log_as_error() {
        assert!(ConsciousnessError::InternalServerError {
            message: "test".to_string()
        }.should_log_as_error());
        
        assert!(!ConsciousnessError::ValidationError {
            errors: HashMap::new()
        }.should_log_as_error());
    }
}