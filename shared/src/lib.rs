//! Shared types and utilities for the Consciousness Engine API
//! 
//! This crate contains common types, traits, and utilities used across
//! all microservices in the Consciousness Engine platform.

pub mod types;
pub mod error;
pub mod config;
pub mod auth;
pub mod metrics;
pub mod tracing_setup;
pub mod database;
pub mod cache;
pub mod vault_client;

// Re-export commonly used types
pub use types::*;
pub use error::{Result, ConsciousnessError};
pub use auth::{Claims, AuthContext};
pub use metrics::MetricsRegistry;

/// Version information for the API
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SERVICE_NAME: &str = "consciousness-engine";

/// Common HTTP headers used across services
pub mod headers {
    pub const REQUEST_ID: &str = "x-request-id";
    pub const CORRELATION_ID: &str = "x-correlation-id";
    pub const USER_ID: &str = "x-user-id";
    pub const CONSCIOUSNESS_SCORE: &str = "x-consciousness-score";
    pub const PROCESSING_TIME: &str = "x-processing-time-ms";
}

/// Common HTTP status codes for consciousness-specific responses
pub mod status_codes {
    use axum::http::StatusCode;
    
    /// Consciousness processing failed due to ethical constraints
    pub const ETHICAL_VIOLATION: StatusCode = StatusCode::UNPROCESSABLE_ENTITY;
    
    /// Consciousness quality below threshold
    pub const QUALITY_INSUFFICIENT: StatusCode = StatusCode::NOT_ACCEPTABLE;
    
    /// Rate limit exceeded for consciousness processing
    pub const CONSCIOUSNESS_RATE_LIMITED: StatusCode = StatusCode::TOO_MANY_REQUESTS;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_service_name() {
        assert_eq!(SERVICE_NAME, "consciousness-engine");
    }
}