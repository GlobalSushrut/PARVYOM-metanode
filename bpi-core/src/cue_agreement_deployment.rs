//! CUE Agreement Deployment and Burn System
//! 
//! This module implements the deployment, burn, and activation system for CUE agreements
//! similar to Solidity contract deployment. Once deployed and "burned", agreements get
//! immutable addresses that control pipeline logic and component working abilities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, debug, warn, error};
use sha2::{Sha256, Digest};

/// CUE Agreement Deployment Manager - handles deployment, burn, and activation
#[derive(Debug)]
pub struct CueAgreementDeploymentManager {
    pub deployed_agreements: Arc<RwLock<HashMap<String, DeployedAgreement>>>,
    pub burned_agreements: Arc<RwLock<HashMap<String, BurnedAgreement>>>,
    pub active_pipelines: Arc<RwLock<HashMap<String, PipelineController>>>,
    pub deployment_config: DeploymentConfig,
}

/// Deployment configuration
#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    pub deployment_directory: String,
    pub burn_confirmation_blocks: u32,
    pub enable_pipeline_control: bool,
    pub require_wallet_signature: bool,
}

/// Deployed agreement (before burning)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedAgreement {
    pub deployment_id: String,
    pub agreement_type: AgreementType,
    pub cue_file_path: String,
    pub cue_content_hash: String,
    pub deployer_wallet: Option<String>,
    pub deployed_at: DateTime<Utc>,
    pub status: DeploymentStatus,
    pub deployment_transaction: String,
}

/// Burned agreement (immutable with address)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurnedAgreement {
    pub agreement_address: String,
    pub deployment_id: String,
    pub agreement_type: AgreementType,
    pub cue_content_hash: String,
    pub burned_at: DateTime<Utc>,
    pub burn_transaction: String,
    pub pipeline_permissions: PipelinePermissions,
    pub component_controls: ComponentControls,
}

/// Agreement types for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgreementType {
    SmartContract {
        contract_type: String,
        bpi_layer: bool,
        fiat_payment_enabled: bool,
    },
    SmartContractPlusPlus {
        governance_type: String,
        multi_bpi_mesh: bool,
        jurisdiction: String,
    },
    AgreementPlus {
        enforcement_type: String,
        cross_bpi: bool,
        wallet_stamp_authority: bool,
    },
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Pending,
    Deployed,
    ReadyToBurn,
    Burned,
    Failed(String),
}

/// Pipeline permissions controlled by burned agreement address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelinePermissions {
    pub can_control_docklock: bool,
    pub can_control_enccluster: bool,
    pub can_control_biso: bool,
    pub can_control_trafficlight: bool,
    pub can_control_storage: bool,
    pub can_control_networking: bool,
    pub can_control_security: bool,
}

/// Component controls managed by agreement address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentControls {
    pub controlled_components: Vec<String>,
    pub resource_limits: HashMap<String, u64>,
    pub security_policies: Vec<String>,
    pub compliance_requirements: Vec<String>,
}

/// Pipeline controller for burned agreements
#[derive(Debug, Clone)]
pub struct PipelineController {
    pub agreement_address: String,
    pub controlled_pipelines: Vec<String>,
    pub active_controls: HashMap<String, serde_json::Value>,
}

impl CueAgreementDeploymentManager {
    /// Create new deployment manager
    pub fn new(config: DeploymentConfig) -> Self {
        Self {
            deployed_agreements: Arc::new(RwLock::new(HashMap::new())),
            burned_agreements: Arc::new(RwLock::new(HashMap::new())),
            active_pipelines: Arc::new(RwLock::new(HashMap::new())),
            deployment_config: config,
        }
    }

    /// Deploy CUE agreement file/folder to specific area
    pub async fn deploy_agreement(
        &self,
        cue_path: &str,
        agreement_type: AgreementType,
        deployer_wallet: Option<String>,
    ) -> Result<String> {
        let deployment_id = Uuid::new_v4().to_string();
        
        info!("🚀 Deploying CUE agreement: {} (ID: {})", cue_path, deployment_id);

        // Read and hash CUE content
        let cue_content = tokio::fs::read_to_string(cue_path).await
            .map_err(|e| anyhow!("Failed to read CUE file: {}", e))?;
        
        let content_hash = self.generate_content_hash(&cue_content);
        
        // Create deployment transaction
        let deployment_transaction = self.create_deployment_transaction(&deployment_id, &content_hash).await?;
        
        // Create deployed agreement
        let deployed_agreement = DeployedAgreement {
            deployment_id: deployment_id.clone(),
            agreement_type,
            cue_file_path: cue_path.to_string(),
            cue_content_hash: content_hash,
            deployer_wallet,
            deployed_at: Utc::now(),
            status: DeploymentStatus::Deployed,
            deployment_transaction,
        };

        // Store deployment
        {
            let mut agreements = self.deployed_agreements.write().await;
            agreements.insert(deployment_id.clone(), deployed_agreement);
        }

        info!("✅ CUE agreement deployed successfully: {}", deployment_id);
        Ok(deployment_id)
    }

    /// Burn deployed agreement to create immutable address
    pub async fn burn_agreement(
        &self,
        deployment_id: &str,
        wallet_signature: Option<String>,
    ) -> Result<String> {
        info!("🔥 Burning CUE agreement: {}", deployment_id);

        // Get deployed agreement
        let deployed_agreement = {
            let agreements = self.deployed_agreements.read().await;
            agreements.get(deployment_id)
                .ok_or_else(|| anyhow!("Deployment not found: {}", deployment_id))?
                .clone()
        };

        // Validate burn requirements
        if self.deployment_config.require_wallet_signature && wallet_signature.is_none() {
            return Err(anyhow!("Wallet signature required for burning"));
        }

        // Generate agreement address (similar to contract address generation)
        let agreement_address = self.generate_agreement_address(&deployed_agreement).await?;
        
        // Create burn transaction
        let burn_transaction = self.create_burn_transaction(&deployment_id, &agreement_address).await?;
        
        // Determine pipeline permissions and component controls
        let (pipeline_permissions, component_controls) = self.determine_controls(&deployed_agreement.agreement_type);
        
        // Create burned agreement
        let burned_agreement = BurnedAgreement {
            agreement_address: agreement_address.clone(),
            deployment_id: deployment_id.to_string(),
            agreement_type: deployed_agreement.agreement_type,
            cue_content_hash: deployed_agreement.cue_content_hash,
            burned_at: Utc::now(),
            burn_transaction,
            pipeline_permissions,
            component_controls,
        };

        // Store burned agreement
        {
            let mut burned = self.burned_agreements.write().await;
            burned.insert(agreement_address.clone(), burned_agreement);
        }

        // Update deployment status
        {
            let mut agreements = self.deployed_agreements.write().await;
            if let Some(agreement) = agreements.get_mut(deployment_id) {
                agreement.status = DeploymentStatus::Burned;
            }
        }

        info!("🔥✅ Agreement burned successfully! Address: {}", agreement_address);
        Ok(agreement_address)
    }

    /// Activate burned agreement for pipeline control
    pub async fn activate_agreement(&self, agreement_address: &str) -> Result<()> {
        info!("⚡ Activating agreement address: {}", agreement_address);

        // Get burned agreement
        let burned_agreement = {
            let burned = self.burned_agreements.read().await;
            burned.get(agreement_address)
                .ok_or_else(|| anyhow!("Burned agreement not found: {}", agreement_address))?
                .clone()
        };

        // Create pipeline controller
        let pipeline_controller = PipelineController {
            agreement_address: agreement_address.to_string(),
            controlled_pipelines: burned_agreement.component_controls.controlled_components.clone(),
            active_controls: HashMap::new(),
        };

        // Activate pipeline controls
        {
            let mut pipelines = self.active_pipelines.write().await;
            pipelines.insert(agreement_address.to_string(), pipeline_controller);
        }

        info!("⚡✅ Agreement activated! Address {} now controls pipelines", agreement_address);
        Ok(())
    }

    /// Generate content hash for CUE file
    fn generate_content_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("sha256:{:x}", hasher.finalize())
    }

    /// Generate agreement address (like contract address)
    async fn generate_agreement_address(&self, deployed: &DeployedAgreement) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(deployed.deployment_id.as_bytes());
        hasher.update(deployed.cue_content_hash.as_bytes());
        hasher.update(deployed.deployed_at.timestamp().to_string().as_bytes());
        
        let hash = hasher.finalize();
        Ok(format!("bpi:{}", hex::encode(&hash[..20]))) // 40 character address
    }

    /// Create deployment transaction
    async fn create_deployment_transaction(&self, deployment_id: &str, content_hash: &str) -> Result<String> {
        let transaction_id = Uuid::new_v4().to_string();
        info!("📝 Created deployment transaction: {} for deployment: {}", transaction_id, deployment_id);
        Ok(transaction_id)
    }

    /// Create burn transaction
    async fn create_burn_transaction(&self, deployment_id: &str, agreement_address: &str) -> Result<String> {
        let transaction_id = Uuid::new_v4().to_string();
        info!("🔥📝 Created burn transaction: {} for address: {}", transaction_id, agreement_address);
        Ok(transaction_id)
    }

    /// Determine pipeline permissions and component controls based on agreement type
    fn determine_controls(&self, agreement_type: &AgreementType) -> (PipelinePermissions, ComponentControls) {
        match agreement_type {
            AgreementType::SmartContract { .. } => {
                (
                    PipelinePermissions {
                        can_control_docklock: true,
                        can_control_enccluster: false,
                        can_control_biso: false,
                        can_control_trafficlight: false,
                        can_control_storage: true,
                        can_control_networking: true,
                        can_control_security: false,
                    },
                    ComponentControls {
                        controlled_components: vec!["docklock".to_string(), "storage".to_string()],
                        resource_limits: HashMap::new(),
                        security_policies: vec![],
                        compliance_requirements: vec![],
                    }
                )
            },
            AgreementType::SmartContractPlusPlus { .. } => {
                (
                    PipelinePermissions {
                        can_control_docklock: true,
                        can_control_enccluster: true,
                        can_control_biso: true,
                        can_control_trafficlight: true,
                        can_control_storage: true,
                        can_control_networking: true,
                        can_control_security: true,
                    },
                    ComponentControls {
                        controlled_components: vec![
                            "docklock".to_string(), 
                            "enccluster".to_string(),
                            "biso".to_string(),
                            "trafficlight".to_string()
                        ],
                        resource_limits: HashMap::new(),
                        security_policies: vec!["governance".to_string()],
                        compliance_requirements: vec!["multi_bpi".to_string()],
                    }
                )
            },
            AgreementType::AgreementPlus { .. } => {
                (
                    PipelinePermissions {
                        can_control_docklock: true,
                        can_control_enccluster: true,
                        can_control_biso: true,
                        can_control_trafficlight: true,
                        can_control_storage: true,
                        can_control_networking: true,
                        can_control_security: true,
                    },
                    ComponentControls {
                        controlled_components: vec![
                            "enforcement".to_string(),
                            "cross_bpi".to_string(),
                            "wallet_stamp".to_string()
                        ],
                        resource_limits: HashMap::new(),
                        security_policies: vec!["cross_bpi_enforcement".to_string()],
                        compliance_requirements: vec!["wallet_stamp_authority".to_string()],
                    }
                )
            }
        }
    }

    /// Get agreement by address
    pub async fn get_agreement_by_address(&self, address: &str) -> Result<BurnedAgreement> {
        let burned = self.burned_agreements.read().await;
        Ok(burned.get(address)
            .ok_or_else(|| anyhow!("Agreement not found at address: {}", address))?
            .clone())

    }

    /// List all deployed agreements
    pub async fn list_deployed_agreements(&self) -> Vec<DeployedAgreement> {
        let agreements = self.deployed_agreements.read().await;
        agreements.values().cloned().collect()
    }

    /// List all burned agreements
    pub async fn list_burned_agreements(&self) -> Vec<BurnedAgreement> {
        let burned = self.burned_agreements.read().await;
        burned.values().cloned().collect()
    }
}

/// Default deployment configuration
impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            deployment_directory: "/bpi/agreements/deployed".to_string(),
            burn_confirmation_blocks: 6,
            enable_pipeline_control: true,
            require_wallet_signature: true,
        }
    }
}
