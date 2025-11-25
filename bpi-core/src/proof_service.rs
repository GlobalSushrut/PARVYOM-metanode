//! OS-level ProofService facade.
//!
//! This module provides a thin, crate-level abstraction over the various
//! proof systems so callers do not need to know about individual POA/POE/POT
//! modules. For now we start with POA (Proof-of-Action); additional proof
//! families will be added incrementally.

use anyhow::Result;
use tracing::info;
use crate::proof_systems::ProofSystem;

use crate::proof_systems::poa_proof::{
    POAProofSystem,
    ContainerStateTransition,
    POAProofData,
};
use crate::proof_systems::poe_proof::{
    POEProofSystem,
    BPIAgreementExecution,
    POEProofData,
};
use crate::proof_systems::pot_proof::{
    POTProofSystem,
    CrossChainTransaction,
    POTProofData,
};
use crate::proof_systems::pog_proof::{
    POGProofSystem,
    EconomicTransaction,
    POGProofData,
};
use crate::proof_systems::poh_proof::{
    POHProofSystem,
    HistoricalEvent,
    POHProofData,
};
use crate::proof_systems::vm_audit_proof::{
    VMAuditProofSystem,
    VMAuditContext,
    VMAuditProofData,
};
use crate::blockchain_os_kernel::BulletproofProver;

/// High-level OS ProofService abstraction.
///
/// This will grow to cover POE/POT/POG/POH, Bulletproofs, and VM audit proofs.
pub trait ProofService {
    /// Generate a POA proof for a single container state transition.
    fn generate_poa_proof(&self, transition: &ContainerStateTransition) -> Result<POAProofData>;

    /// Generate a POE proof for a single BPI agreement execution.
    fn generate_poe_proof(&self, execution: &BPIAgreementExecution) -> Result<POEProofData>;

    /// Generate a POT proof for a single cross-chain transaction.
    fn generate_pot_proof(&self, tx: &CrossChainTransaction) -> Result<POTProofData>;

    /// Generate a POG proof for a single economic transaction.
    fn generate_pog_proof(&self, tx: &EconomicTransaction) -> Result<POGProofData>;

    /// Generate a POH proof for a single historical event.
    fn generate_poh_proof(&self, event: &HistoricalEvent) -> Result<POHProofData>;

    /// Generate a comprehensive VM audit proof that orchestrates all core
    /// proof systems for a single audit context.
    fn generate_vm_audit_proof(&self, ctx: &VMAuditContext) -> Result<VMAuditProofData>;

    /// Generate a Bulletproof range proof for a single 64-bit value.
    /// Returns (proof_bytes, commitment_bytes).
    fn generate_bulletproof_range(&self, value: u64) -> Result<(Vec<u8>, Vec<u8>)>;

    /// Verify a Bulletproof range proof produced by `generate_bulletproof_range`.
    fn verify_bulletproof_range(&self, proof_bytes: &[u8], commitment_bytes: &[u8]) -> Result<bool>;
}

/// Default in-process implementation of ProofService that delegates to the
/// existing POAProofSystem logic, preserving existing proof shapes and
/// hashing behaviour.
pub struct DefaultProofService {
    poa_system: POAProofSystem,
    poe_system: POEProofSystem,
    pot_system: POTProofSystem,
    pog_system: POGProofSystem,
    poh_system: POHProofSystem,
    vm_audit_system: VMAuditProofSystem,
    bulletproof_prover: BulletproofProver,
}

impl Default for DefaultProofService {
    fn default() -> Self {
        Self {
            poa_system: POAProofSystem::new(),
            poe_system: POEProofSystem::new(),
            pot_system: POTProofSystem::new(),
            pog_system: POGProofSystem::new(),
            poh_system: POHProofSystem::new(),
            vm_audit_system: VMAuditProofSystem::new(),
            bulletproof_prover: BulletproofProver::new(),
        }
    }
}

impl ProofService for DefaultProofService {
    fn generate_poa_proof(&self, transition: &ContainerStateTransition) -> Result<POAProofData> {
        // Reuse the existing POAProofSystem::generate_proof implementation to
        // avoid duplicating logic. We serialize the transition to JSON bytes
        // and then parse the resulting proof JSON back into POAProofData.
        let data = serde_json::to_vec(transition)?;
        let proof_json = self.poa_system.generate_proof(&data)?;
        let proof: POAProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'POA', message = 'Generated POA proof via ProofService'");
        Ok(proof)
    }

    fn generate_poe_proof(&self, execution: &BPIAgreementExecution) -> Result<POEProofData> {
        // Reuse the existing POEProofSystem::generate_proof implementation to
        // avoid duplicating logic. We serialize the execution to JSON bytes
        // and then parse the resulting proof JSON back into POEProofData.
        let data = serde_json::to_vec(execution)?;
        let proof_json = self.poe_system.generate_proof(&data)?;
        let proof: POEProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'POE', message = 'Generated POE proof via ProofService'");
        Ok(proof)
    }

    fn generate_pot_proof(&self, tx: &CrossChainTransaction) -> Result<POTProofData> {
        // Reuse the existing POTProofSystem::generate_proof implementation to
        // avoid duplicating logic. We serialize the transaction to JSON bytes
        // and then parse the resulting proof JSON back into POTProofData.
        let data = serde_json::to_vec(tx)?;
        let proof_json = self.pot_system.generate_proof(&data)?;
        let proof: POTProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'POT', message = 'Generated POT proof via ProofService'");
        Ok(proof)
    }

    fn generate_pog_proof(&self, tx: &EconomicTransaction) -> Result<POGProofData> {
        // Reuse the existing POGProofSystem::generate_proof implementation to
        // avoid duplicating logic. We serialize the transaction to JSON bytes
        // and then parse the resulting proof JSON back into POGProofData.
        let data = serde_json::to_vec(tx)?;
        let proof_json = self.pog_system.generate_proof(&data)?;
        let proof: POGProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'POG', message = 'Generated POG proof via ProofService'");
        Ok(proof)
    }

    fn generate_poh_proof(&self, event: &HistoricalEvent) -> Result<POHProofData> {
        // Reuse the existing POHProofSystem::generate_proof implementation to
        // avoid duplicating logic. We serialize the event to JSON bytes and
        // then parse the resulting proof JSON back into POHProofData.
        let data = serde_json::to_vec(event)?;
        let proof_json = self.poh_system.generate_proof(&data)?;
        let proof: POHProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'POH', message = 'Generated POH proof via ProofService'");
        Ok(proof)
    }

    fn generate_vm_audit_proof(&self, ctx: &VMAuditContext) -> Result<VMAuditProofData> {
        // Reuse the existing VMAuditProofSystem::generate_proof implementation
        // to avoid duplicating logic. We serialize the context to JSON bytes
        // and then parse the resulting proof JSON back into VMAuditProofData.
        let data = serde_json::to_vec(ctx)?;
        let proof_json = self.vm_audit_system.generate_proof(&data)?;
        let proof: VMAuditProofData = serde_json::from_str(&proof_json)?;
        info!("proof_type = 'VM_AUDIT', message = 'Generated VM audit proof via ProofService'");
        Ok(proof)
    }

    fn generate_bulletproof_range(&self, value: u64) -> Result<(Vec<u8>, Vec<u8>)> {
        // Directly delegate to the production BulletproofProver. This keeps
        // all cryptographic behaviour identical while providing a unified
        // facade for callers.
        let result = self.bulletproof_prover.prove(value, None)?;
        info!("proof_type = 'BULLETPROOF_RANGE', message = 'Generated Bulletproof range proof via ProofService'");
        Ok(result)
    }

    fn verify_bulletproof_range(&self, proof_bytes: &[u8], commitment_bytes: &[u8]) -> Result<bool> {
        let verified = self.bulletproof_prover.verify(proof_bytes, commitment_bytes)?;
        info!("proof_type = 'BULLETPROOF_RANGE', verified = %verified, message = 'Verified Bulletproof range proof via ProofService'");
        Ok(verified)
    }
}
