//! Category Theory Framework for Ledger Operations
//! 
//! This module implements category theory structures for composing operations
//! across different ledger types (DockLock, ENC, BPI, BPCI, Economy)

use crate::{Hash, MathError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ledger types in the Metanode system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LedgerType {
    DockLock,   // Container operations
    ENC,        // Cluster coordination
    BPI,        // Agreement execution
    BPCI,       // Cross-chain consensus
    Economy,    // Economic operations
}

/// Objects in the ledger category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerObject {
    pub ledger_type: LedgerType,
    pub object_id: String,
    pub state_hash: Hash,
    pub timestamp: crate::Timestamp,
}

/// Morphisms (transformations) between ledger objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerMorphism {
    pub source: LedgerObject,
    pub target: LedgerObject,
    pub transformation: TransformationType,
    pub proof_hash: Hash,
}

/// Types of transformations in the ledger category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformationType {
    ReceiptAggregation,     // Aggregate receipts into transactions
    ProofComposition,       // Compose multiple proofs
    StateTransition,        // State change within ledger
    CrossLedgerSync,        // Synchronization across ledgers
    ConsensusVote,          // Consensus participation
}

/// Category theory trait for ledger operations
pub trait LedgerCategory {
    type Object;
    type Morphism;
    
    /// Compose two morphisms: g ∘ f
    fn compose(f: Self::Morphism, g: Self::Morphism) -> Result<Self::Morphism, MathError>;
    
    /// Identity morphism for an object
    fn identity(obj: Self::Object) -> Self::Morphism;
    
    /// Verify category laws (associativity, identity)
    fn verify_laws(&self) -> bool;
}

/// Implementation of the ledger category
#[derive(Debug)]
pub struct MetanodeLedgerCategory {
    objects: HashMap<String, LedgerObject>,
    morphisms: Vec<LedgerMorphism>,
}

impl MetanodeLedgerCategory {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            morphisms: Vec::new(),
        }
    }
    
    pub fn add_object(&mut self, obj: LedgerObject) {
        self.objects.insert(obj.object_id.clone(), obj);
    }
    
    pub fn add_morphism(&mut self, morphism: LedgerMorphism) {
        self.morphisms.push(morphism);
    }
    
    /// Create a receipt aggregation morphism
    pub fn create_receipt_aggregation(
        &self,
        source_receipts: Vec<LedgerObject>,
        target_transaction: LedgerObject,
    ) -> Result<LedgerMorphism, MathError> {
        if source_receipts.is_empty() {
            return Err(MathError::CategoryComposition(
                "Cannot aggregate empty receipt set".to_string()
            ));
        }
        
        // Verify all receipts are from the same ledger type
        let ledger_type = &source_receipts[0].ledger_type;
        if !source_receipts.iter().all(|r| &r.ledger_type == ledger_type) {
            return Err(MathError::CategoryComposition(
                "All receipts must be from the same ledger type".to_string()
            ));
        }
        
        // Create aggregated source object
        let aggregated_hash = self.aggregate_hashes(&source_receipts);
        let source = LedgerObject {
            ledger_type: ledger_type.clone(),
            object_id: format!("aggregated_{}", hex::encode(&aggregated_hash[..8])),
            state_hash: aggregated_hash,
            timestamp: chrono::Utc::now(),
        };
        
        let proof_data = format!("receipt_aggregation_{}_{}", 
            source_receipts.len(), 
            hex::encode(target_transaction.state_hash)
        );
        let proof_hash = crate::hash_data(proof_data.as_bytes());
        
        Ok(LedgerMorphism {
            source,
            target: target_transaction,
            transformation: TransformationType::ReceiptAggregation,
            proof_hash,
        })
    }
    
    /// Aggregate multiple hashes using Merkle tree approach
    fn aggregate_hashes(&self, objects: &[LedgerObject]) -> Hash {
        let mut hashes: Vec<Hash> = objects.iter().map(|obj| obj.state_hash).collect();
        
        // Build Merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                if chunk.len() == 2 {
                    let combined = [chunk[0], chunk[1]].concat();
                    next_level.push(crate::hash_data(&combined));
                } else {
                    next_level.push(chunk[0]);
                }
            }
            hashes = next_level;
        }
        
        hashes[0]
    }
}

impl LedgerCategory for MetanodeLedgerCategory {
    type Object = LedgerObject;
    type Morphism = LedgerMorphism;
    
    fn compose(f: Self::Morphism, g: Self::Morphism) -> Result<Self::Morphism, MathError> {
        // Verify composition is valid: target of f must match source of g
        if f.target.object_id != g.source.object_id {
            return Err(MathError::CategoryComposition(
                format!("Cannot compose morphisms: target of f ({}) != source of g ({})",
                    f.target.object_id, g.source.object_id)
            ));
        }
        
        // Create composed morphism
        let composition_data = format!("compose_{}_{}", 
            hex::encode(f.proof_hash), 
            hex::encode(g.proof_hash)
        );
        let proof_hash = crate::hash_data(composition_data.as_bytes());
        
        Ok(LedgerMorphism {
            source: f.source,
            target: g.target,
            transformation: TransformationType::ProofComposition,
            proof_hash,
        })
    }
    
    fn identity(obj: Self::Object) -> Self::Morphism {
        let identity_data = format!("identity_{}", obj.object_id);
        let proof_hash = crate::hash_data(identity_data.as_bytes());
        
        LedgerMorphism {
            source: obj.clone(),
            target: obj,
            transformation: TransformationType::StateTransition,
            proof_hash,
        }
    }
    
    fn verify_laws(&self) -> bool {
        // TODO: Implement category law verification
        // 1. Associativity: (h ∘ g) ∘ f = h ∘ (g ∘ f)
        // 2. Identity: id_B ∘ f = f = f ∘ id_A for f: A → B
        true
    }
}

impl Default for MetanodeLedgerCategory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ledger_category_creation() {
        let category = MetanodeLedgerCategory::new();
        assert!(category.objects.is_empty());
        assert!(category.morphisms.is_empty());
    }
    
    #[test]
    fn test_receipt_aggregation() {
        let mut category = MetanodeLedgerCategory::new();
        
        // Create test receipts
        let receipts = vec![
            LedgerObject {
                ledger_type: LedgerType::DockLock,
                object_id: "receipt_1".to_string(),
                state_hash: crate::hash_data(b"receipt_1"),
                timestamp: chrono::Utc::now(),
            },
            LedgerObject {
                ledger_type: LedgerType::DockLock,
                object_id: "receipt_2".to_string(),
                state_hash: crate::hash_data(b"receipt_2"),
                timestamp: chrono::Utc::now(),
            },
        ];
        
        let transaction = LedgerObject {
            ledger_type: LedgerType::DockLock,
            object_id: "transaction_1".to_string(),
            state_hash: crate::hash_data(b"transaction_1"),
            timestamp: chrono::Utc::now(),
        };
        
        let morphism = category.create_receipt_aggregation(receipts, transaction);
        assert!(morphism.is_ok());
        
        let morphism = morphism.unwrap();
        assert_eq!(morphism.transformation, TransformationType::ReceiptAggregation);
    }
    
    #[test]
    fn test_identity_morphism() {
        let obj = LedgerObject {
            ledger_type: LedgerType::BPI,
            object_id: "test_object".to_string(),
            state_hash: crate::hash_data(b"test"),
            timestamp: chrono::Utc::now(),
        };
        
        let identity = MetanodeLedgerCategory::identity(obj.clone());
        assert_eq!(identity.source.object_id, obj.object_id);
        assert_eq!(identity.target.object_id, obj.object_id);
    }
}
