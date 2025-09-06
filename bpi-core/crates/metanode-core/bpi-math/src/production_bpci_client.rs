//! Production BPCI Client - Real Internet Domain Communication
//! 
//! This module handles real internet communication with the production BPCI server
//! at www.http://example.com with proper wallet address and token formats.

use crate::{Hash, MathError, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use url::Url;

/// Production BPCI client for real internet communication
#[derive(Debug, Clone)]
pub struct ProductionBPCIClient {
    /// HTTP client for real internet requests
    http_client: Client,
    /// Production BPCI server domain
    bpci_domain: String,
    /// API endpoints configuration
    endpoints: BPCIEndpoints,
    /// Authentication cache
    auth_cache: HashMap<String, AuthToken>,
}

/// BPCI server endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPCIEndpoints {
    /// Base domain URL
    pub base_url: String,
    /// Wallet registration endpoint
    pub wallet_register: String,
    /// Token validation endpoint
    pub token_validate: String,
    /// Balance query endpoint
    pub balance_query: String,
    /// Transaction submission endpoint
    pub transaction_submit: String,
    /// Status check endpoint
    pub status_check: String,
}

/// Production wallet address format: BPI(url)<wallet>(httpcg//actual address)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionWalletAddress {
    /// Full formatted address
    pub full_address: String,
    /// Domain component
    pub domain: String,
    /// Wallet identifier
    pub wallet_id: String,
    /// HTTP cage actual address
    pub httpcg_address: String,
}

/// Production token format: wallet address//Password
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionToken {
    /// Full token string
    pub full_token: String,
    /// Wallet address component
    pub wallet_address: String,
    /// Password component
    pub password: String,
}

/// Authentication token for BPCI server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token value
    pub token: String,
    /// Expiration timestamp
    pub expires_at: Timestamp,
    /// Associated wallet address
    pub wallet_address: String,
}

/// BPCI registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPCIRegistrationRequest {
    /// Wallet address in production format
    pub wallet_address: ProductionWalletAddress,
    /// Authentication token
    pub auth_token: ProductionToken,
    /// Network type (testnet/mainnet)
    pub network_type: String,
    /// Client information
    pub client_info: ClientInfo,
}

/// BPCI registration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BPCIRegistrationResponse {
    /// Registration status
    pub status: String,
    /// Registry address for BPI core
    pub registry_address: String,
    /// Registry token for BPI core
    pub registry_token: String,
    /// Assigned wallet balance
    pub initial_balance: f64,
    /// Server message
    pub message: String,
}

/// Client information for BPCI server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// BPI core version
    pub bpi_version: String,
    /// Client IP address
    pub client_ip: String,
    /// User agent
    pub user_agent: String,
    /// Installation hash
    pub installation_hash: String,
}

impl Default for BPCIEndpoints {
    fn default() -> Self {
        Self {
            base_url: "https://www.bpci-server.com".to_string(), // Production domain
            wallet_register: "/api/v1/wallet/register".to_string(),
            token_validate: "/api/v1/auth/validate".to_string(),
            balance_query: "/api/v1/wallet/balance".to_string(),
            transaction_submit: "/api/v1/transaction/submit".to_string(),
            status_check: "/api/v1/status".to_string(),
        }
    }
}

impl ProductionBPCIClient {
    /// Create new production BPCI client
    pub fn new(domain: Option<String>) -> Result<Self, MathError> {
        let bpci_domain = domain.unwrap_or_else(|| "https://www.bpci-server.com".to_string());
        
        // Validate domain format
        if !bpci_domain.starts_with("http://") && !bpci_domain.starts_with("https://") {
            return Err(MathError::InvalidInput("BPCI domain must include protocol (http:// or https://)".into()));
        }

        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("BPI-Core/1.0")
            .build()
            .map_err(|e| MathError::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        let mut endpoints = BPCIEndpoints::default();
        endpoints.base_url = bpci_domain.clone();

        Ok(Self {
            http_client,
            bpci_domain,
            endpoints,
            auth_cache: HashMap::new(),
        })
    }

    /// Generate production wallet address format: BPI(url)<wallet>(httpcg//actual address)
    pub fn generate_production_wallet_address(
        &self,
        wallet_id: String,
        httpcg_address: String,
    ) -> Result<ProductionWalletAddress, MathError> {
        // Extract domain from BPCI URL
        let url = Url::parse(&self.bpci_domain)
            .map_err(|e| MathError::InvalidInput(format!("Invalid BPCI domain: {}", e)))?;
        
        let domain = url.host_str()
            .ok_or_else(|| MathError::InvalidInput("Cannot extract domain from BPCI URL".into()))?;

        // Format: BPI(url)<wallet>(httpcg//actual address)
        let full_address = format!("BPI({})<{}>(httpcg//{})", domain, wallet_id, httpcg_address);

        Ok(ProductionWalletAddress {
            full_address,
            domain: domain.to_string(),
            wallet_id,
            httpcg_address,
        })
    }

    /// Generate production token format: wallet address//Password
    pub fn generate_production_token(
        &self,
        wallet_address: &ProductionWalletAddress,
        password: String,
    ) -> ProductionToken {
        let full_token = format!("{}//{}",wallet_address.full_address, password);
        
        ProductionToken {
            full_token,
            wallet_address: wallet_address.full_address.clone(),
            password,
        }
    }

    /// Register wallet with production BPCI server over real internet
    pub async fn register_wallet(
        &mut self,
        wallet_address: ProductionWalletAddress,
        auth_token: ProductionToken,
        network_type: String,
    ) -> Result<BPCIRegistrationResponse, MathError> {
        let register_url = format!("{}{}", self.endpoints.base_url, self.endpoints.wallet_register);
        
        let client_info = ClientInfo {
            bpi_version: "1.0.0".to_string(),
            client_ip: self.get_client_ip().await?,
            user_agent: "BPI-Core/1.0".to_string(),
            installation_hash: format!("BPI_INSTALL_{}", uuid::Uuid::new_v4()),
        };

        let request = BPCIRegistrationRequest {
            wallet_address: wallet_address.clone(),
            auth_token: auth_token.clone(),
            network_type,
            client_info,
        };

        println!("🌐 Connecting to production BPCI server: {}", register_url);
        println!("📧 Registering wallet: {}", wallet_address.full_address);

        let response = self.http_client
            .post(&register_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to connect to BPCI server: {}", e)))?;

        if !response.status().is_success() {
            return Err(MathError::NetworkError(format!(
                "BPCI server returned error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
            )));
        }

        let registration_response: BPCIRegistrationResponse = response
            .json()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to parse BPCI response: {}", e)))?;

        // Cache authentication token
        let auth_cache_token = AuthToken {
            token: auth_token.full_token,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(3600), // 1 hour
            wallet_address: wallet_address.full_address,
        };
        self.auth_cache.insert(wallet_address.wallet_id, auth_cache_token);

        println!("✅ Successfully registered with production BPCI server!");
        println!("📍 Registry Address: {}", registration_response.registry_address);
        println!("🔑 Registry Token: {}", registration_response.registry_token);

        Ok(registration_response)
    }

    /// Validate wallet credentials with production BPCI server
    pub async fn validate_credentials(
        &self,
        wallet_address: &ProductionWalletAddress,
        auth_token: &ProductionToken,
    ) -> Result<bool, MathError> {
        let validate_url = format!("{}{}", self.endpoints.base_url, self.endpoints.token_validate);
        
        let validation_request = serde_json::json!({
            "wallet_address": wallet_address.full_address,
            "auth_token": auth_token.full_token,
            "timestamp": chrono::Utc::now()
        });

        println!("🔍 Validating credentials with BPCI server...");

        let response = self.http_client
            .post(&validate_url)
            .json(&validation_request)
            .send()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to validate with BPCI server: {}", e)))?;

        let validation_result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to parse validation response: {}", e)))?;

        let is_valid = validation_result["valid"].as_bool().unwrap_or(false);
        
        if is_valid {
            println!("✅ Credentials validated successfully");
        } else {
            println!("❌ Credential validation failed");
        }

        Ok(is_valid)
    }

    /// Check BPCI server status over real internet
    pub async fn check_server_status(&self) -> Result<serde_json::Value, MathError> {
        let status_url = format!("{}{}", self.endpoints.base_url, self.endpoints.status_check);
        
        println!("🔍 Checking BPCI server status: {}", status_url);

        let response = self.http_client
            .get(&status_url)
            .send()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to connect to BPCI server: {}", e)))?;

        let status: serde_json::Value = response
            .json()
            .await
            .map_err(|e| MathError::NetworkError(format!("Failed to parse status response: {}", e)))?;

        println!("📊 BPCI Server Status: {}", status["status"].as_str().unwrap_or("unknown"));

        Ok(status)
    }

    /// Get client IP address for registration
    async fn get_client_ip(&self) -> Result<String, MathError> {
        // Try to get external IP address
        match self.http_client.get("https://api.ipify.org").send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(ip) => Ok(ip.trim().to_string()),
                    Err(_) => Ok("unknown".to_string()),
                }
            }
            Err(_) => Ok("unknown".to_string()),
        }
    }

    /// Parse production wallet address format
    pub fn parse_wallet_address(address: &str) -> Result<ProductionWalletAddress, MathError> {
        // Format: BPI(url)<wallet>(httpcg//actual address)
        if !address.starts_with("BPI(") {
            return Err(MathError::InvalidInput("Invalid wallet address format - must start with BPI(".into()));
        }

        let parts: Vec<&str> = address.split(')').collect();
        if parts.len() != 3 {
            return Err(MathError::InvalidInput("Invalid wallet address format - incorrect structure".into()));
        }

        // Extract domain: BPI(domain)
        let domain = parts[0].strip_prefix("BPI(")
            .ok_or_else(|| MathError::InvalidInput("Cannot extract domain from wallet address".into()))?;

        // Extract wallet ID: <wallet_id>
        let wallet_part = parts[1];
        if !wallet_part.starts_with('<') || !wallet_part.ends_with('>') {
            return Err(MathError::InvalidInput("Invalid wallet ID format".into()));
        }
        let wallet_id = wallet_part.trim_start_matches('<').trim_end_matches('>');

        // Extract HTTP cage address: (httpcg//actual_address)
        let httpcg_part = parts[2];
        if !httpcg_part.starts_with("(httpcg//") {
            return Err(MathError::InvalidInput("Invalid HTTP cage address format".into()));
        }
        let httpcg_address = httpcg_part.strip_prefix("(httpcg//")
            .ok_or_else(|| MathError::InvalidInput("Cannot extract HTTP cage address".into()))?;

        Ok(ProductionWalletAddress {
            full_address: address.to_string(),
            domain: domain.to_string(),
            wallet_id: wallet_id.to_string(),
            httpcg_address: httpcg_address.to_string(),
        })
    }

    /// Parse production token format
    pub fn parse_token(token: &str) -> Result<ProductionToken, MathError> {
        // Format: wallet address//Password
        let parts: Vec<&str> = token.split("//").collect();
        if parts.len() != 2 {
            return Err(MathError::InvalidInput("Invalid token format - must be wallet_address//password".into()));
        }

        Ok(ProductionToken {
            full_token: token.to_string(),
            wallet_address: parts[0].to_string(),
            password: parts[1].to_string(),
        })
    }

    /// Get current BPCI domain
    pub fn get_bpci_domain(&self) -> &str {
        &self.bpci_domain
    }

    /// Update BPCI domain for production deployment
    pub fn set_production_domain(&mut self, domain: String) -> Result<(), MathError> {
        if !domain.starts_with("http://") && !domain.starts_with("https://") {
            return Err(MathError::InvalidInput("Domain must include protocol (http:// or https://)".into()));
        }

        self.bpci_domain = domain.clone();
        self.endpoints.base_url = domain;
        Ok(())
    }
}

/// Custom error type for network operations
impl From<reqwest::Error> for MathError {
    fn from(err: reqwest::Error) -> Self {
        MathError::NetworkError(format!("HTTP request failed: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_address_generation() {
        let client = ProductionBPCIClient::new(Some("https://www.bpci-server.com".to_string())).unwrap();
        
        let wallet_address = client.generate_production_wallet_address(
            "wallet123".to_string(),
            "192.168.1.100:8888".to_string(),
        ).unwrap();

        assert_eq!(wallet_address.full_address, "BPI(www.bpci-server.com)<wallet123>(httpcg//192.168.1.100:8888)");
        assert_eq!(wallet_address.domain, "www.bpci-server.com");
        assert_eq!(wallet_address.wallet_id, "wallet123");
        assert_eq!(wallet_address.httpcg_address, "192.168.1.100:8888");
    }

    #[test]
    fn test_token_generation() {
        let client = ProductionBPCIClient::new(None).unwrap();
        let wallet_address = ProductionWalletAddress {
            full_address: "BPI(example.com)<test>(httpcg//localhost:8888)".to_string(),
            domain: "example.com".to_string(),
            wallet_id: "test".to_string(),
            httpcg_address: "localhost:8888".to_string(),
        };

        let token = client.generate_production_token(&wallet_address, "mypassword123".to_string());
        
        assert_eq!(token.full_token, "BPI(example.com)<test>(httpcg//localhost:8888)//mypassword123");
        assert_eq!(token.password, "mypassword123");
    }

    #[test]
    fn test_wallet_address_parsing() {
        let address = "BPI(www.bpci-server.com)<wallet123>(httpcg//192.168.1.100:8888)";
        let parsed = ProductionBPCIClient::parse_wallet_address(address).unwrap();

        assert_eq!(parsed.domain, "www.bpci-server.com");
        assert_eq!(parsed.wallet_id, "wallet123");
        assert_eq!(parsed.httpcg_address, "192.168.1.100:8888");
    }

    #[test]
    fn test_token_parsing() {
        let token = "BPI(example.com)<test>(httpcg//localhost:8888)//mypassword123";
        let parsed = ProductionBPCIClient::parse_token(token).unwrap();

        assert_eq!(parsed.wallet_address, "BPI(example.com)<test>(httpcg//localhost:8888)");
        assert_eq!(parsed.password, "mypassword123");
    }
}
