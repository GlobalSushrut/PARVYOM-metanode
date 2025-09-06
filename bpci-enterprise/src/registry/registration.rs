use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Result;

use super::{NodeType, NodeCapability, NodeStatus, NetworkEndpoints, ReputationScore};
use super::{IdentityProof, AuthorityLevel};

/// Complete node registration structure for the enhanced BPCI Registry System
/// Combines node information, identity proof, authority level, and operational metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRegistration {
    /// Unique node identifier (generated during registration)
    pub node_id: Option<String>,
    /// Type of node being registered
    pub node_type: NodeType,
    /// Identity proof (D-Adhaar + D-PAN)
    pub identity: IdentityProof,
    /// Authority level (Community/Bank/Hybrid)
    pub authority: AuthorityLevel,
    /// Node capabilities and services offered
    pub capabilities: Vec<NodeCapability>,
    /// Network endpoints for communication
    pub endpoints: NetworkEndpoints,
    /// Stake amount (for validators and enterprise nodes)
    pub stake: Option<u64>,
    /// Node reputation score
    pub reputation: ReputationScore,
    /// Current node status
    pub status: NodeStatus,
    /// Node metadata
    pub metadata: NodeMetadata,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
}

impl NodeRegistration {
    /// Create a new node registration
    pub fn new(
        node_type: NodeType,
        identity: IdentityProof,
        authority: AuthorityLevel,
        endpoints: NetworkEndpoints,
    ) -> Self {
        let now = Utc::now();
        Self {
            node_id: None, // Will be set during registration
            node_type,
            identity,
            authority,
            capabilities: Vec::new(),
            endpoints,
            stake: None,
            reputation: ReputationScore::new(),
            status: NodeStatus::Onboarding,
            metadata: NodeMetadata::new(),
            registered_at: now,
            last_activity: now,
        }
    }

    /// Add a capability to the node
    pub fn add_capability(&mut self, capability: NodeCapability) {
        self.capabilities.push(capability);
    }

    /// Set stake amount for validator/enterprise nodes
    pub fn set_stake(&mut self, stake_amount: u64) {
        self.stake = Some(stake_amount);
    }

    /// Update node status
    pub fn update_status(&mut self, new_status: NodeStatus) {
        self.status = new_status;
        self.last_activity = Utc::now();
    }

    /// Check if node is eligible for validator role
    pub fn is_validator_eligible(&self) -> bool {
        match &self.node_type {
            NodeType::BpciEnterprise { validator, .. } => *validator,
            NodeType::Hybrid { .. } => true, // Hybrid nodes can be validators
            _ => false,
        }
    }

    /// Check if node is eligible for mining
    pub fn is_miner_eligible(&self) -> bool {
        match &self.node_type {
            NodeType::BpciEnterprise { miner, .. } => *miner,
            _ => false,
        }
    }

    /// Check if node is eligible for notary committee
    pub fn is_notary_eligible(&self) -> bool {
        match &self.node_type {
            NodeType::BpciEnterprise { notary_committee, .. } => *notary_committee,
            _ => false,
        }
    }

    /// Get minimum stake required for this node type
    pub fn minimum_stake_required(&self) -> u64 {
        match &self.node_type {
            NodeType::BpiCommunity { .. } => 0, // No stake required
            NodeType::BpciEnterprise { validator, miner, notary_committee, .. } => {
                let mut min_stake = 0;
                if *validator { min_stake = min_stake.max(1000000); } // 1M tokens
                if *miner { min_stake = min_stake.max(500000); } // 500K tokens
                if *notary_committee { min_stake = min_stake.max(2000000); } // 2M tokens
                min_stake
            },
            NodeType::Hybrid { .. } => 500000, // 500K tokens for hybrid
            NodeType::BankApiRegistry { .. } => 10000000, // 10M tokens for bank registry (highest security)
            NodeType::GovernmentApiRegistry { .. } => 15000000, // 15M tokens for government registry (highest authority)
            NodeType::RoundtableApi { .. } => 5000000, // 5M tokens for governance coordination
        }
    }

    /// Validate registration completeness
    pub fn validate(&self) -> Result<()> {
        // Check identity proof
        if !self.identity.verify_crypto_proof() {
            return Err(anyhow::anyhow!("Invalid cryptographic proof"));
        }

        // Check authority level requirements
        let trust_score = self.authority.trust_score();
        let min_trust_required = match &self.node_type {
            NodeType::BpiCommunity { .. } => 100,
            NodeType::BpciEnterprise { .. } => 500,
            NodeType::Hybrid { .. } => 300,
            NodeType::BankApiRegistry { .. } => 900, // Highest trust for bank operations
            NodeType::GovernmentApiRegistry { .. } => 950, // Maximum trust for government operations
            NodeType::RoundtableApi { .. } => 800, // High trust for governance coordination
        };

        if trust_score < min_trust_required {
            return Err(anyhow::anyhow!(
                "Insufficient trust score: {} < {}", 
                trust_score, 
                min_trust_required
            ));
        }

        // Check stake requirements
        let min_stake = self.minimum_stake_required();
        if min_stake > 0 {
            match self.stake {
                Some(stake) if stake >= min_stake => {},
                Some(stake) => return Err(anyhow::anyhow!(
                    "Insufficient stake: {} < {}", 
                    stake, 
                    min_stake
                )),
                None => return Err(anyhow::anyhow!(
                    "Stake required: {}", 
                    min_stake
                )),
            }
        }

        // Check capabilities
        if self.capabilities.is_empty() {
            return Err(anyhow::anyhow!("At least one capability must be specified"));
        }

        // Check endpoints
        if self.endpoints.primary.is_empty() {
            return Err(anyhow::anyhow!("Primary endpoint must be specified"));
        }

        Ok(())
    }
}

/// Node metadata for additional information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Human-readable node name
    pub name: Option<String>,
    /// Node description
    pub description: Option<String>,
    /// Geographic location
    pub location: Option<GeographicLocation>,
    /// Hardware specifications
    pub hardware: Option<HardwareSpecs>,
    /// Software versions
    pub software: Option<SoftwareVersions>,
    /// Operational metrics
    pub metrics: Option<OperationalMetrics>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom metadata fields
    pub custom_fields: std::collections::HashMap<String, String>,
}

impl NodeMetadata {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            location: None,
            hardware: None,
            software: None,
            metrics: None,
            tags: Vec::new(),
            custom_fields: std::collections::HashMap::new(),
        }
    }
}

/// Geographic location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicLocation {
    pub country: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
}

/// Hardware specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSpecs {
    pub cpu_cores: u32,
    pub cpu_model: Option<String>,
    pub memory_gb: u32,
    pub storage_gb: u64,
    pub network_bandwidth_mbps: u32,
    pub gpu_count: Option<u32>,
    pub gpu_model: Option<String>,
}

/// Software versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareVersions {
    pub metanode_version: String,
    pub bpci_version: String,
    pub os_version: String,
    pub docker_version: Option<String>,
    pub kubernetes_version: Option<String>,
}

/// Operational metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalMetrics {
    pub uptime_percentage: f64,
    pub average_response_time_ms: u32,
    pub total_requests_processed: u64,
    pub error_rate_percentage: f64,
    pub last_maintenance: DateTime<Utc>,
    pub next_maintenance_scheduled: Option<DateTime<Utc>>,
}

/// Registration request structure for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Node type to register
    pub node_type: NodeTypeRequest,
    /// Identity information
    pub identity: IdentityRequest,
    /// Authority information
    pub authority: AuthorityRequest,
    /// Network endpoints
    pub endpoints: NetworkEndpoints,
    /// Capabilities to register
    pub capabilities: Vec<NodeCapability>,
    /// Stake amount (if applicable)
    pub stake: Option<u64>,
    /// Optional metadata
    pub metadata: Option<NodeMetadata>,
}

/// Simplified node type for registration requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeTypeRequest {
    BpiCommunity {
        app_hosting: bool,
        community_governance: bool,
    },
    BpciEnterprise {
        validator: bool,
        miner: bool,
        notary_committee: bool,
        banking_compliance: bool,
    },
    Hybrid {
        bank_sponsored: bool,
        community_operated: bool,
    },
}

/// Identity request for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRequest {
    pub did: String,
    pub dadhaar_id: Option<String>,
    pub dpan_id: Option<String>,
    pub public_key: String,
    pub signature: String,
}

/// Authority request for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityRequest {
    Community {
        reputation_score: u32,
        community_vouchers: Vec<String>,
    },
    Bank {
        bank_name: String,
        bank_id: String,
        kyc_verified: bool,
        aml_compliant: bool,
    },
    Hybrid {
        bank_name: String,
        bank_id: String,
        community_reputation: u32,
    },
}

/// Registration response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    /// Generated node ID
    pub node_id: String,
    /// Registration status
    pub status: RegistrationStatus,
    /// Verification requirements
    pub verification_required: Vec<VerificationRequirement>,
    /// Next steps for the registrant
    pub next_steps: Vec<String>,
    /// Estimated time to complete registration
    pub estimated_completion_time: String,
    /// Support contact information
    pub support_contact: Option<String>,
}

/// Registration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Registration submitted and pending review
    Pending,
    /// Registration approved and active
    Approved,
    /// Registration rejected
    Rejected { reason: String },
    /// Additional verification required
    VerificationRequired,
    /// Registration in progress
    InProgress,
}

/// Verification requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationRequirement {
    /// Identity verification required
    Identity {
        verification_type: String,
        deadline: DateTime<Utc>,
    },
    /// Authority verification required
    Authority {
        authority_type: String,
        required_documents: Vec<String>,
    },
    /// Technical verification required
    Technical {
        capability_tests: Vec<String>,
        performance_requirements: Vec<String>,
    },
    /// Stake verification required
    Stake {
        required_amount: u64,
        escrow_address: String,
    },
}

/// Registration service for handling node registrations
#[derive(Debug)]
pub struct RegistrationService {
    /// Pending registrations
    pub pending_registrations: std::collections::HashMap<String, NodeRegistration>,
    /// Approved registrations
    pub approved_registrations: std::collections::HashMap<String, NodeRegistration>,
    /// Registration statistics
    pub stats: RegistrationStats,
}

impl RegistrationService {
    /// Create a new registration service
    pub fn new() -> Self {
        Self {
            pending_registrations: std::collections::HashMap::new(),
            approved_registrations: std::collections::HashMap::new(),
            stats: RegistrationStats::new(),
        }
    }

    /// Process a new registration request
    pub async fn process_registration(
        &mut self, 
        request: RegistrationRequest
    ) -> Result<RegistrationResponse> {
        // Convert request to internal registration format
        let mut registration = self.convert_request_to_registration(request)?;
        
        // Validate registration
        registration.validate()?;
        
        // Generate node ID
        let node_id = self.generate_node_id(&registration);
        registration.node_id = Some(node_id.clone());
        
        // Determine verification requirements
        let verification_required = self.determine_verification_requirements(&registration);
        
        // Create response before moving registration
        let next_steps = self.generate_next_steps(&registration);
        let estimated_completion_time = self.estimate_completion_time(&registration);
        
        // Store pending registration
        self.pending_registrations.insert(node_id.clone(), registration);
        
        // Update statistics
        self.stats.total_registrations += 1;
        self.stats.pending_registrations += 1;
        
        // Create response
        let response = RegistrationResponse {
            node_id,
            status: if verification_required.is_empty() {
                RegistrationStatus::Approved
            } else {
                RegistrationStatus::VerificationRequired
            },
            verification_required,
            next_steps,
            estimated_completion_time,
            support_contact: Some("support@bpci.io".to_string()),
        };
        
        Ok(response)
    }

    /// Approve a pending registration
    pub async fn approve_registration(&mut self, node_id: &str) -> Result<()> {
        if let Some(registration) = self.pending_registrations.remove(node_id) {
            self.approved_registrations.insert(node_id.to_string(), registration);
            self.stats.pending_registrations -= 1;
            self.stats.approved_registrations += 1;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Registration not found: {}", node_id))
        }
    }

    /// Reject a pending registration
    pub async fn reject_registration(&mut self, node_id: &str, reason: String) -> Result<()> {
        if self.pending_registrations.remove(node_id).is_some() {
            self.stats.pending_registrations -= 1;
            self.stats.rejected_registrations += 1;
            // Could store rejection reason in a separate structure
            Ok(())
        } else {
            Err(anyhow::anyhow!("Registration not found: {}", node_id))
        }
    }

    fn convert_request_to_registration(&self, request: RegistrationRequest) -> Result<NodeRegistration> {
        // Convert request structures to internal structures
        // This is a simplified conversion - in production, would include proper validation
        
        let node_type = match request.node_type {
            NodeTypeRequest::BpiCommunity { app_hosting, community_governance } => {
                NodeType::BpiCommunity {
                    app_hosting,
                    community_governance,
                    max_apps: Some(10), // Default
                    supported_app_types: vec![], // Would be populated based on capabilities
                }
            },
            NodeTypeRequest::BpciEnterprise { validator, miner, notary_committee, banking_compliance } => {
                NodeType::BpciEnterprise {
                    validator,
                    miner,
                    notary_committee,
                    banking_compliance,
                    enhanced_security: super::node_types::SecurityLevel::Standard,
                    regulatory_compliance: vec![],
                }
            },
            NodeTypeRequest::Hybrid { bank_sponsored, community_operated } => {
                NodeType::Hybrid {
                    bank_sponsored,
                    community_operated,
                    dual_authority: true,
                    bank_sponsor: None, // Would be populated from authority request
                    community_operator: None,
                }
            },
        };

        let identity = IdentityProof::new(request.identity.did);
        let authority = AuthorityLevel::Community {
            basic_verification: true,
            community_vouching: 0,
            reputation_score: 100,
            participation_years: 0,
            roles: vec![],
        };

        let mut registration = NodeRegistration::new(
            node_type,
            identity,
            authority,
            request.endpoints,
        );

        registration.capabilities = request.capabilities;
        registration.stake = request.stake;
        
        if let Some(metadata) = request.metadata {
            registration.metadata = metadata;
        }

        Ok(registration)
    }

    fn generate_node_id(&self, registration: &NodeRegistration) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&registration.identity.did);
        hasher.update(&registration.endpoints.primary);
        hasher.update(&Utc::now().timestamp().to_string());
        
        let hash = hasher.finalize();
        format!("node_{}", hex::encode(&hash[..8]))
    }

    fn determine_verification_requirements(&self, registration: &NodeRegistration) -> Vec<VerificationRequirement> {
        let mut requirements = Vec::new();
        
        // Check if identity verification is needed
        if registration.identity.verification_level == super::identity::VerificationLevel::Basic {
            requirements.push(VerificationRequirement::Identity {
                verification_type: "Enhanced KYC".to_string(),
                deadline: Utc::now() + chrono::Duration::days(7),
            });
        }
        
        // Check if stake verification is needed
        if let Some(stake) = registration.stake {
            if stake >= 1000000 { // Large stakes need verification
                requirements.push(VerificationRequirement::Stake {
                    required_amount: stake,
                    escrow_address: "escrow_address_placeholder".to_string(),
                });
            }
        }
        
        requirements
    }

    fn generate_next_steps(&self, registration: &NodeRegistration) -> Vec<String> {
        let mut steps = Vec::new();
        
        match &registration.node_type {
            NodeType::BpiCommunity { .. } => {
                steps.push("Complete community onboarding process".to_string());
                steps.push("Join community governance discussions".to_string());
            },
            NodeType::BpciEnterprise { validator, miner, notary_committee, .. } => {
                if *validator {
                    steps.push("Set up validator node infrastructure".to_string());
                    steps.push("Stake required tokens".to_string());
                }
                if *miner {
                    steps.push("Configure mining software".to_string());
                    steps.push("Join mining pool (optional)".to_string());
                }
                if *notary_committee {
                    steps.push("Complete notary committee application".to_string());
                    steps.push("Undergo background verification".to_string());
                }
            },
            NodeType::Hybrid { .. } => {
                steps.push("Coordinate with bank sponsor".to_string());
                steps.push("Set up community operation protocols".to_string());
            },
            NodeType::BankApiRegistry { .. } => {
                steps.push("Complete bank regulatory compliance verification".to_string());
                steps.push("Set up bank-stamped BPI infrastructure".to_string());
                steps.push("Configure autonomous economy protocols".to_string());
                steps.push("Implement enhanced security protocols".to_string());
            },
            NodeType::GovernmentApiRegistry { .. } => {
                steps.push("Complete government authority verification".to_string());
                steps.push("Set up government-stamped BPI infrastructure".to_string());
                steps.push("Configure jurisdictional management protocols".to_string());
                steps.push("Implement emergency response capabilities".to_string());
            },
            NodeType::RoundtableApi { .. } => {
                steps.push("Set up parliament-style governance infrastructure".to_string());
                steps.push("Configure voting and consensus mechanisms".to_string());
                steps.push("Implement audit and transparency features".to_string());
                steps.push("Establish coordination protocols".to_string());
            },
        }
        
        steps.push("Complete node health checks".to_string());
        steps.push("Begin network participation".to_string());
        
        steps
    }

    fn estimate_completion_time(&self, registration: &NodeRegistration) -> String {
        match &registration.node_type {
            NodeType::BpiCommunity { .. } => "1-2 days".to_string(),
            NodeType::BpciEnterprise { .. } => "1-2 weeks".to_string(),
            NodeType::Hybrid { .. } => "2-4 weeks".to_string(),
            NodeType::BankApiRegistry { .. } => "4-8 weeks".to_string(), // Extended time for regulatory compliance
            NodeType::GovernmentApiRegistry { .. } => "6-12 weeks".to_string(), // Longest time for government verification
            NodeType::RoundtableApi { .. } => "3-6 weeks".to_string(), // Moderate time for governance setup
        }
    }
}

/// Registration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationStats {
    pub total_registrations: u64,
    pub pending_registrations: u64,
    pub approved_registrations: u64,
    pub rejected_registrations: u64,
    pub bpi_community_registrations: u64,
    pub bpci_enterprise_registrations: u64,
    pub hybrid_registrations: u64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl RegistrationStats {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            total_registrations: 0,
            pending_registrations: 0,
            approved_registrations: 0,
            rejected_registrations: 0,
            bpi_community_registrations: 0,
            bpci_enterprise_registrations: 0,
            hybrid_registrations: 0,
            created_at: now,
            last_updated: now,
        }
    }
}
