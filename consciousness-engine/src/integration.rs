//! Integration module for external systems
//! 
//! This module handles integration with external systems, APIs, and services.

use crate::types::*;
use crate::error::ConsciousnessError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Integration manager for external systems
pub struct IntegrationManager {
    /// Registered integrations
    integrations: HashMap<String, Box<dyn Integration + Send + Sync>>,
    
    /// Configuration
    config: IntegrationConfig,
}

/// Configuration for integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable external integrations
    pub enabled: bool,
    
    /// Timeout for external calls
    pub timeout_seconds: u64,
    
    /// Retry attempts
    pub retry_attempts: u32,
    
    /// Rate limiting
    pub rate_limit_per_minute: u32,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 30,
            retry_attempts: 3,
            rate_limit_per_minute: 60,
        }
    }
}

/// Trait for external integrations
pub trait Integration {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn call(&self, request: IntegrationRequest) -> Result<IntegrationResponse, ConsciousnessError>;
}

/// Integration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRequest {
    pub method: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub context: Option<String>,
}

/// Integration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub success: bool,
    pub data: serde_json::Value,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl IntegrationManager {
    pub fn new() -> Self {
        Self {
            integrations: HashMap::new(),
            config: IntegrationConfig::default(),
        }
    }
    
    pub fn register_integration(&mut self, integration: Box<dyn Integration + Send + Sync>) {
        self.integrations.insert(integration.name().to_string(), integration);
    }
    
    pub async fn call_integration(&self, name: &str, request: IntegrationRequest) -> Result<IntegrationResponse, ConsciousnessError> {
        if !self.config.enabled {
            return Err(ConsciousnessError::ConfigurationError("Integrations disabled".to_string()));
        }
        
        let integration = self.integrations.get(name)
            .ok_or_else(|| ConsciousnessError::InvalidInput(format!("Integration {} not found", name)))?;
        
        if !integration.is_available() {
            return Err(ConsciousnessError::NetworkError(format!("Integration {} not available", name)));
        }
        
        integration.call(request)
    }
}

/// Mock integration for testing
pub struct MockIntegration {
    name: String,
    available: bool,
}

impl MockIntegration {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            available: true,
        }
    }
}

impl Integration for MockIntegration {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_available(&self) -> bool {
        self.available
    }
    
    fn call(&self, request: IntegrationRequest) -> Result<IntegrationResponse, ConsciousnessError> {
        Ok(IntegrationResponse {
            success: true,
            data: serde_json::json!({
                "method": request.method,
                "echo": request.parameters
            }),
            error: None,
            metadata: HashMap::new(),
        })
    }
}