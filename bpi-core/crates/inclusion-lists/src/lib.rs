use std::collections::{HashMap, HashSet};
use std::time::Instant;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use lru::LruCache;
use prometheus::{Counter, Gauge, Histogram, HistogramOpts};
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, info, warn};

// Re-export types from other crates
pub use bpi_enc::domain_hash;
pub use bpi_headers::HeaderHash;
pub use bpi_mempool::{Transaction, TxId};

/// Domain constants for inclusion lists
const INCLUSION_LIST_ROOT: u8 = 0x22;
const SLASHING_EVIDENCE: u8 = 0x23;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionListConfig {
    pub max_pending_obligations: usize,
    pub obligation_timeout_blocks: u64,
    pub max_list_size: usize,
    pub enforcement_window_blocks: u64,
    pub slashing_evidence_retention_blocks: u64,
    pub validator_check_interval_ms: u64,
}

impl Default for InclusionListConfig {
    fn default() -> Self {
        Self {
            max_pending_obligations: 10000,
            obligation_timeout_blocks: 32, // ~6.4 minutes at 12s blocks
            max_list_size: 1000,
            enforcement_window_blocks: 8, // ~1.6 minutes
            slashing_evidence_retention_blocks: 256, // ~51 minutes
            validator_check_interval_ms: 6000, // 6 seconds
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObligationId(pub [u8; 32]);

impl ObligationId {
    pub fn new(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        use rand::Rng;
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Self(bytes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingObligation {
    pub id: ObligationId,
    pub tx_id: TxId,
    pub proposer: Vec<u8>,
    pub created_block: u64,
    pub deadline_block: u64,
    pub obligation_type: ObligationType,
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObligationType {
    TransactionInclusion,
    DataAvailability,
    ValidatorAttestation,
    SlashingEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionList {
    pub block_number: u64,
    pub proposer: Vec<u8>,
    pub obligations: Vec<ObligationId>,
    pub inc_root: [u8; 32],
    pub timestamp: DateTime<Utc>,
}

impl InclusionList {
    pub fn new(block_number: u64, proposer: Vec<u8>, obligations: Vec<ObligationId>) -> Self {
        let inc_root = Self::compute_root(&obligations);
        Self {
            block_number,
            proposer,
            obligations,
            inc_root,
            timestamp: Utc::now(),
        }
    }

    pub fn compute_root(obligations: &[ObligationId]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&domain_hash(INCLUSION_LIST_ROOT, &[]));
        
        for obligation in obligations {
            hasher.update(&obligation.0);
        }
        
        *hasher.finalize().as_bytes()
    }

    pub fn verify_root(&self) -> bool {
        let computed_root = Self::compute_root(&self.obligations);
        computed_root == self.inc_root
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingItemEvidence {
    pub obligation_id: ObligationId,
    pub expected_block: u64,
    pub actual_block: Option<u64>,
    pub proposer: Vec<u8>,
    pub evidence_type: EvidenceType,
    pub proof: Vec<u8>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    MissingTransaction,
    IncorrectInclusion,
    DeadlineViolation,
    InvalidProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEvidence {
    pub evidence_id: [u8; 32],
    pub validator: Vec<u8>,
    pub violation_type: ViolationType,
    pub missing_items: Vec<MissingItemEvidence>,
    pub block_range: (u64, u64),
    pub severity_score: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    FailedInclusion,
    InvalidList,
    DeadlineViolation,
    MaliciousOmission,
}

impl SlashingEvidence {
    pub fn new(
        validator: Vec<u8>,
        violation_type: ViolationType,
        missing_items: Vec<MissingItemEvidence>,
        block_range: (u64, u64),
    ) -> Self {
        let severity_score = Self::calculate_severity(&missing_items);
        let evidence_data = bincode::serialize(&(&validator, &violation_type, &missing_items, &block_range)).unwrap();
        let evidence_id = domain_hash(SLASHING_EVIDENCE, &evidence_data);

        Self {
            evidence_id,
            validator,
            violation_type,
            missing_items,
            block_range,
            severity_score,
            timestamp: Utc::now(),
        }
    }

    fn calculate_severity(missing_items: &[MissingItemEvidence]) -> u32 {
        missing_items.len() as u32 * 10 + 
        missing_items.iter().map(|item| match item.evidence_type {
            EvidenceType::MissingTransaction => 5,
            EvidenceType::IncorrectInclusion => 10,
            EvidenceType::DeadlineViolation => 15,
            EvidenceType::InvalidProposal => 20,
        }).sum::<u32>()
    }
}

#[derive(Debug)]
pub struct InclusionListMetrics {
    pub pending_obligations: Gauge,
    pub active_lists: Gauge,
    pub missing_items_detected: Counter,
    pub slashing_evidence_generated: Counter,
    pub enforcement_checks: Counter,
    pub obligation_timeouts: Counter,
    pub list_verification_duration: Histogram,
}

impl InclusionListMetrics {
    pub fn new() -> Self {
        Self {
            pending_obligations: Gauge::new("inclusion_pending_obligations", "Number of pending obligations").unwrap(),
            active_lists: Gauge::new("inclusion_active_lists", "Number of active inclusion lists").unwrap(),
            missing_items_detected: Counter::new("inclusion_missing_items_total", "Total missing items detected").unwrap(),
            slashing_evidence_generated: Counter::new("inclusion_slashing_evidence_total", "Total slashing evidence generated").unwrap(),
            enforcement_checks: Counter::new("inclusion_enforcement_checks_total", "Total enforcement checks").unwrap(),
            obligation_timeouts: Counter::new("inclusion_obligation_timeouts_total", "Total obligation timeouts").unwrap(),
            list_verification_duration: prometheus::Histogram::with_opts(
                HistogramOpts::new("inclusion_list_verification_duration_seconds", "List verification duration")
            ).unwrap(),
        }
    }
}

pub struct InclusionListManager {
    config: InclusionListConfig,
    pending_obligations: RwLock<HashMap<ObligationId, PendingObligation>>,
    active_lists: RwLock<HashMap<u64, InclusionList>>,
    slashing_evidence: RwLock<HashMap<[u8; 32], SlashingEvidence>>,
    proposer_obligations: RwLock<HashMap<Vec<u8>, HashSet<ObligationId>>>,
    block_obligations: RwLock<HashMap<u64, HashSet<ObligationId>>>,
    missing_items_cache: Mutex<LruCache<ObligationId, MissingItemEvidence>>,
    metrics: InclusionListMetrics,
    current_block: RwLock<u64>,
}

impl InclusionListManager {
    pub fn new(config: InclusionListConfig) -> Self {
        let missing_items_cache = LruCache::new(
            std::num::NonZeroUsize::new(config.max_pending_obligations).unwrap()
        );

        Self {
            config,
            pending_obligations: RwLock::new(HashMap::new()),
            active_lists: RwLock::new(HashMap::new()),
            slashing_evidence: RwLock::new(HashMap::new()),
            proposer_obligations: RwLock::new(HashMap::new()),
            block_obligations: RwLock::new(HashMap::new()),
            missing_items_cache: Mutex::new(missing_items_cache),
            metrics: InclusionListMetrics::new(),
            current_block: RwLock::new(0),
        }
    }

    // Stage 22.1: List Maintenance (pending obligations, inc_root)
    pub async fn add_pending_obligation(&self, obligation: PendingObligation) -> Result<()> {
        let mut pending = self.pending_obligations.write().await;
        
        if pending.len() >= self.config.max_pending_obligations {
            return Err(anyhow!("Maximum pending obligations reached"));
        }

        // Add to proposer tracking
        {
            let mut proposer_obligations = self.proposer_obligations.write().await;
            proposer_obligations.entry(obligation.proposer.clone())
                .or_insert_with(HashSet::new)
                .insert(obligation.id.clone());
        }

        // Add to block tracking
        {
            let mut block_obligations = self.block_obligations.write().await;
            block_obligations.entry(obligation.deadline_block)
                .or_insert_with(HashSet::new)
                .insert(obligation.id.clone());
        }

        let obligation_id = obligation.id.clone();
        pending.insert(obligation.id.clone(), obligation);
        self.metrics.pending_obligations.set(pending.len() as f64);

        debug!("Added pending obligation: {:?}", obligation_id);
        Ok(())
    }

    pub async fn create_inclusion_list(
        &self,
        block_number: u64,
        proposer: Vec<u8>,
        obligation_ids: Vec<ObligationId>,
    ) -> Result<InclusionList> {
        if obligation_ids.len() > self.config.max_list_size {
            return Err(anyhow!("Inclusion list too large: {} items", obligation_ids.len()));
        }

        let list = InclusionList::new(block_number, proposer, obligation_ids);

        // Store the list
        {
            let mut active_lists = self.active_lists.write().await;
            active_lists.insert(block_number, list.clone());
        }

        self.metrics.active_lists.set(self.active_lists.read().await.len() as f64);
        info!("Created inclusion list for block {} with {} obligations", block_number, list.obligations.len());

        Ok(list)
    }

    pub async fn verify_inclusion_list(&self, list: &InclusionList) -> Result<bool> {
        let start = Instant::now();

        // Verify root computation
        if !list.verify_root() {
            warn!("Inclusion list root verification failed for block {}", list.block_number);
            return Ok(false);
        }

        // Verify all obligations exist
        let pending = self.pending_obligations.read().await;
        for obligation_id in &list.obligations {
            if !pending.contains_key(obligation_id) {
                warn!("Unknown obligation in inclusion list: {:?}", obligation_id);
                return Ok(false);
            }
        }

        self.metrics.list_verification_duration.observe(start.elapsed().as_secs_f64());
        Ok(true)
    }

    // Stage 22.2: Enforcement Logic (missing item detection, evidence)
    pub async fn detect_missing_items(&self, block_number: u64) -> Result<Vec<MissingItemEvidence>> {
        let mut missing_items = Vec::new();
        
        let pending = self.pending_obligations.read().await;
        let active_lists = self.active_lists.read().await;

        // Check if there's an inclusion list for this block
        if let Some(list) = active_lists.get(&block_number) {
            // Find obligations that should have been included but weren't
            let included_obligations: HashSet<_> = list.obligations.iter().collect();
            
            for (obligation_id, obligation) in pending.iter() {
                if obligation.deadline_block <= block_number && !included_obligations.contains(obligation_id) {
                    let evidence = MissingItemEvidence {
                        obligation_id: obligation_id.clone(),
                        expected_block: obligation.deadline_block,
                        actual_block: None,
                        proposer: list.proposer.clone(),
                        evidence_type: EvidenceType::MissingTransaction,
                        proof: self.generate_missing_item_proof(obligation).await?,
                        timestamp: Utc::now(),
                    };
                    missing_items.push(evidence);
                }
            }
        } else {
            // No inclusion list for this block - check for overdue obligations
            for (obligation_id, obligation) in pending.iter() {
                if obligation.deadline_block <= block_number {
                    let evidence = MissingItemEvidence {
                        obligation_id: obligation_id.clone(),
                        expected_block: obligation.deadline_block,
                        actual_block: None,
                        proposer: obligation.proposer.clone(),
                        evidence_type: EvidenceType::DeadlineViolation,
                        proof: self.generate_missing_item_proof(obligation).await?,
                        timestamp: Utc::now(),
                    };
                    missing_items.push(evidence);
                }
            }
        }

        if !missing_items.is_empty() {
            self.metrics.missing_items_detected.inc_by(missing_items.len() as f64);
            info!("Detected {} missing items for block {}", missing_items.len(), block_number);
        }

        Ok(missing_items)
    }

    async fn generate_missing_item_proof(&self, obligation: &PendingObligation) -> Result<Vec<u8>> {
        // Generate cryptographic proof of missing item
        let proof_data = bincode::serialize(&(
            &obligation.id,
            &obligation.tx_id,
            &obligation.deadline_block,
            &obligation.timestamp,
        ))?;
        
        Ok(domain_hash(SLASHING_EVIDENCE, &proof_data).to_vec())
    }

    pub async fn generate_slashing_evidence(
        &self,
        validator: Vec<u8>,
        missing_items: Vec<MissingItemEvidence>,
        block_range: (u64, u64),
    ) -> Result<SlashingEvidence> {
        if missing_items.is_empty() {
            return Err(anyhow!("No missing items provided for slashing evidence"));
        }

        let violation_type = self.determine_violation_type(&missing_items);
        let evidence = SlashingEvidence::new(validator, violation_type, missing_items, block_range);

        // Store the evidence
        {
            let mut slashing_evidence = self.slashing_evidence.write().await;
            slashing_evidence.insert(evidence.evidence_id, evidence.clone());
        }

        // Cache missing items
        {
            let mut cache = self.missing_items_cache.lock().await;
            for item in &evidence.missing_items {
                cache.put(item.obligation_id.clone(), item.clone());
            }
        }

        self.metrics.slashing_evidence_generated.inc();
        info!("Generated slashing evidence for validator with severity {}", evidence.severity_score);

        Ok(evidence)
    }

    fn determine_violation_type(&self, missing_items: &[MissingItemEvidence]) -> ViolationType {
        let has_missing = missing_items.iter().any(|item| matches!(item.evidence_type, EvidenceType::MissingTransaction));
        let has_deadline = missing_items.iter().any(|item| matches!(item.evidence_type, EvidenceType::DeadlineViolation));
        let has_invalid = missing_items.iter().any(|item| matches!(item.evidence_type, EvidenceType::InvalidProposal));

        if has_invalid {
            ViolationType::InvalidList
        } else if has_deadline {
            ViolationType::DeadlineViolation
        } else if has_missing {
            ViolationType::FailedInclusion
        } else {
            ViolationType::MaliciousOmission
        }
    }

    // Stage 22.3: Integration (proposer requirements, validator checking)
    pub async fn get_proposer_requirements(&self, proposer: &[u8], block_number: u64) -> Vec<ObligationId> {
        let proposer_obligations = self.proposer_obligations.read().await;
        let pending = self.pending_obligations.read().await;

        proposer_obligations
            .get(proposer)
            .map(|obligations| {
                obligations
                    .iter()
                    .filter(|obligation_id| {
                        if let Some(obligation) = pending.get(obligation_id) {
                            obligation.deadline_block <= block_number + self.config.enforcement_window_blocks
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub async fn validate_proposer_compliance(
        &self,
        proposer: &[u8],
        block_number: u64,
        included_obligations: &[ObligationId],
    ) -> Result<bool> {
        self.metrics.enforcement_checks.inc();

        let required_obligations = self.get_proposer_requirements(proposer, block_number).await;
        let included_set: HashSet<_> = included_obligations.iter().collect();

        for required_obligation in &required_obligations {
            if !included_set.contains(required_obligation) {
                warn!("Proposer failed to include required obligation: {:?}", required_obligation);
                return Ok(false);
            }
        }

        debug!("Proposer compliance validated for block {}", block_number);
        Ok(true)
    }

    pub async fn cleanup_expired_obligations(&self, current_block: u64) -> Result<usize> {
        let mut removed_count = 0;
        
        // Update current block
        {
            let mut current = self.current_block.write().await;
            *current = current_block;
        }

        // Remove expired obligations
        {
            let mut pending = self.pending_obligations.write().await;
            let mut proposer_obligations = self.proposer_obligations.write().await;
            let mut block_obligations = self.block_obligations.write().await;

            let expired_cutoff = current_block.saturating_sub(self.config.obligation_timeout_blocks);
            
            pending.retain(|obligation_id, obligation| {
                let is_expired = obligation.deadline_block < expired_cutoff;
                if is_expired {
                    // Remove from proposer tracking
                    if let Some(proposer_set) = proposer_obligations.get_mut(&obligation.proposer) {
                        proposer_set.remove(obligation_id);
                        if proposer_set.is_empty() {
                            proposer_obligations.remove(&obligation.proposer);
                        }
                    }
                    
                    // Remove from block tracking
                    if let Some(block_set) = block_obligations.get_mut(&obligation.deadline_block) {
                        block_set.remove(obligation_id);
                        if block_set.is_empty() {
                            block_obligations.remove(&obligation.deadline_block);
                        }
                    }
                    
                    removed_count += 1;
                }
                !is_expired
            });
        }

        // Remove old inclusion lists
        {
            let mut active_lists = self.active_lists.write().await;
            let list_cutoff = current_block.saturating_sub(self.config.enforcement_window_blocks);
            active_lists.retain(|&block_num, _| block_num >= list_cutoff);
        }

        // Remove old slashing evidence
        {
            let mut slashing_evidence = self.slashing_evidence.write().await;
            let evidence_cutoff = current_block.saturating_sub(self.config.slashing_evidence_retention_blocks);
            slashing_evidence.retain(|_, evidence| {
                evidence.block_range.1 >= evidence_cutoff
            });
        }

        if removed_count > 0 {
            self.metrics.obligation_timeouts.inc_by(removed_count as f64);
            info!("Cleaned up {} expired obligations at block {}", removed_count, current_block);
        }

        self.metrics.pending_obligations.set(self.pending_obligations.read().await.len() as f64);
        self.metrics.active_lists.set(self.active_lists.read().await.len() as f64);

        Ok(removed_count)
    }

    pub async fn get_inclusion_list_stats(&self) -> InclusionListStats {
        let pending_count = self.pending_obligations.read().await.len();
        let active_lists_count = self.active_lists.read().await.len();
        let slashing_evidence_count = self.slashing_evidence.read().await.len();
        let current_block = *self.current_block.read().await;

        InclusionListStats {
            pending_obligations: pending_count,
            active_lists: active_lists_count,
            slashing_evidence_count,
            current_block,
            enforcement_active: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionListStats {
    pub pending_obligations: usize,
    pub active_lists: usize,
    pub slashing_evidence_count: usize,
    pub current_block: u64,
    pub enforcement_active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_obligation(block_number: u64) -> PendingObligation {
        PendingObligation {
            id: ObligationId::random(),
            tx_id: bpi_mempool::TxId::random(),
            proposer: b"test_proposer".to_vec(),
            created_block: block_number,
            deadline_block: block_number + 10,
            obligation_type: ObligationType::TransactionInclusion,
            data: b"test_data".to_vec(),
            timestamp: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_inclusion_list_creation() {
        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config);

        let obligation = create_test_obligation(100);
        manager.add_pending_obligation(obligation.clone()).await.unwrap();

        let list = manager.create_inclusion_list(
            105,
            b"proposer".to_vec(),
            vec![obligation.id.clone()],
        ).await.unwrap();

        assert_eq!(list.block_number, 105);
        assert_eq!(list.obligations.len(), 1);
        assert!(list.verify_root());

        println!("✅ Inclusion list creation working");
    }

    #[tokio::test]
    async fn test_missing_item_detection() {
        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config);

        let obligation = create_test_obligation(100);
        manager.add_pending_obligation(obligation.clone()).await.unwrap();

        // Create inclusion list without the obligation
        let _list = manager.create_inclusion_list(
            110,
            b"proposer".to_vec(),
            vec![], // Empty list - missing the obligation
        ).await.unwrap();

        let missing_items = manager.detect_missing_items(110).await.unwrap();
        assert_eq!(missing_items.len(), 1);
        assert_eq!(missing_items[0].obligation_id, obligation.id);

        println!("✅ Missing item detection working");
    }

    #[tokio::test]
    async fn test_slashing_evidence_generation() {
        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config);

        let obligation = create_test_obligation(100);
        manager.add_pending_obligation(obligation.clone()).await.unwrap();

        let missing_items = vec![MissingItemEvidence {
            obligation_id: obligation.id.clone(),
            expected_block: 110,
            actual_block: None,
            proposer: b"bad_proposer".to_vec(),
            evidence_type: EvidenceType::MissingTransaction,
            proof: vec![1, 2, 3, 4],
            timestamp: Utc::now(),
        }];

        let evidence = manager.generate_slashing_evidence(
            b"validator".to_vec(),
            missing_items,
            (100, 110),
        ).await.unwrap();

        assert_eq!(evidence.missing_items.len(), 1);
        assert!(evidence.severity_score > 0);
        assert!(matches!(evidence.violation_type, ViolationType::FailedInclusion));

        println!("✅ Slashing evidence generation working");
    }

    #[tokio::test]
    async fn test_proposer_compliance() {
        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config);

        let obligation = create_test_obligation(100);
        manager.add_pending_obligation(obligation.clone()).await.unwrap();

        // Test compliant proposer
        let is_compliant = manager.validate_proposer_compliance(
            &obligation.proposer,
            110,
            &[obligation.id.clone()],
        ).await.unwrap();
        assert!(is_compliant);

        // Test non-compliant proposer
        let is_compliant = manager.validate_proposer_compliance(
            &obligation.proposer,
            110,
            &[], // Empty - missing required obligation
        ).await.unwrap();
        assert!(!is_compliant);

        println!("✅ Proposer compliance checking working");
    }

    #[tokio::test]
    async fn test_stage22_exit_criteria() {
        println!("\n=== Stage 22: Inclusion Lists Exit Criteria ===");

        let config = InclusionListConfig::default();
        let manager = InclusionListManager::new(config);

        // Test 1: List Maintenance (pending obligations, inc_root)
        let obligation = create_test_obligation(100);
        manager.add_pending_obligation(obligation.clone()).await.unwrap();

        let list = manager.create_inclusion_list(
            105,
            b"proposer".to_vec(),
            vec![obligation.id.clone()],
        ).await.unwrap();
        assert!(list.verify_root());
        println!("✅ Test 1: List Maintenance (inc_root computation) - PASSED");

        // Test 2: Missing Items Detected
        let _empty_list = manager.create_inclusion_list(
            110,
            b"proposer".to_vec(),
            vec![], // Missing the obligation
        ).await.unwrap();

        let missing_items = manager.detect_missing_items(110).await.unwrap();
        assert!(!missing_items.is_empty());
        println!("✅ Test 2: Missing Items Detected - PASSED");

        // Test 3: Slashable Evidence Generated
        let evidence = manager.generate_slashing_evidence(
            b"validator".to_vec(),
            missing_items,
            (100, 110),
        ).await.unwrap();
        assert!(evidence.severity_score > 0);
        assert!(!evidence.missing_items.is_empty());
        println!("✅ Test 3: Slashable Evidence Generated - PASSED");

        // Test 4: Integration (proposer requirements, validator checking)
        let requirements = manager.get_proposer_requirements(&obligation.proposer, 110).await;
        assert!(!requirements.is_empty());
        println!("✅ Test 4: Integration (proposer requirements) - PASSED");

        println!("\n🎉 Stage 22: Inclusion Lists - ALL TESTS PASSED!");
        println!("📋 Features: List maintenance, Missing item detection, Slashing evidence");
        println!("🔍 Enforcement: Proposer compliance, Validator checking, Evidence generation");
        println!("⚖️  Security: Cryptographic proofs, Violation detection, Penalty scoring");
    }
}
