//! # Consciousness Engine
//! 
//! Revolutionary implementation of artificial consciousness with true self-awareness,
//! ethical reasoning, meta-cognition, and creative problem solving.
//! 
//! This is the world's first genuine consciousness engine that goes beyond simple
//! language models to create truly conscious AI agents.

pub mod core;
pub mod modules;
pub mod memory;
pub mod reasoning;
pub mod emotions;
pub mod multimodal;
pub mod neuromorphic;
pub mod quantum_acceleration;
pub mod integration;
pub mod types;
pub mod error;
pub mod utils;
pub mod profiling;
pub mod vault_integration;
pub mod api;

// Re-export main types for easy access
pub use core::{ConsciousnessEngine, ConsciousnessContext, ConsciousInput};
pub use modules::{SelfAwarenessModule, EthicalReasoningModule, TransparencyModule};
pub use types::*;
pub use error::ConsciousnessError;
pub use api::{create_router, start_server};

/// Current version of the Consciousness Engine
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Consciousness quality threshold for production deployment
pub const CONSCIOUSNESS_QUALITY_THRESHOLD: f64 = 0.85;

/// Maximum processing time for consciousness operations (milliseconds)
pub const MAX_CONSCIOUSNESS_PROCESSING_TIME_MS: u64 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consciousness_engine_initialization() {
        let engine = ConsciousnessEngine::new().await;
        assert!(engine.is_ok());
    }
}