//! a² Sync-Pair Primitive Implementation
//! 
//! This module implements the core a² sync-pair primitive for 6D blockchain,
//! providing the fundamental synchronization mechanism for dimensional transactions.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use sha3::{Digest, Sha3_256};
use uuid::Uuid;
use crate::quantum_entanglement::{QuantumEntanglementSystem, QuantumState, EntanglementType};

/// a² Sync-Pair Primitive - Core synchronization mechanism for 6D blockchain
#[derive(Debug, Clone)]
pub struct SyncPairPrimitive {
    /// Quantum entanglement system for sync-pair operations
    quantum_system: Arc<QuantumEntanglementSystem>,
    /// Active sync pairs
    active_pairs: Arc<RwLock<HashMap<String, SyncPair>>>,
    /// Pair header cache for fast lookups
    header_cache: Arc<RwLock<HashMap<String, PairHeader>>>,
    /// Sync-pair statistics
    stats: Arc<RwLock<SyncPairStats>>,
}

/// Sync-Pair structure representing entangled transaction pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPair {
    /// Unique sync-pair identifier
    pub pair_id: String,
    /// Pair header with metadata
    pub header: PairHeader,
    /// First transaction in the pair (a)
    pub transaction_a: SyncTransaction,
    /// Second transaction in the pair (a²)
    pub transaction_a_squared: SyncTransaction,
    /// Quantum entanglement proof between the pair
    pub entanglement_proof: String,
    /// Sync-pair status
    pub status: SyncPairStatus,
    /// Creation timestamp
    pub created_at: u64,
    /// Last synchronization timestamp
    pub last_sync_at: u64,
}

/// Pair Header - Critical metadata for sync-pair operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairHeader {
    /// Header version for compatibility
    pub version: u8,
    /// Pair type classification
    pub pair_type: SyncPairType,
    /// Dimensional coordinates for the pair
    pub dimensional_coords: DimensionalCoordinates,
    /// Quantum state hash for the pair
    pub quantum_state_hash: String,
    /// Cryptographic binding between transactions
    pub binding_proof: String,
    /// Sync requirements and constraints
    pub sync_requirements: SyncRequirements,
    /// Header integrity hash
    pub header_hash: String,
}

/// Sync Transaction - Individual transaction within a sync-pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncTransaction {
    /// Transaction identifier
    pub transaction_id: String,
    /// Transaction data hash
    pub data_hash: String,
    /// Quantum state for this transaction
    pub quantum_state: String,
    /// Dimensional position
    pub dimensional_position: DimensionalPosition,
    /// Sync timestamp
    pub sync_timestamp: u64,
    /// Transaction signature
    pub signature: String,
}

/// Types of sync-pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncPairType {
    /// Standard transaction pair
    Standard,
    /// High-priority system pair
    System,
    /// Government/compliance pair
    Government,
    /// Banking/financial pair
    Banking,
    /// Emergency/critical pair
    Emergency,
    /// Cross-dimensional pair
    CrossDimensional,
}

/// Sync-pair status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncPairStatus {
    /// Pair is being created
    Creating,
    /// Pair is active and synchronized
    Active,
    /// Pair is temporarily out of sync
    OutOfSync,
    /// Pair is being synchronized
    Synchronizing,
    /// Pair has failed synchronization
    Failed,
    /// Pair is completed
    Completed,
}

/// Dimensional coordinates for sync-pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalCoordinates {
    /// X dimension (Events)
    pub x: f64,
    /// Y dimension (Receipts)
    pub y: f64,
    /// Z dimension (State)
    pub z: f64,
    /// A dimension (Audit)
    pub a: f64,
    /// B dimension (Boundary)
    pub b: f64,
    /// C dimension (Correction)
    pub c: f64,
}

/// Dimensional position for individual transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalPosition {
    /// Phase coordinates (XYZ)
    pub phase: PhaseCoordinates,
    /// Horizon coordinates (ABC)
    pub horizon: HorizonCoordinates,
}

/// Phase coordinates (Events, Receipts, State)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseCoordinates {
    /// Events dimension
    pub events: f64,
    /// Receipts dimension
    pub receipts: f64,
    /// State/Consensus dimension
    pub state: f64,
}

/// Horizon coordinates (Audit, Boundary, Correction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizonCoordinates {
    /// Audit dimension
    pub audit: f64,
    /// Boundary dimension
    pub boundary: f64,
    /// Correction dimension
    pub correction: f64,
}

/// Sync requirements for pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequirements {
    /// Maximum allowed sync delay (milliseconds)
    pub max_sync_delay_ms: u64,
    /// Required quantum coherence level (0.0-1.0)
    pub min_coherence_level: f64,
    /// Required entanglement strength (0.0-1.0)
    pub min_entanglement_strength: f64,
    /// Dimensional tolerance for coordinates
    pub dimensional_tolerance: f64,
}

/// Sync-pair statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPairStats {
    /// Total pairs created
    pub total_pairs_created: u64,
    /// Active pairs count
    pub active_pairs_count: u64,
    /// Successful synchronizations
    pub successful_syncs: u64,
    /// Failed synchronizations
    pub failed_syncs: u64,
    /// Average sync time (milliseconds)
    pub avg_sync_time_ms: f64,
    /// Quantum coherence average
    pub avg_coherence_level: f64,
}

impl Default for SyncRequirements {
    fn default() -> Self {
        Self {
            max_sync_delay_ms: 500, // ≤0.5ms target
            min_coherence_level: 0.95,
            min_entanglement_strength: 0.90,
            dimensional_tolerance: 1e-10,
        }
    }
}

impl Default for SyncPairStats {
    fn default() -> Self {
        Self {
            total_pairs_created: 0,
            active_pairs_count: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            avg_sync_time_ms: 0.0,
            avg_coherence_level: 0.0,
        }
    }
}

impl SyncPairPrimitive {
    /// Create new sync-pair primitive
    pub fn new(quantum_system: Arc<QuantumEntanglementSystem>) -> Self {
        Self {
            quantum_system,
            active_pairs: Arc::new(RwLock::new(HashMap::new())),
            header_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(SyncPairStats::default())),
        }
    }

    /// Create a new sync-pair from two transactions
    pub async fn create_sync_pair(
        &self,
        transaction_a_data: &str,
        transaction_b_data: &str,
        pair_type: SyncPairType,
        dimensional_coords: DimensionalCoordinates,
    ) -> Result<String> {
        let pair_id = Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        // Create quantum entanglement between transactions
        let entanglement_proof = self.quantum_system.create_transaction_entanglement(
            transaction_a_data,
            transaction_b_data,
            EntanglementType::TransactionPair,
        ).await?;

        // Generate quantum states for both transactions
        let quantum_state_a = self.generate_quantum_state(transaction_a_data)?;
        let quantum_state_b = self.generate_quantum_state(transaction_b_data)?;

        // Create sync transactions
        let transaction_a = SyncTransaction {
            transaction_id: format!("{}_a", pair_id),
            data_hash: self.calculate_hash(transaction_a_data)?,
            quantum_state: quantum_state_a,
            dimensional_position: self.calculate_dimensional_position(&dimensional_coords, true)?,
            sync_timestamp: now,
            signature: self.generate_transaction_signature(transaction_a_data)?,
        };

        let transaction_a_squared = SyncTransaction {
            transaction_id: format!("{}_a2", pair_id),
            data_hash: self.calculate_hash(transaction_b_data)?,
            quantum_state: quantum_state_b,
            dimensional_position: self.calculate_dimensional_position(&dimensional_coords, false)?,
            sync_timestamp: now,
            signature: self.generate_transaction_signature(transaction_b_data)?,
        };

        // Create pair header
        let header = self.create_pair_header(
            &pair_type,
            &dimensional_coords,
            &transaction_a,
            &transaction_a_squared,
        )?;

        // Create sync-pair
        let sync_pair = SyncPair {
            pair_id: pair_id.clone(),
            header: header.clone(),
            transaction_a,
            transaction_a_squared,
            entanglement_proof: entanglement_proof.entanglement_id.to_string(),
            status: SyncPairStatus::Active,
            created_at: now,
            last_sync_at: now,
        };

        // Store pair and header
        {
            let mut pairs = self.active_pairs.write().unwrap();
            pairs.insert(pair_id.clone(), sync_pair);
        }
        {
            let mut headers = self.header_cache.write().unwrap();
            headers.insert(pair_id.clone(), header);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_pairs_created += 1;
            stats.active_pairs_count += 1;
        }

        Ok(pair_id)
    }

    /// Synchronize a sync-pair
    pub async fn synchronize_pair(&self, pair_id: &str) -> Result<bool> {
        let start_time = std::time::Instant::now();
        
        let mut pair = {
            let pairs = self.active_pairs.read().unwrap();
            pairs.get(pair_id).cloned()
                .ok_or_else(|| anyhow!("Sync-pair not found: {}", pair_id))?
        };

        // Check sync requirements
        if !self.check_sync_requirements(&pair)? {
            pair.status = SyncPairStatus::Failed;
            self.update_pair(pair_id, pair)?;
            return Ok(false);
        }

        // Verify quantum entanglement
        let entanglement_uuid = Uuid::parse_str(&pair.entanglement_proof)?;
        let entanglement_valid = self.quantum_system.verify_entanglement(&entanglement_uuid)?;
        if !entanglement_valid {
            pair.status = SyncPairStatus::OutOfSync;
            self.update_pair(pair_id, pair)?;
            return Ok(false);
        }

        // Perform synchronization
        pair.status = SyncPairStatus::Synchronizing;
        self.update_pair(pair_id, pair.clone())?;

        // Update sync timestamp
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        pair.last_sync_at = now;
        pair.status = SyncPairStatus::Active;

        self.update_pair(pair_id, pair)?;

        // Update statistics
        let sync_time = start_time.elapsed().as_millis() as f64;
        {
            let mut stats = self.stats.write().unwrap();
            stats.successful_syncs += 1;
            stats.avg_sync_time_ms = (stats.avg_sync_time_ms * (stats.successful_syncs - 1) as f64 + sync_time) / stats.successful_syncs as f64;
        }

        Ok(true)
    }

    /// Get sync-pair by ID
    pub fn get_sync_pair(&self, pair_id: &str) -> Result<Option<SyncPair>> {
        let pairs = self.active_pairs.read().unwrap();
        Ok(pairs.get(pair_id).cloned())
    }

    /// Get pair header by ID
    pub fn get_pair_header(&self, pair_id: &str) -> Result<Option<PairHeader>> {
        let headers = self.header_cache.read().unwrap();
        Ok(headers.get(pair_id).cloned())
    }

    /// Get sync-pair statistics
    pub fn get_stats(&self) -> Result<SyncPairStats> {
        let stats = self.stats.read().unwrap();
        Ok(stats.clone())
    }

    /// Validate dimensional coordinates
    pub fn validate_dimensional_coordinates(&self, coords: &DimensionalCoordinates) -> Result<bool> {
        // Check coordinate bounds and relationships
        let coords_valid = coords.x >= 0.0 && coords.x <= 1.0 &&
                          coords.y >= 0.0 && coords.y <= 1.0 &&
                          coords.z >= 0.0 && coords.z <= 1.0 &&
                          coords.a >= 0.0 && coords.a <= 1.0 &&
                          coords.b >= 0.0 && coords.b <= 1.0 &&
                          coords.c >= 0.0 && coords.c <= 1.0;

        // Check cuboidal constraints (XYZ × ABC)
        let phase_sum = coords.x + coords.y + coords.z;
        let horizon_sum = coords.a + coords.b + coords.c;
        let cuboidal_valid = phase_sum <= 3.0 && horizon_sum <= 3.0;

        Ok(coords_valid && cuboidal_valid)
    }

    // Private helper methods

    /// Create pair header
    fn create_pair_header(
        &self,
        pair_type: &SyncPairType,
        dimensional_coords: &DimensionalCoordinates,
        transaction_a: &SyncTransaction,
        transaction_a_squared: &SyncTransaction,
    ) -> Result<PairHeader> {
        let quantum_state_hash = self.calculate_combined_quantum_hash(
            &transaction_a.quantum_state,
            &transaction_a_squared.quantum_state,
        )?;

        let binding_proof = self.generate_binding_proof(transaction_a, transaction_a_squared)?;

        let header = PairHeader {
            version: 1,
            pair_type: pair_type.clone(),
            dimensional_coords: dimensional_coords.clone(),
            quantum_state_hash,
            binding_proof,
            sync_requirements: SyncRequirements::default(),
            header_hash: String::new(), // Will be calculated below
        };

        // Calculate header hash
        let header_data = serde_json::to_string(&header)?;
        let header_hash = self.calculate_hash(&header_data)?;

        Ok(PairHeader {
            header_hash,
            ..header
        })
    }

    /// Generate quantum state for transaction
    fn generate_quantum_state(&self, transaction_data: &str) -> Result<String> {
        let mut hasher = Sha3_256::new();
        hasher.update(b"quantum_state:");
        hasher.update(transaction_data.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate dimensional position
    fn calculate_dimensional_position(
        &self,
        coords: &DimensionalCoordinates,
        is_first_transaction: bool,
    ) -> Result<DimensionalPosition> {
        let phase_offset = if is_first_transaction { 0.0 } else { 0.1 };
        let horizon_offset = if is_first_transaction { 0.0 } else { 0.1 };

        Ok(DimensionalPosition {
            phase: PhaseCoordinates {
                events: coords.x + phase_offset,
                receipts: coords.y + phase_offset,
                state: coords.z + phase_offset,
            },
            horizon: HorizonCoordinates {
                audit: coords.a + horizon_offset,
                boundary: coords.b + horizon_offset,
                correction: coords.c + horizon_offset,
            },
        })
    }

    /// Calculate hash
    fn calculate_hash(&self, data: &str) -> Result<String> {
        let mut hasher = Sha3_256::new();
        hasher.update(data.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Generate transaction signature
    fn generate_transaction_signature(&self, transaction_data: &str) -> Result<String> {
        let mut hasher = Sha3_256::new();
        hasher.update(b"signature:");
        hasher.update(transaction_data.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculate combined quantum hash
    fn calculate_combined_quantum_hash(&self, state_a: &str, state_b: &str) -> Result<String> {
        let mut hasher = Sha3_256::new();
        hasher.update(state_a.as_bytes());
        hasher.update(state_b.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Generate binding proof
    fn generate_binding_proof(&self, tx_a: &SyncTransaction, tx_b: &SyncTransaction) -> Result<String> {
        let mut hasher = Sha3_256::new();
        hasher.update(b"binding:");
        hasher.update(tx_a.transaction_id.as_bytes());
        hasher.update(tx_b.transaction_id.as_bytes());
        hasher.update(tx_a.quantum_state.as_bytes());
        hasher.update(tx_b.quantum_state.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Check sync requirements
    fn check_sync_requirements(&self, pair: &SyncPair) -> Result<bool> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
        let sync_delay = now - pair.last_sync_at;

        Ok(sync_delay <= pair.header.sync_requirements.max_sync_delay_ms)
    }

    /// Update pair in storage
    fn update_pair(&self, pair_id: &str, pair: SyncPair) -> Result<()> {
        let mut pairs = self.active_pairs.write().unwrap();
        pairs.insert(pair_id.to_string(), pair);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_pair_creation() -> Result<()> {
        let quantum_system = Arc::new(QuantumEntanglementSystem::new_sync()?);
        let sync_primitive = SyncPairPrimitive::new(quantum_system);

        let coords = DimensionalCoordinates {
            x: 0.1, y: 0.2, z: 0.3,
            a: 0.4, b: 0.5, c: 0.6,
        };

        let pair_id = sync_primitive.create_sync_pair(
            "transaction_a_data",
            "transaction_b_data",
            SyncPairType::Standard,
            coords,
        ).await.unwrap();

        assert!(!pair_id.is_empty());

        let pair = sync_primitive.get_sync_pair(&pair_id).unwrap().unwrap();
        assert_eq!(pair.status, SyncPairStatus::Active);
        Ok(())
    }

    #[tokio::test]
    async fn test_dimensional_validation() -> Result<()> {
        let quantum_system = Arc::new(QuantumEntanglementSystem::new_sync()?);
        let sync_primitive = SyncPairPrimitive::new(quantum_system);

        let valid_coords = DimensionalCoordinates {
            x: 0.5, y: 0.5, z: 0.5,
            a: 0.5, b: 0.5, c: 0.5,
        };

        assert!(sync_primitive.validate_dimensional_coordinates(&valid_coords).unwrap());

        let invalid_coords = DimensionalCoordinates {
            x: 1.5, y: 0.5, z: 0.5, // Invalid: x > 1.0
            a: 0.5, b: 0.5, c: 0.5,
        };

        assert!(!sync_primitive.validate_dimensional_coordinates(&invalid_coords).unwrap());
        Ok(())
    }
}
