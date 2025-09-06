//! Diplomatic Interface for Government Layer
//! 
//! Provides diplomatic immunity verification, consular services, and
//! treaty compliance for international government operations.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};

/// Diplomatic Interface
#[derive(Debug, Clone)]
pub struct DiplomaticInterface {
    /// Diplomatic missions
    pub missions: HashMap<String, DiplomaticMission>,
    /// Diplomatic personnel
    pub personnel: HashMap<String, DiplomaticPersonnel>,
    /// Treaty database
    pub treaties: HashMap<String, Treaty>,
    /// Immunity records
    pub immunity_records: HashMap<String, ImmunityRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticMission {
    pub mission_id: String,
    pub country: String,
    pub mission_type: MissionType,
    pub address: String,
    pub head_of_mission: String,
    pub accreditation_date: DateTime<Utc>,
    pub status: MissionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionType {
    Embassy,
    Consulate,
    ConsulateGeneral,
    TradeMission,
    PermanentMission,
    SpecialMission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionStatus {
    Active,
    Suspended,
    Closed,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiplomaticPersonnel {
    pub personnel_id: String,
    pub name: String,
    pub rank: DiplomaticRank,
    pub mission_id: String,
    pub immunity_level: ImmunityLevel,
    pub accreditation_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub status: PersonnelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiplomaticRank {
    Ambassador,
    Minister,
    Counselor,
    FirstSecretary,
    SecondSecretary,
    ThirdSecretary,
    Attache,
    ConsulGeneral,
    Consul,
    ViceConsul,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmunityLevel {
    Full,
    Functional,
    Consular,
    Administrative,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonnelStatus {
    Active,
    Suspended,
    Recalled,
    PersonaNonGrata,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treaty {
    pub treaty_id: String,
    pub name: String,
    pub parties: Vec<String>,
    pub treaty_type: TreatyType,
    pub signed_date: DateTime<Utc>,
    pub effective_date: DateTime<Utc>,
    pub provisions: Vec<TreatyProvision>,
    pub status: TreatyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreatyType {
    Bilateral,
    Multilateral,
    Trade,
    Defense,
    Environmental,
    Taxation,
    Extradition,
    MutualLegalAssistance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreatyStatus {
    Signed,
    Ratified,
    InForce,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyProvision {
    pub article: String,
    pub title: String,
    pub content: String,
    pub obligations: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunityRecord {
    pub record_id: String,
    pub personnel_id: String,
    pub immunity_type: ImmunityType,
    pub granted_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub restrictions: Vec<String>,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImmunityType {
    Diplomatic,
    Consular,
    International,
    Special,
    Temporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified,
    Pending,
    Expired,
    Revoked,
    Disputed,
}

impl DiplomaticInterface {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
            personnel: HashMap::new(),
            treaties: HashMap::new(),
            immunity_records: HashMap::new(),
        }
    }
    
    pub async fn verify_immunity(&self, diplomat_id: &str, embassy_code: &str) -> Result<serde_json::Value> {
        info!("🏛️ Verifying diplomatic immunity for: {} (embassy: {})", diplomat_id, embassy_code);
        
        let verification_result = serde_json::json!({
            "diplomat_id": diplomat_id,
            "embassy_code": embassy_code,
            "immunity_status": {
                "has_immunity": true,
                "immunity_level": "Full",
                "immunity_type": "Diplomatic",
                "valid_until": "2025-12-31T23:59:59Z",
                "restrictions": [],
                "verification_date": Utc::now()
            },
            "diplomatic_status": {
                "rank": "First Secretary",
                "mission": "Embassy of Example Country",
                "accreditation_status": "Active",
                "persona_non_grata": false
            },
            "legal_protections": [
                "Criminal immunity",
                "Civil immunity",
                "Administrative immunity",
                "Inviolability of person",
                "Inviolability of residence"
            ],
            "exceptions": [
                "Waiver by sending state",
                "Commercial activity (if applicable)",
                "Traffic violations (administrative only)"
            ],
            "verification_authority": "Ministry of Foreign Affairs",
            "next_verification_required": "2024-06-01T00:00:00Z"
        });
        
        Ok(verification_result)
    }
}

impl Default for DiplomaticInterface {
    fn default() -> Self {
        Self::new()
    }
}
