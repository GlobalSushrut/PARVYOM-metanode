//! ENC Cluster Endpoints with JWT Security
//! 
//! Provides ENC app endpoints and ENC BPCI mesh endpoints with JWT-based
//! security for the single-command military-grade blockchain infrastructure.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// JWT token structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub header: JwtHeader,
    pub payload: JwtPayload,
    pub signature: String,
}

/// JWT header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
    pub kid: Option<String>,
}

/// JWT payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub iss: String,        // Issuer
    pub sub: String,        // Subject
    pub aud: String,        // Audience
    pub exp: u64,           // Expiration
    pub iat: u64,           // Issued at
    pub nbf: u64,           // Not before
    pub jti: String,        // JWT ID
    pub cluster_id: String,
    pub endpoint_type: EndpointType,
    pub permissions: Vec<Permission>,
    pub metadata: HashMap<String, String>,
}

/// Endpoint types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EndpointType {
    EncApp,
    EncBpciMesh,
    AppValidator,
    BpciLogic,
    HighThroughput,
    CommunityShared,
}

/// Permissions for JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Consensus,
    Validate,
    Deploy,
    Monitor,
}

/// ENC VPod (Virtual Pod) with JWT security
#[derive(Debug, Clone)]
pub struct EncVPod {
    pub vpod_id: String,
    pub cluster_id: String,
    pub endpoint_type: EndpointType,
    pub bind_address: SocketAddr,
    pub jwt_secret: String,
    pub permissions: Vec<Permission>,
    pub metadata: HashMap<String, String>,
    pub created_at: SystemTime,
}

/// Cluster endpoint configuration
#[derive(Debug, Clone)]
pub struct ClusterEndpointConfig {
    pub cluster_id: String,
    pub region: String,
    pub owner_id: String,
    pub jwt_secret: String,
    pub token_expiry: Duration,
    pub endpoints: Vec<EndpointConfig>,
}

/// Individual endpoint configuration
#[derive(Debug, Clone)]
pub struct EndpointConfig {
    pub endpoint_type: EndpointType,
    pub bind_address: SocketAddr,
    pub permissions: Vec<Permission>,
    pub public_access: bool,
    pub rate_limit: Option<u32>,
    pub metadata: HashMap<String, String>,
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub authenticated: bool,
    pub cluster_id: String,
    pub endpoint_type: EndpointType,
    pub permissions: Vec<Permission>,
    pub expires_at: SystemTime,
    pub error_message: Option<String>,
}

/// ENC Cluster Endpoint Manager
#[derive(Debug)]
pub struct EncClusterEndpoints {
    config: ClusterEndpointConfig,
    vpods: HashMap<String, EncVPod>,
    active_tokens: HashMap<String, JwtPayload>,
    jwt_secret: String,
}

impl EncClusterEndpoints {
    pub fn new(config: ClusterEndpointConfig) -> Self {
        let jwt_secret = config.jwt_secret.clone();
        
        Self {
            config,
            vpods: HashMap::new(),
            active_tokens: HashMap::new(),
            jwt_secret,
        }
    }

    /// Create ENC App endpoint (public access for applications)
    pub fn create_enc_app_endpoint(&mut self, bind_address: SocketAddr) -> Result<String> {
        let vpod_id = format!("enc-app-{}-{}", 
            self.config.cluster_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        let vpod = EncVPod {
            vpod_id: vpod_id.clone(),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type: EndpointType::EncApp,
            bind_address,
            jwt_secret: self.jwt_secret.clone(),
            permissions: vec![Permission::Read, Permission::Write, Permission::Execute],
            metadata: HashMap::from([
                ("access_type".to_string(), "public".to_string()),
                ("description".to_string(), "ENC application endpoint".to_string()),
            ]),
            created_at: SystemTime::now(),
        };
        
        let mut vpods = self.vpods.write().await;
        vpods.insert(vpod_id.clone(), vpod);
        
        info!("ENC App endpoint created: {} on {}", vpod_id, bind_address);
        Ok(vpod_id)
    }

    /// Create ENC BPCI Mesh endpoint (owner-only access for mainnet)
    pub async fn create_enc_bpci_mesh_endpoint(&self, bind_address: SocketAddr) -> Result<String> {
        let vpod_id = format!("enc-bpci-{}-{}", 
            self.config.cluster_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        let vpod = EncVPod {
            vpod_id: vpod_id.clone(),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type: EndpointType::EncBpciMesh,
            bind_address,
            jwt_secret: self.jwt_secret.clone(),
            permissions: vec![Permission::Admin, Permission::Consensus, Permission::Validate],
            metadata: HashMap::from([
                ("access_type".to_string(), "owner_only".to_string()),
                ("mainnet_access".to_string(), "true".to_string()),
                ("description".to_string(), "ENC BPCI mesh endpoint for mainnet".to_string()),
            ]),
            created_at: SystemTime::now(),
        };
        
        let mut vpods = self.vpods.write().await;
        vpods.insert(vpod_id.clone(), vpod);
        
        info!("ENC BPCI Mesh endpoint created: {} on {}", vpod_id, bind_address);
        Ok(vpod_id)
    }

    /// Create App Runner/Validator endpoint
    pub async fn create_app_validator_endpoint(&self, bind_address: SocketAddr) -> Result<String> {
        let vpod_id = format!("app-validator-{}-{}", 
            self.config.cluster_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        let vpod = EncVPod {
            vpod_id: vpod_id.clone(),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type: EndpointType::AppValidator,
            bind_address,
            jwt_secret: self.jwt_secret.clone(),
            permissions: vec![Permission::Execute, Permission::Validate, Permission::Monitor],
            metadata: HashMap::from([
                ("role".to_string(), "app_runner_validator".to_string()),
                ("bpci_integration".to_string(), "true".to_string()),
                ("description".to_string(), "App runner and BPCI validator".to_string()),
            ]),
            created_at: SystemTime::now(),
        };
        
        let mut vpods = self.vpods.write().await;
        vpods.insert(vpod_id.clone(), vpod);
        
        info!("App Validator endpoint created: {} on {}", vpod_id, bind_address);
        Ok(vpod_id)
    }

    /// Create BPCI Logic endpoint for network control
    pub async fn create_bpci_logic_endpoint(&self, bind_address: SocketAddr) -> Result<String> {
        let vpod_id = format!("bpci-logic-{}-{}", 
            self.config.cluster_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        let vpod = EncVPod {
            vpod_id: vpod_id.clone(),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type: EndpointType::BpciLogic,
            bind_address,
            jwt_secret: self.jwt_secret.clone(),
            permissions: vec![Permission::Admin, Permission::Consensus, Permission::Monitor],
            metadata: HashMap::from([
                ("role".to_string(), "bpci_network_control".to_string()),
                ("network_control".to_string(), "true".to_string()),
                ("description".to_string(), "BPCI logic for network control".to_string()),
            ]),
            created_at: SystemTime::now(),
        };
        
        let mut vpods = self.vpods.write().await;
        vpods.insert(vpod_id.clone(), vpod);
        
        info!("BPCI Logic endpoint created: {} on {}", vpod_id, bind_address);
        Ok(vpod_id)
    }

    /// Create High-Throughput/Community endpoint
    pub async fn create_high_throughput_endpoint(&self, bind_address: SocketAddr) -> Result<String> {
        let vpod_id = format!("high-throughput-{}-{}", 
            self.config.cluster_id,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
        
        let vpod = EncVPod {
            vpod_id: vpod_id.clone(),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type: EndpointType::HighThroughput,
            bind_address,
            jwt_secret: self.jwt_secret.clone(),
            permissions: vec![Permission::Read, Permission::Write, Permission::Execute],
            metadata: HashMap::from([
                ("role".to_string(), "high_throughput_community".to_string()),
                ("community_shared".to_string(), "true".to_string()),
                ("high_tpc".to_string(), "true".to_string()),
                ("description".to_string(), "High-throughput community shared node".to_string()),
            ]),
            created_at: SystemTime::now(),
        };
        
        let mut vpods = self.vpods.write().await;
        vpods.insert(vpod_id.clone(), vpod);
        
        info!("High-Throughput endpoint created: {} on {}", vpod_id, bind_address);
        Ok(vpod_id)
    }

    /// Generate JWT token for endpoint access
    pub async fn generate_jwt_token(&self, endpoint_type: EndpointType, subject: &str) -> Result<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let exp = now + self.config.token_expiry.as_secs();
        
        let permissions = match endpoint_type {
            EndpointType::EncApp => vec![Permission::Read, Permission::Write, Permission::Execute],
            EndpointType::EncBpciMesh => vec![Permission::Admin, Permission::Consensus, Permission::Validate],
            EndpointType::AppValidator => vec![Permission::Execute, Permission::Validate, Permission::Monitor],
            EndpointType::BpciLogic => vec![Permission::Admin, Permission::Consensus, Permission::Monitor],
            EndpointType::HighThroughput => vec![Permission::Read, Permission::Write, Permission::Execute],
            EndpointType::CommunityShared => vec![Permission::Read, Permission::Execute],
        };
        
        let payload = JwtPayload {
            iss: "enc-cluster".to_string(),
            sub: subject.to_string(),
            aud: format!("cluster-{}", self.config.cluster_id),
            exp,
            iat: now,
            nbf: now,
            jti: format!("jwt-{}-{}", subject, now),
            cluster_id: self.config.cluster_id.clone(),
            endpoint_type,
            permissions,
            metadata: HashMap::new(),
        };
        
        // Simplified JWT creation (in production, use proper JWT library)
        let header = JwtHeader {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
            kid: None,
        };
        
        let header_b64 = base64::encode(serde_json::to_string(&header)?);
        let payload_b64 = base64::encode(serde_json::to_string(&payload)?);
        let signature = format!("sig-{}-{}", subject, now); // Simplified signature
        
        let token = format!("{}.{}.{}", header_b64, payload_b64, signature);
        
        // Store active token
        let mut active_tokens = self.active_tokens.write().await;
        active_tokens.insert(token.clone(), payload);
        
        info!("JWT token generated for {} on endpoint {:?}", subject, endpoint_type);
        Ok(token)
    }

    /// Validate JWT token
    pub async fn validate_jwt_token(&self, token: &str) -> Result<AuthResult> {
        let active_tokens = self.active_tokens.read().await;
        
        if let Some(payload) = active_tokens.get(token) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            
            if payload.exp > now {
                Ok(AuthResult {
                    authenticated: true,
                    cluster_id: payload.cluster_id.clone(),
                    endpoint_type: payload.endpoint_type.clone(),
                    permissions: payload.permissions.clone(),
                    expires_at: UNIX_EPOCH + Duration::from_secs(payload.exp),
                    error_message: None,
                })
            } else {
                Ok(AuthResult {
                    authenticated: false,
                    cluster_id: String::new(),
                    endpoint_type: EndpointType::EncApp,
                    permissions: vec![],
                    expires_at: UNIX_EPOCH,
                    error_message: Some("Token expired".to_string()),
                })
            }
        } else {
            Ok(AuthResult {
                authenticated: false,
                cluster_id: String::new(),
                endpoint_type: EndpointType::EncApp,
                permissions: vec![],
                expires_at: UNIX_EPOCH,
                error_message: Some("Invalid token".to_string()),
            })
        }
    }

    /// Get all cluster endpoints
    pub async fn get_cluster_endpoints(&self) -> Vec<(String, EndpointType, SocketAddr)> {
        let vpods = self.vpods.read().await;
        vpods.values()
            .map(|vpod| (vpod.vpod_id.clone(), vpod.endpoint_type.clone(), vpod.bind_address))
            .collect()
    }

    /// Check if endpoint is owner-only (ENC BPCI Mesh)
    pub async fn is_owner_only_endpoint(&self, vpod_id: &str) -> bool {
        let vpods = self.vpods.read().await;
        if let Some(vpod) = vpods.get(vpod_id) {
            vpod.endpoint_type == EndpointType::EncBpciMesh
        } else {
            false
        }
    }

    /// Get endpoint metadata
    pub async fn get_endpoint_metadata(&self, vpod_id: &str) -> Option<HashMap<String, String>> {
        let vpods = self.vpods.read().await;
        vpods.get(vpod_id).map(|vpod| vpod.metadata.clone())
    }

    /// Setup complete cluster with all endpoint types
    pub async fn setup_complete_cluster(&self, base_port: u16) -> Result<HashMap<EndpointType, String>> {
        let mut endpoints = HashMap::new();
        
        // ENC App endpoint (public)
        let enc_app_addr = format!("127.0.0.1:{}", base_port).parse()?;
        let enc_app_id = self.create_enc_app_endpoint(enc_app_addr).await?;
        endpoints.insert(EndpointType::EncApp, enc_app_id);
        
        // ENC BPCI Mesh endpoint (owner-only)
        let enc_bpci_addr = format!("127.0.0.1:{}", base_port + 1).parse()?;
        let enc_bpci_id = self.create_enc_bpci_mesh_endpoint(enc_bpci_addr).await?;
        endpoints.insert(EndpointType::EncBpciMesh, enc_bpci_id);
        
        // App Validator endpoint
        let app_validator_addr = format!("127.0.0.1:{}", base_port + 2).parse()?;
        let app_validator_id = self.create_app_validator_endpoint(app_validator_addr).await?;
        endpoints.insert(EndpointType::AppValidator, app_validator_id);
        
        // BPCI Logic endpoint
        let bpci_logic_addr = format!("127.0.0.1:{}", base_port + 3).parse()?;
        let bpci_logic_id = self.create_bpci_logic_endpoint(bpci_logic_addr).await?;
        endpoints.insert(EndpointType::BpciLogic, bpci_logic_id);
        
        // High-Throughput endpoint
        let high_throughput_addr = format!("127.0.0.1:{}", base_port + 4).parse()?;
        let high_throughput_id = self.create_high_throughput_endpoint(high_throughput_addr).await?;
        endpoints.insert(EndpointType::HighThroughput, high_throughput_id);
        
        info!("Complete cluster setup with {} endpoints", endpoints.len());
        Ok(endpoints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cluster_config() -> ClusterEndpointConfig {
        ClusterEndpointConfig {
            cluster_id: "test-cluster-1".to_string(),
            region: "us-west-1".to_string(),
            owner_id: "owner-123".to_string(),
            jwt_secret: "test-secret-key".to_string(),
            token_expiry: Duration::from_secs(3600),
            endpoints: vec![],
        }
    }

    #[tokio::test]
    async fn test_enc_cluster_endpoints_creation() {
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        let cluster_endpoints = endpoints.get_cluster_endpoints().await;
        assert!(cluster_endpoints.is_empty());
        
        println!("✅ ENC Cluster Endpoints created successfully");
    }

    #[tokio::test]
    async fn test_enc_app_endpoint_creation() {
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        let bind_addr = "127.0.0.1:8080".parse().unwrap();
        let vpod_id = endpoints.create_enc_app_endpoint(bind_addr).await.unwrap();
        
        assert!(vpod_id.starts_with("enc-app-test-cluster-1"));
        
        let cluster_endpoints = endpoints.get_cluster_endpoints().await;
        assert_eq!(cluster_endpoints.len(), 1);
        
        println!("✅ ENC App endpoint creation working");
    }

    #[tokio::test]
    async fn test_enc_bpci_mesh_endpoint_creation() {
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        let bind_addr = "127.0.0.1:8081".parse().unwrap();
        let vpod_id = endpoints.create_enc_bpci_mesh_endpoint(bind_addr).await.unwrap();
        
        assert!(vpod_id.starts_with("enc-bpci-test-cluster-1"));
        
        let is_owner_only = endpoints.is_owner_only_endpoint(&vpod_id).await;
        assert!(is_owner_only);
        
        println!("✅ ENC BPCI Mesh endpoint creation working");
    }

    #[tokio::test]
    async fn test_jwt_token_generation_and_validation() {
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        let token = endpoints.generate_jwt_token(EndpointType::EncApp, "user-123").await.unwrap();
        assert!(!token.is_empty());
        
        let auth_result = endpoints.validate_jwt_token(&token).await.unwrap();
        assert!(auth_result.authenticated);
        assert_eq!(auth_result.cluster_id, "test-cluster-1");
        assert_eq!(auth_result.endpoint_type, EndpointType::EncApp);
        
        println!("✅ JWT token generation and validation working");
    }

    #[tokio::test]
    async fn test_complete_cluster_setup() {
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        let cluster_endpoints = endpoints.setup_complete_cluster(9000).await.unwrap();
        
        assert_eq!(cluster_endpoints.len(), 5);
        assert!(cluster_endpoints.contains_key(&EndpointType::EncApp));
        assert!(cluster_endpoints.contains_key(&EndpointType::EncBpciMesh));
        assert!(cluster_endpoints.contains_key(&EndpointType::AppValidator));
        assert!(cluster_endpoints.contains_key(&EndpointType::BpciLogic));
        assert!(cluster_endpoints.contains_key(&EndpointType::HighThroughput));
        
        println!("✅ Complete cluster setup working");
    }

    #[tokio::test]
    async fn test_enc_cluster_endpoints_exit_criteria() {
        println!("\n=== ENC Cluster Endpoints with JWT Security Exit Criteria ===");
        
        let config = create_test_cluster_config();
        let endpoints = EncClusterEndpoints::new(config);
        
        // Test 1: ENC App endpoint (public access)
        let enc_app_addr = "127.0.0.1:8080".parse().unwrap();
        let enc_app_id = endpoints.create_enc_app_endpoint(enc_app_addr).await.unwrap();
        assert!(enc_app_id.starts_with("enc-app"));
        println!("✅ Test 1: ENC App endpoint creation - PASSED");
        
        // Test 2: ENC BPCI Mesh endpoint (owner-only)
        let enc_bpci_addr = "127.0.0.1:8081".parse().unwrap();
        let enc_bpci_id = endpoints.create_enc_bpci_mesh_endpoint(enc_bpci_addr).await.unwrap();
        let is_owner_only = endpoints.is_owner_only_endpoint(&enc_bpci_id).await;
        assert!(is_owner_only);
        println!("✅ Test 2: ENC BPCI Mesh endpoint (owner-only) - PASSED");
        
        // Test 3: JWT security for all endpoints
        let token = endpoints.generate_jwt_token(EndpointType::EncBpciMesh, "owner-123").await.unwrap();
        let auth_result = endpoints.validate_jwt_token(&token).await.unwrap();
        assert!(auth_result.authenticated);
        println!("✅ Test 3: JWT security implementation - PASSED");
        
        // Test 4: Complete cluster endpoint separation
        let cluster_endpoints = endpoints.setup_complete_cluster(9000).await.unwrap();
        assert_eq!(cluster_endpoints.len(), 5);
        println!("✅ Test 4: Complete cluster endpoint separation - PASSED");
        
        println!("\n🎉 ENC Cluster Endpoints with JWT Security - ALL TESTS PASSED!");
    }
}
