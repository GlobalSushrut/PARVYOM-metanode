use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Dispute Resolution System
#[derive(Debug)]
pub struct DisputeResolver {
    disputes: Arc<RwLock<HashMap<Uuid, Dispute>>>,
    arbitrators: Arc<RwLock<HashMap<Uuid, Arbitrator>>>,
    resolution_rules: Vec<ResolutionRule>,
}

/// Dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: Uuid,
    pub document_id: Uuid,
    pub parties: Vec<String>,
    pub reason: DisputeReason,
    pub evidence: Vec<Evidence>,
    pub status: DisputeStatus,
    pub created_at: DateTime<Utc>,
    pub arbitrator_id: Option<Uuid>,
}

/// Dispute Reason
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisputeReason {
    DocumentAuthenticity,
    NotaryMisconduct,
    FraudulentNotarization,
    IdentityDispute,
    Other(String),
}

/// Evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub evidence_type: EvidenceType,
    pub data: String,
    pub submitted_by: String,
    pub submitted_at: DateTime<Utc>,
}

/// Evidence Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Document,
    Testimony,
    ExpertOpinion,
    TechnicalAnalysis,
    Other(String),
}

/// Dispute Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisputeStatus {
    Open,
    UnderReview,
    Resolved,
    Dismissed,
}

/// Arbitrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arbitrator {
    pub id: Uuid,
    pub name: String,
    pub qualifications: Vec<String>,
    pub specializations: Vec<String>,
    pub reputation_score: f64,
    pub active_cases: u32,
}

/// Resolution Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionRule {
    pub id: String,
    pub description: String,
    pub applicable_disputes: Vec<DisputeReason>,
    pub procedure: String,
}

impl DisputeResolver {
    /// Create a new dispute resolver
    pub fn new() -> Self {
        Self {
            disputes: Arc::new(RwLock::new(HashMap::new())),
            arbitrators: Arc::new(RwLock::new(HashMap::new())),
            resolution_rules: Self::default_resolution_rules(),
        }
    }

    /// Default resolution rules
    fn default_resolution_rules() -> Vec<ResolutionRule> {
        vec![
            ResolutionRule {
                id: "authenticity_dispute".to_string(),
                description: "Rules for document authenticity disputes".to_string(),
                applicable_disputes: vec![DisputeReason::DocumentAuthenticity],
                procedure: "Technical analysis and expert review".to_string(),
            },
            ResolutionRule {
                id: "notary_misconduct".to_string(),
                description: "Rules for notary misconduct disputes".to_string(),
                applicable_disputes: vec![DisputeReason::NotaryMisconduct],
                procedure: "Professional review and disciplinary action".to_string(),
            },
        ]
    }

    /// File a dispute
    pub async fn file_dispute(&self, dispute: Dispute) -> Result<Uuid> {
        let mut disputes = self.disputes.write().await;
        let dispute_id = dispute.id;
        disputes.insert(dispute_id, dispute);
        Ok(dispute_id)
    }

    /// Resolve a dispute
    pub async fn resolve_dispute(&self, dispute_id: Uuid, _resolution: String) -> Result<()> {
        let mut disputes = self.disputes.write().await;
        if let Some(dispute) = disputes.get_mut(&dispute_id) {
            dispute.status = DisputeStatus::Resolved;
        }
        Ok(())
    }

    /// Get dispute by ID
    pub async fn get_dispute(&self, dispute_id: Uuid) -> Option<Dispute> {
        self.disputes.read().await.get(&dispute_id).cloned()
    }

    /// List all disputes
    pub async fn list_disputes(&self) -> Vec<Uuid> {
        self.disputes.read().await.keys().cloned().collect()
    }

    /// Add arbitrator
    pub async fn add_arbitrator(&self, arbitrator: Arbitrator) -> Result<()> {
        let mut arbitrators = self.arbitrators.write().await;
        arbitrators.insert(arbitrator.id, arbitrator);
        Ok(())
    }

    /// Get arbitrator by ID
    pub async fn get_arbitrator(&self, arbitrator_id: Uuid) -> Option<Arbitrator> {
        self.arbitrators.read().await.get(&arbitrator_id).cloned()
    }
}
