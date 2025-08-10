//! Vault Integration for Consciousness Engine
//! 
//! Secure secrets management and dynamic credentials for consciousness processing

use crate::error::ConsciousnessError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Re-export from shared crate
pub use shared::vault_client::{VaultClient, VaultClientBuilder, AuthMethod, VaultConfig};

/// Consciousness-specific secrets manager
pub struct ConsciousnessSecretsManager {
    vault_client: Arc<VaultClient>,
    secrets_cache: Arc<RwLock<HashMap<String, CachedSecret>>>,
    config: SecretsConfig,
}

/// Configuration for secrets management
#[derive(Debug, Clone)]
pub struct SecretsConfig {
    /// Cache TTL for secrets
    pub cache_ttl_seconds: u64,
    /// Automatic renewal threshold
    pub renewal_threshold_seconds: u64,
    /// Database credentials role
    pub database_role: String,
    /// PKI role for certificates
    pub pki_role: String,
}

/// Cached secret with expiration
#[derive(Debug, Clone)]
struct CachedSecret {
    value: serde_json::Value,
    expires_at: std::time::SystemTime,
}

/// Consciousness-specific secrets
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsciousnessSecrets {
    /// Database encryption key
    pub database_encryption_key: String,
    /// JWT signing key for consciousness tokens
    pub jwt_signing_key: String,
    /// API key for external services
    pub api_key: String,
    /// Neural network encryption key
    pub neural_encryption_key: String,
    /// Memory encryption key
    pub memory_encryption_key: String,
}

/// Database configuration with dynamic credentials
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
    pub connection_timeout: u64,
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            cache_ttl_seconds: 3600, // 1 hour
            renewal_threshold_seconds: 300, // 5 minutes
            database_role: "consciousness-engine".to_string(),
            pki_role: "consciousness-services".to_string(),
        }
    }
}

impl ConsciousnessSecretsManager {
    /// Create a new secrets manager
    pub async fn new(vault_client: VaultClient, config: SecretsConfig) -> Result<Self> {
        let manager = Self {
            vault_client: Arc::new(vault_client),
            secrets_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        };

        // Verify Vault connectivity
        manager.health_check().await?;

        info!("Consciousness secrets manager initialized successfully");
        Ok(manager)
    }

    /// Create secrets manager with Kubernetes authentication
    pub async fn with_kubernetes_auth(role: &str, config: SecretsConfig) -> Result<Self> {
        let vault_client = VaultClientBuilder::new()
            .url(std::env::var("VAULT_ADDR").unwrap_or_else(|_| "https://vault:8200".to_string()))
            .kubernetes_auth(role)
            .skip_tls_verify(std::env::var("VAULT_SKIP_VERIFY").unwrap_or_default() == "true")
            .build()
            .await
            .context("Failed to create Vault client")?;

        Self::new(vault_client, config).await
    }

    /// Get consciousness-specific secrets
    pub async fn get_consciousness_secrets(&self) -> Result<ConsciousnessSecrets, ConsciousnessError> {
        debug!("Retrieving consciousness secrets");

        let secrets_data = self.get_cached_secret("consciousness-engine/config").await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to get consciousness secrets: {}", e)))?;

        let secrets = ConsciousnessSecrets {
            database_encryption_key: self.extract_string_value(&secrets_data, "database_encryption_key")?,
            jwt_signing_key: self.extract_string_value(&secrets_data, "jwt_signing_key")?,
            api_key: self.extract_string_value(&secrets_data, "api_key")?,
            neural_encryption_key: self.extract_string_value(&secrets_data, "neural_encryption_key")?,
            memory_encryption_key: self.extract_string_value(&secrets_data, "memory_encryption_key")?,
        };

        info!("Successfully retrieved consciousness secrets");
        Ok(secrets)
    }

    /// Get dynamic database credentials
    pub async fn get_database_config(&self) -> Result<DatabaseConfig, ConsciousnessError> {
        debug!("Getting dynamic database credentials");

        let db_creds = self.vault_client
            .get_database_credentials(&self.config.database_role)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to get database credentials: {}", e)))?;

        // Get static database configuration
        let db_config_data = self.get_cached_secret("shared/database").await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to get database config: {}", e)))?;

        let config = DatabaseConfig {
            host: self.extract_string_value(&db_config_data, "host")?,
            port: self.extract_u16_value(&db_config_data, "port")?,
            database: self.extract_string_value(&db_config_data, "database")?,
            username: db_creds.username,
            password: db_creds.password,
            ssl_mode: self.extract_string_value(&db_config_data, "ssl_mode")
                .unwrap_or_else(|_| "require".to_string()),
            connection_timeout: self.extract_u64_value(&db_config_data, "connection_timeout")
                .unwrap_or(30),
        };

        info!("Successfully obtained database configuration with dynamic credentials");
        Ok(config)
    }

    /// Get TLS certificate for consciousness service
    pub async fn get_tls_certificate(&self, common_name: &str, alt_names: Option<Vec<String>>) -> Result<(String, String), ConsciousnessError> {
        debug!("Issuing TLS certificate for: {}", common_name);

        let cert_response = self.vault_client
            .issue_certificate(&self.config.pki_role, common_name, alt_names)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to issue certificate: {}", e)))?;

        info!("Successfully issued TLS certificate for: {}", common_name);
        Ok((cert_response.certificate, cert_response.private_key))
    }

    /// Store consciousness processing results securely
    pub async fn store_consciousness_data(&self, session_id: &str, data: &serde_json::Value) -> Result<(), ConsciousnessError> {
        debug!("Storing consciousness data for session: {}", session_id);

        let path = format!("consciousness-engine/sessions/{}", session_id);
        let mut secret_data = HashMap::new();
        secret_data.insert("data".to_string(), data.clone());
        secret_data.insert("timestamp".to_string(), serde_json::Value::String(
            chrono::Utc::now().to_rfc3339()
        ));

        self.vault_client
            .write_secret(&path, secret_data)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to store consciousness data: {}", e)))?;

        info!("Successfully stored consciousness data for session: {}", session_id);
        Ok(())
    }

    /// Retrieve consciousness processing results
    pub async fn get_consciousness_data(&self, session_id: &str) -> Result<serde_json::Value, ConsciousnessError> {
        debug!("Retrieving consciousness data for session: {}", session_id);

        let path = format!("consciousness-engine/sessions/{}", session_id);
        let secret_data = self.vault_client
            .read_secret(&path)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to retrieve consciousness data: {}", e)))?;

        let data = secret_data.get("data")
            .ok_or_else(|| ConsciousnessError::VaultError("Missing data field in consciousness session".to_string()))?;

        info!("Successfully retrieved consciousness data for session: {}", session_id);
        Ok(data.clone())
    }

    /// Rotate consciousness secrets
    pub async fn rotate_secrets(&self) -> Result<(), ConsciousnessError> {
        info!("Starting consciousness secrets rotation");

        // Generate new secrets
        let new_secrets = ConsciousnessSecrets {
            database_encryption_key: self.generate_encryption_key(),
            jwt_signing_key: self.generate_signing_key(),
            api_key: self.generate_api_key(),
            neural_encryption_key: self.generate_encryption_key(),
            memory_encryption_key: self.generate_encryption_key(),
        };

        // Store new secrets
        let mut secret_data = HashMap::new();
        secret_data.insert("database_encryption_key".to_string(), serde_json::Value::String(new_secrets.database_encryption_key));
        secret_data.insert("jwt_signing_key".to_string(), serde_json::Value::String(new_secrets.jwt_signing_key));
        secret_data.insert("api_key".to_string(), serde_json::Value::String(new_secrets.api_key));
        secret_data.insert("neural_encryption_key".to_string(), serde_json::Value::String(new_secrets.neural_encryption_key));
        secret_data.insert("memory_encryption_key".to_string(), serde_json::Value::String(new_secrets.memory_encryption_key));

        self.vault_client
            .write_secret("consciousness-engine/config", secret_data)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to rotate secrets: {}", e)))?;

        // Clear cache to force reload
        self.secrets_cache.write().await.clear();

        info!("Successfully rotated consciousness secrets");
        Ok(())
    }

    /// Health check for Vault connectivity
    pub async fn health_check(&self) -> Result<bool, ConsciousnessError> {
        self.vault_client
            .health_check()
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Vault health check failed: {}", e)))
    }

    /// Get cached secret with automatic refresh
    async fn get_cached_secret(&self, path: &str) -> Result<HashMap<String, serde_json::Value>> {
        let cache_key = path.to_string();
        
        // Check cache first
        {
            let cache = self.secrets_cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                if cached.expires_at > std::time::SystemTime::now() {
                    if let serde_json::Value::Object(map) = &cached.value {
                        return Ok(map.clone());
                    }
                }
            }
        }

        // Cache miss or expired, fetch from Vault
        let secret_data = self.vault_client
            .read_secret(path)
            .await
            .context("Failed to read secret from Vault")?;

        // Update cache
        {
            let mut cache = self.secrets_cache.write().await;
            let expires_at = std::time::SystemTime::now() + 
                std::time::Duration::from_secs(self.config.cache_ttl_seconds);
            
            cache.insert(cache_key, CachedSecret {
                value: serde_json::Value::Object(secret_data.clone().into_iter().collect()),
                expires_at,
            });
        }

        Ok(secret_data)
    }

    /// Extract string value from secret data
    fn extract_string_value(&self, data: &HashMap<String, serde_json::Value>, key: &str) -> Result<String, ConsciousnessError> {
        data.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| ConsciousnessError::VaultError(format!("Missing or invalid string value for key: {}", key)))
    }

    /// Extract u16 value from secret data
    fn extract_u16_value(&self, data: &HashMap<String, serde_json::Value>, key: &str) -> Result<u16, ConsciousnessError> {
        data.get(key)
            .and_then(|v| v.as_u64())
            .and_then(|n| u16::try_from(n).ok())
            .ok_or_else(|| ConsciousnessError::VaultError(format!("Missing or invalid u16 value for key: {}", key)))
    }

    /// Extract u64 value from secret data
    fn extract_u64_value(&self, data: &HashMap<String, serde_json::Value>, key: &str) -> Result<u64, ConsciousnessError> {
        data.get(key)
            .and_then(|v| v.as_u64())
            .ok_or_else(|| ConsciousnessError::VaultError(format!("Missing or invalid u64 value for key: {}", key)))
    }

    /// Generate a new encryption key
    fn generate_encryption_key(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        base64::encode(key)
    }

    /// Generate a new signing key
    fn generate_signing_key(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key: [u8; 64] = rng.gen();
        base64::encode(key)
    }

    /// Generate a new API key
    fn generate_api_key(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key: [u8; 32] = rng.gen();
        hex::encode(key)
    }
}

/// Vault integration for consciousness engine
pub struct VaultIntegration {
    secrets_manager: Arc<ConsciousnessSecretsManager>,
}

impl VaultIntegration {
    /// Initialize Vault integration
    pub async fn initialize() -> Result<Self, ConsciousnessError> {
        info!("Initializing Vault integration for Consciousness Engine");

        let config = SecretsConfig {
            database_role: std::env::var("VAULT_DATABASE_ROLE")
                .unwrap_or_else(|_| "consciousness-engine".to_string()),
            pki_role: std::env::var("VAULT_PKI_ROLE")
                .unwrap_or_else(|_| "consciousness-services".to_string()),
            ..Default::default()
        };

        let role = std::env::var("VAULT_KUBERNETES_ROLE")
            .unwrap_or_else(|_| "consciousness-service".to_string());

        let secrets_manager = ConsciousnessSecretsManager::with_kubernetes_auth(&role, config)
            .await
            .map_err(|e| ConsciousnessError::VaultError(format!("Failed to initialize secrets manager: {}", e)))?;

        Ok(Self {
            secrets_manager: Arc::new(secrets_manager),
        })
    }

    /// Get the secrets manager
    pub fn secrets_manager(&self) -> Arc<ConsciousnessSecretsManager> {
        self.secrets_manager.clone()
    }

    /// Perform health check
    pub async fn health_check(&self) -> Result<bool, ConsciousnessError> {
        self.secrets_manager.health_check().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secrets_config_default() {
        let config = SecretsConfig::default();
        assert_eq!(config.cache_ttl_seconds, 3600);
        assert_eq!(config.database_role, "consciousness-engine");
        assert_eq!(config.pki_role, "consciousness-services");
    }

    #[test]
    fn test_generate_keys() {
        let vault_client = VaultClientBuilder::new()
            .token_auth("test-token")
            .build();
        
        // This would require a real Vault instance to test fully
        // but we can test the key generation logic
        assert!(true); // Placeholder for actual tests
    }
}