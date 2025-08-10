//! Vault Client for Consciousness Platform
//! 
//! Secure integration with HashiCorp Vault for secrets management,
//! dynamic credentials, and certificate management.

use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Vault client for Consciousness Platform services
#[derive(Clone)]
pub struct VaultClient {
    client: Client,
    base_url: String,
    token: RwLock<Option<String>>,
    auth_method: AuthMethod,
    config: VaultConfig,
}

/// Vault authentication methods
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// Kubernetes service account authentication
    Kubernetes {
        role: String,
        service_account_token_path: String,
    },
    /// Token-based authentication
    Token(String),
    /// AppRole authentication
    AppRole {
        role_id: String,
        secret_id: String,
    },
}

/// Vault client configuration
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// Vault server URL
    pub url: String,
    /// Request timeout
    pub timeout: Duration,
    /// Token renewal threshold (renew when TTL < threshold)
    pub renewal_threshold: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Skip TLS verification (for development only)
    pub skip_tls_verify: bool,
    /// Vault namespace (for Vault Enterprise)
    pub namespace: Option<String>,
}

/// Vault authentication response
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub auth: AuthData,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub client_token: String,
    pub lease_duration: u64,
    pub renewable: bool,
    pub policies: Vec<String>,
}

/// Vault secret response
#[derive(Debug, Deserialize)]
pub struct SecretResponse {
    pub data: SecretData,
    pub lease_duration: u64,
    pub renewable: bool,
}

#[derive(Debug, Deserialize)]
pub struct SecretData {
    pub data: HashMap<String, serde_json::Value>,
    pub metadata: Option<SecretMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct SecretMetadata {
    pub version: u32,
    pub created_time: String,
    pub deletion_time: Option<String>,
    pub destroyed: bool,
}

/// Database credentials from Vault
#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    pub lease_id: String,
    pub lease_duration: u64,
}

/// PKI certificate response
#[derive(Debug, Deserialize)]
pub struct CertificateResponse {
    pub certificate: String,
    pub private_key: String,
    pub ca_chain: Vec<String>,
    pub serial_number: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            url: "https://vault:8200".to_string(),
            timeout: Duration::from_secs(30),
            renewal_threshold: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            skip_tls_verify: false,
            namespace: None,
        }
    }
}

impl VaultClient {
    /// Create a new Vault client
    pub async fn new(auth_method: AuthMethod, config: VaultConfig) -> Result<Self> {
        let mut client_builder = ClientBuilder::new()
            .timeout(config.timeout)
            .user_agent("consciousness-platform/1.0");

        if config.skip_tls_verify {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        let client = client_builder.build()
            .context("Failed to create HTTP client")?;

        let vault_client = Self {
            client,
            base_url: config.url.clone(),
            token: RwLock::new(None),
            auth_method,
            config,
        };

        // Authenticate on creation
        vault_client.authenticate().await?;

        Ok(vault_client)
    }

    /// Authenticate with Vault
    pub async fn authenticate(&self) -> Result<()> {
        info!("Authenticating with Vault");

        let auth_response = match &self.auth_method {
            AuthMethod::Kubernetes { role, service_account_token_path } => {
                self.authenticate_kubernetes(role, service_account_token_path).await?
            }
            AuthMethod::Token(token) => {
                // For token auth, we just store the token
                let mut token_guard = self.token.write().await;
                *token_guard = Some(token.clone());
                return Ok(());
            }
            AuthMethod::AppRole { role_id, secret_id } => {
                self.authenticate_approle(role_id, secret_id).await?
            }
        };

        let mut token_guard = self.token.write().await;
        *token_guard = Some(auth_response.auth.client_token);

        info!("Successfully authenticated with Vault");
        Ok(())
    }

    /// Authenticate using Kubernetes service account
    async fn authenticate_kubernetes(&self, role: &str, token_path: &str) -> Result<AuthResponse> {
        let jwt = tokio::fs::read_to_string(token_path).await
            .context("Failed to read service account token")?;

        let auth_data = serde_json::json!({
            "role": role,
            "jwt": jwt.trim()
        });

        let url = format!("{}/v1/auth/kubernetes/login", self.base_url);
        let response = self.client
            .post(&url)
            .json(&auth_data)
            .send()
            .await
            .context("Failed to send Kubernetes auth request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Kubernetes authentication failed: {}", error_text);
        }

        let auth_response: AuthResponse = response.json().await
            .context("Failed to parse authentication response")?;

        Ok(auth_response)
    }

    /// Authenticate using AppRole
    async fn authenticate_approle(&self, role_id: &str, secret_id: &str) -> Result<AuthResponse> {
        let auth_data = serde_json::json!({
            "role_id": role_id,
            "secret_id": secret_id
        });

        let url = format!("{}/v1/auth/approle/login", self.base_url);
        let response = self.client
            .post(&url)
            .json(&auth_data)
            .send()
            .await
            .context("Failed to send AppRole auth request")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("AppRole authentication failed: {}", error_text);
        }

        let auth_response: AuthResponse = response.json().await
            .context("Failed to parse authentication response")?;

        Ok(auth_response)
    }

    /// Read a secret from Vault KV v2
    pub async fn read_secret(&self, path: &str) -> Result<HashMap<String, serde_json::Value>> {
        debug!("Reading secret from path: {}", path);

        let token = self.get_valid_token().await?;
        let url = format!("{}/v1/consciousness/data/{}", self.base_url, path);

        let mut request = self.client.get(&url)
            .header("X-Vault-Token", token);

        if let Some(namespace) = &self.config.namespace {
            request = request.header("X-Vault-Namespace", namespace);
        }

        let response = request.send().await
            .context("Failed to send secret read request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to read secret ({}): {}", status, error_text);
        }

        let secret_response: SecretResponse = response.json().await
            .context("Failed to parse secret response")?;

        debug!("Successfully read secret from path: {}", path);
        Ok(secret_response.data.data)
    }

    /// Write a secret to Vault KV v2
    pub async fn write_secret(&self, path: &str, data: HashMap<String, serde_json::Value>) -> Result<()> {
        debug!("Writing secret to path: {}", path);

        let token = self.get_valid_token().await?;
        let url = format!("{}/v1/consciousness/data/{}", self.base_url, path);

        let secret_data = serde_json::json!({
            "data": data
        });

        let mut request = self.client.post(&url)
            .header("X-Vault-Token", token)
            .json(&secret_data);

        if let Some(namespace) = &self.config.namespace {
            request = request.header("X-Vault-Namespace", namespace);
        }

        let response = request.send().await
            .context("Failed to send secret write request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to write secret ({}): {}", status, error_text);
        }

        info!("Successfully wrote secret to path: {}", path);
        Ok(())
    }

    /// Get dynamic database credentials
    pub async fn get_database_credentials(&self, role: &str) -> Result<DatabaseCredentials> {
        debug!("Getting database credentials for role: {}", role);

        let token = self.get_valid_token().await?;
        let url = format!("{}/v1/database/creds/{}", self.base_url, role);

        let mut request = self.client.get(&url)
            .header("X-Vault-Token", token);

        if let Some(namespace) = &self.config.namespace {
            request = request.header("X-Vault-Namespace", namespace);
        }

        let response = request.send().await
            .context("Failed to get database credentials")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get database credentials ({}): {}", status, error_text);
        }

        let creds_response: serde_json::Value = response.json().await
            .context("Failed to parse credentials response")?;

        let credentials = DatabaseCredentials {
            username: creds_response["data"]["username"].as_str()
                .context("Missing username in response")?
                .to_string(),
            password: creds_response["data"]["password"].as_str()
                .context("Missing password in response")?
                .to_string(),
            lease_id: creds_response["lease_id"].as_str()
                .context("Missing lease_id in response")?
                .to_string(),
            lease_duration: creds_response["lease_duration"].as_u64()
                .context("Missing lease_duration in response")?,
        };

        info!("Successfully obtained database credentials for role: {}", role);
        Ok(credentials)
    }

    /// Issue a certificate from PKI
    pub async fn issue_certificate(&self, role: &str, common_name: &str, alt_names: Option<Vec<String>>) -> Result<CertificateResponse> {
        debug!("Issuing certificate for CN: {}", common_name);

        let token = self.get_valid_token().await?;
        let url = format!("{}/v1/pki-internal/issue/{}", self.base_url, role);

        let mut cert_data = serde_json::json!({
            "common_name": common_name,
            "ttl": "720h" // 30 days
        });

        if let Some(alt_names) = alt_names {
            cert_data["alt_names"] = serde_json::Value::String(alt_names.join(","));
        }

        let mut request = self.client.post(&url)
            .header("X-Vault-Token", token)
            .json(&cert_data);

        if let Some(namespace) = &self.config.namespace {
            request = request.header("X-Vault-Namespace", namespace);
        }

        let response = request.send().await
            .context("Failed to issue certificate")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to issue certificate ({}): {}", status, error_text);
        }

        let cert_response: serde_json::Value = response.json().await
            .context("Failed to parse certificate response")?;

        let certificate = CertificateResponse {
            certificate: cert_response["data"]["certificate"].as_str()
                .context("Missing certificate in response")?
                .to_string(),
            private_key: cert_response["data"]["private_key"].as_str()
                .context("Missing private_key in response")?
                .to_string(),
            ca_chain: cert_response["data"]["ca_chain"].as_array()
                .context("Missing ca_chain in response")?
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect(),
            serial_number: cert_response["data"]["serial_number"].as_str()
                .context("Missing serial_number in response")?
                .to_string(),
        };

        info!("Successfully issued certificate for CN: {}", common_name);
        Ok(certificate)
    }

    /// Renew the current token
    pub async fn renew_token(&self) -> Result<()> {
        debug!("Renewing Vault token");

        let token = {
            let token_guard = self.token.read().await;
            token_guard.as_ref()
                .context("No token available for renewal")?
                .clone()
        };

        let url = format!("{}/v1/auth/token/renew-self", self.base_url);
        let mut request = self.client.post(&url)
            .header("X-Vault-Token", &token);

        if let Some(namespace) = &self.config.namespace {
            request = request.header("X-Vault-Namespace", namespace);
        }

        let response = request.send().await
            .context("Failed to renew token")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            warn!("Token renewal failed ({}): {}", status, error_text);
            
            // If renewal fails, try to re-authenticate
            self.authenticate().await?;
        } else {
            info!("Successfully renewed Vault token");
        }

        Ok(())
    }

    /// Get a valid token, renewing if necessary
    async fn get_valid_token(&self) -> Result<String> {
        let token_guard = self.token.read().await;
        match token_guard.as_ref() {
            Some(token) => Ok(token.clone()),
            None => {
                drop(token_guard);
                self.authenticate().await?;
                let new_token_guard = self.token.read().await;
                new_token_guard.as_ref()
                    .context("Token not available after authentication")?
                    .clone()
                    .pipe(Ok)
            }
        }
    }

    /// Check if the client is healthy
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/v1/sys/health", self.base_url);
        
        let response = self.client.get(&url)
            .send()
            .await
            .context("Failed to check Vault health")?;

        Ok(response.status().is_success())
    }
}

/// Convenience trait for pipeline operations
trait Pipe<T> {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(T) -> U;
}

impl<T> Pipe<T> for T {
    fn pipe<F, U>(self, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        f(self)
    }
}

/// Vault client builder for easier configuration
pub struct VaultClientBuilder {
    config: VaultConfig,
    auth_method: Option<AuthMethod>,
}

impl VaultClientBuilder {
    pub fn new() -> Self {
        Self {
            config: VaultConfig::default(),
            auth_method: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.config.url = url.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn skip_tls_verify(mut self, skip: bool) -> Self {
        self.config.skip_tls_verify = skip;
        self
    }

    pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
        self.config.namespace = Some(namespace.into());
        self
    }

    pub fn kubernetes_auth(mut self, role: impl Into<String>) -> Self {
        self.auth_method = Some(AuthMethod::Kubernetes {
            role: role.into(),
            service_account_token_path: "/var/run/secrets/kubernetes.io/serviceaccount/token".to_string(),
        });
        self
    }

    pub fn token_auth(mut self, token: impl Into<String>) -> Self {
        self.auth_method = Some(AuthMethod::Token(token.into()));
        self
    }

    pub fn approle_auth(mut self, role_id: impl Into<String>, secret_id: impl Into<String>) -> Self {
        self.auth_method = Some(AuthMethod::AppRole {
            role_id: role_id.into(),
            secret_id: secret_id.into(),
        });
        self
    }

    pub async fn build(self) -> Result<VaultClient> {
        let auth_method = self.auth_method
            .context("Authentication method must be specified")?;

        VaultClient::new(auth_method, self.config).await
    }
}

impl Default for VaultClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_vault_client_builder() {
        let builder = VaultClientBuilder::new()
            .url("https://vault.test:8200")
            .timeout(Duration::from_secs(10))
            .skip_tls_verify(true)
            .token_auth("test-token");

        // This would fail in tests without a real Vault instance
        // but demonstrates the builder pattern
        assert!(builder.config.skip_tls_verify);
        assert_eq!(builder.config.url, "https://vault.test:8200");
    }
}