//! Consciousness Engine API Server
//! 
//! Main binary for running the consciousness engine as a standalone API server.

use consciousness_engine::{ConsciousnessEngine, start_server};
use std::env;
use tokio;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🧠 Starting Consciousness Engine API Server");
    
    // Get port from environment or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    // Initialize consciousness engine
    info!("🔧 Initializing Consciousness Engine...");
    let consciousness_engine = match ConsciousnessEngine::new().await {
        Ok(engine) => {
            info!("✅ Consciousness Engine initialized successfully");
            engine
        },
        Err(e) => {
            error!("❌ Failed to initialize Consciousness Engine: {}", e);
            return Err(Box::new(e));
        }
    };
    
    // Start the API server
    info!("🚀 Starting API server on port {}", port);
    if let Err(e) = start_server(consciousness_engine, port).await {
        error!("❌ Server error: {}", e);
        return Err(e);
    }
    
    Ok(())
}