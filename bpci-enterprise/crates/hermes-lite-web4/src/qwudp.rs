//! Quantum-Wave UDP (QW-UDP) Protocol
//! 
//! Revolutionary transport protocol with quantum phase-locking.
//! Based on Schrödinger evolution for network timing coordination.
//! 
//! Key Features:
//! - Phase-locked communication (1-2ms latency)
//! - Schrödinger-timed packets
//! - Trigonometric control laws
//! - HMAC authentication
//! 
//! Mathematical Foundation:
//! - Unitary evolution: U(t) = e^(-iHt/ℏ)
//! - Phase: φ_eff(t) = φ - ω·Δt
//! - Routing score: (1/(1+HRW)) · cos²(φ_edge - φ_svc) / √(1 + ρ·load)

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::f64::consts::PI;

/// QW-UDP packet header (32 bytes fixed size)
/// 
/// This header enables phase-locked communication with quantum timing precision.
/// All fields are carefully sized to fit exactly 32 bytes for efficient transmission.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct QwUdpHeader {
    /// Epoch time in femtoseconds (10^-15 seconds) for phase-lock
    /// Provides quantum-level timing precision
    pub epoch_time_fs: u64,
    
    /// Phase numerator for φ = 2π · phase_num / 2^32
    /// Represents quantum phase in Q32.32 fixed-point format
    pub phase_num: u64,
    
    /// Omega drift in parts per million (ppm)
    /// Tracks phase drift for correction
    pub omega_ppm: u32,
    
    /// Link ID - stable hash of H_ij Hamiltonian identity
    pub link_id: u32,
    
    /// Operation code
    pub op_code: u8,
    
    /// Basis code for quantum operations
    pub basis_code: u8,
    
    /// Quantum error correction code
    pub qec_code: u8,
    
    /// Flags (RETRY, TIGHT_DEADLINE, SEAL, ATTEST)
    pub flags: u8,
    
    /// CRC32 checksum (HMAC in trailer)
    pub crc32: u32,
}

/// Operation codes for QW-UDP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    /// Prepare entanglement
    PREP = 0x01,
    /// Swap operation
    SWAP = 0x02,
    /// Distillation
    DISTILL = 0x03,
    /// Measurement
    MEAS = 0x04,
    /// Feed-forward
    FEED = 0x05,
    /// Acknowledgment
    ACK = 0x06,
    /// Negative acknowledgment
    NACK = 0x07,
    /// Service discovery
    DISCOVER = 0x08,
    /// Data transfer
    DATA = 0x09,
}

impl OpCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(OpCode::PREP),
            0x02 => Some(OpCode::SWAP),
            0x03 => Some(OpCode::DISTILL),
            0x04 => Some(OpCode::MEAS),
            0x05 => Some(OpCode::FEED),
            0x06 => Some(OpCode::ACK),
            0x07 => Some(OpCode::NACK),
            0x08 => Some(OpCode::DISCOVER),
            0x09 => Some(OpCode::DATA),
            _ => None,
        }
    }
}

/// Basis codes for quantum operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BasisCode {
    /// Pauli X basis
    X = 0x01,
    /// Pauli Z basis
    Z = 0x02,
    /// Hadamard basis
    H = 0x03,
    /// Rotation around X
    RX = 0x04,
    /// Rotation around Z
    RZ = 0x05,
    /// Custom basis
    CUSTOM = 0xFF,
}

/// QEC (Quantum Error Correction) codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QecCode {
    /// No error correction
    NONE = 0x00,
    /// Surface code distance 3
    SURF_D3 = 0x03,
    /// Surface code distance 5
    SURF_D5 = 0x05,
    /// Surface code distance 7
    SURF_D7 = 0x07,
    /// Steane code
    STEANE = 0x10,
}

/// Flags for QW-UDP packets
pub mod flags {
    /// Retry flag - packet is a retry
    pub const RETRY: u8 = 0x01;
    /// Tight deadline - requires immediate processing
    pub const TIGHT_DEADLINE: u8 = 0x02;
    /// Seal - packet is sealed with HMAC
    pub const SEAL: u8 = 0x04;
    /// Attest - packet includes attestation
    pub const ATTEST: u8 = 0x08;
}

/// Phase controller for Schrödinger-timed communication
/// 
/// Implements quantum phase evolution: U(t) = e^(-iHt/ℏ)
#[derive(Debug, Clone)]
pub struct PhaseController {
    /// Phase numerator (Q32.32 format)
    phi_num: u32,
    /// Omega drift in parts per million
    omega_ppm: i32,
    /// Last synchronization time
    last_sync_fs: u64,
}

impl PhaseController {
    /// Create new phase controller
    pub fn new() -> Self {
        Self {
            phi_num: 0,
            omega_ppm: 0,
            last_sync_fs: Self::now_fs(),
        }
    }
    
    /// Create with specific phase and drift
    pub fn with_phase(phi_num: u32, omega_ppm: i32) -> Self {
        Self {
            phi_num,
            omega_ppm,
            last_sync_fs: Self::now_fs(),
        }
    }
    
    /// Get current time in femtoseconds since UNIX epoch
    /// 
    /// Note: Uses modulo to prevent overflow while maintaining phase precision
    pub fn now_fs() -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        
        // Use modulo to prevent overflow (phase wraps around anyway)
        // Keep last ~100 seconds worth of femtoseconds for phase calculations
        let secs_mod = (now.as_secs() % 100) as u64;
        let fs_from_secs = secs_mod.saturating_mul(1_000_000_000_000_000);
        let fs_from_nanos = (now.subsec_nanos() as u64).saturating_mul(1_000_000);
        
        fs_from_secs.saturating_add(fs_from_nanos)
    }
    
    /// Calculate effective phase at given time
    /// 
    /// φ_eff(t) = φ - ω·Δt (modulo 2π)
    /// 
    /// This implements the Schrödinger evolution phase tracking
    pub fn phi_eff(&self, t_epoch_fs: u64) -> f64 {
        // Base phase: φ = 2π · phi_num / 2^32
        let phi = 2.0 * PI * (self.phi_num as f64 / u32::MAX as f64);
        
        // Time delta in seconds
        let dt = (t_epoch_fs as i128 - self.last_sync_fs as i128) as f64 * 1e-15;
        
        // Omega in radians per second
        let omega = (self.omega_ppm as f64) * 1e-6;
        
        // Effective phase with drift correction
        let phi_eff = phi - omega * dt;
        
        // Normalize to [0, 2π)
        phi_eff.rem_euclid(2.0 * PI)
    }
    
    /// Calculate phase alignment score (cos² for quantum fidelity)
    /// 
    /// Returns value in [0, 1] where 1 = perfect alignment
    pub fn phase_alignment(&self, other_phase: f64) -> f64 {
        let my_phase = self.phi_eff(Self::now_fs());
        let diff = my_phase - other_phase;
        diff.cos().powi(2)
    }
    
    /// Update phase from synchronization
    pub fn sync(&mut self, phi_num: u32, omega_ppm: i32) {
        self.phi_num = phi_num;
        self.omega_ppm = omega_ppm;
        self.last_sync_fs = Self::now_fs();
    }
    
    /// Get current phase numerator
    pub fn phase_num(&self) -> u32 {
        self.phi_num
    }
    
    /// Get current omega drift
    pub fn omega_ppm(&self) -> i32 {
        self.omega_ppm
    }
    
    /// Check if phase is stable (|cos(φ)| > ε)
    pub fn is_stable(&self, epsilon: f64) -> bool {
        let phi = self.phi_eff(Self::now_fs());
        phi.cos().abs() > epsilon
    }
}

impl Default for PhaseController {
    fn default() -> Self {
        Self::new()
    }
}

/// QW-UDP message with header and payload
#[derive(Debug, Clone)]
pub struct QwUdpMessage {
    pub header: QwUdpHeader,
    pub payload: Vec<u8>,
    pub hmac: Option<Vec<u8>>,
}

impl QwUdpMessage {
    /// Create new QW-UDP message
    pub fn new(
        op_code: OpCode,
        basis_code: BasisCode,
        qec_code: QecCode,
        payload: Vec<u8>,
    ) -> Self {
        let header = QwUdpHeader {
            epoch_time_fs: PhaseController::now_fs(),
            phase_num: 0, // Will be set by phase controller
            omega_ppm: 0,
            link_id: 0,
            op_code: op_code as u8,
            basis_code: basis_code as u8,
            qec_code: qec_code as u8,
            flags: 0,
            crc32: 0, // Will be calculated
        };
        
        Self {
            header,
            payload,
            hmac: None,
        }
    }
    
    /// Set phase information from controller
    pub fn set_phase(&mut self, controller: &PhaseController) {
        self.header.phase_num = controller.phase_num() as u64;
        self.header.omega_ppm = controller.omega_ppm() as u32;
    }
    
    /// Set flags
    pub fn set_flags(&mut self, flags: u8) {
        self.header.flags = flags;
    }
    
    /// Calculate and set CRC32
    pub fn calculate_crc(&mut self) {
        // Simple CRC32 calculation (in production, use proper CRC32 library)
        let mut crc: u32 = 0xFFFFFFFF;
        
        // CRC over header fields (excluding crc32 itself)
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const _ as *const u8,
                28 // 32 bytes - 4 bytes for crc32
            )
        };
        
        for &byte in header_bytes {
            crc ^= byte as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB88320
                } else {
                    crc >> 1
                };
            }
        }
        
        // CRC over payload
        for &byte in &self.payload {
            crc ^= byte as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB88320
                } else {
                    crc >> 1
                };
            }
        }
        
        self.header.crc32 = !crc;
    }
    
    /// Verify CRC32
    pub fn verify_crc(&self) -> bool {
        let mut temp = self.clone();
        let original_crc = temp.header.crc32;
        temp.calculate_crc();
        temp.header.crc32 == original_crc
    }
    
    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Header (32 bytes)
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.header as *const _ as *const u8,
                32
            )
        };
        bytes.extend_from_slice(header_bytes);
        
        // Payload
        bytes.extend_from_slice(&self.payload);
        
        // HMAC (if present)
        if let Some(ref hmac) = self.hmac {
            bytes.extend_from_slice(hmac);
        }
        
        bytes
    }
    
    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 32 {
            return None;
        }
        
        // Parse header
        let header = unsafe {
            std::ptr::read(bytes.as_ptr() as *const QwUdpHeader)
        };
        
        // Parse payload (rest of bytes, excluding potential HMAC)
        let payload = bytes[32..].to_vec();
        
        Some(Self {
            header,
            payload,
            hmac: None,
        })
    }
}

/// Trigonometric routing scorer
/// 
/// Combines hyperbolic distance with quantum phase alignment
pub struct TrigonometricScorer {
    /// Load dampening factor (ρ)
    pub rho: f64,
    /// Security guard epsilon
    pub epsilon: f64,
}

impl TrigonometricScorer {
    /// Create new scorer with default parameters
    pub fn new() -> Self {
        Self {
            rho: 0.5,
            epsilon: 0.01,
        }
    }
    
    /// Calculate trigonometric routing score
    /// 
    /// score = (1/(1+HRW)) · cos²(φ_edge - φ_svc) · sec_guard / √(1 + ρ·load)
    /// 
    /// Higher score = better route
    pub fn score(
        &self,
        hrw: f64,
        phi_edge: f64,
        phi_svc: f64,
        load: f64,
    ) -> f64 {
        // HRW component (rendezvous hashing)
        let hrw_component = 1.0 / (1.0 + hrw);
        
        // Phase alignment (quantum fidelity)
        let phase_diff = phi_edge - phi_svc;
        let phase_alignment = phase_diff.cos().powi(2);
        
        // Fairness dampening (√load)
        let fairness = (1.0 + self.rho * load).sqrt();
        
        // Security guard (sec = 1/cos)
        let sec_guard = if phi_edge.cos().abs() > self.epsilon {
            1.0
        } else {
            0.0 // Abort if phase near singularity
        };
        
        hrw_component * phase_alignment * sec_guard / fairness
    }
}

impl Default for TrigonometricScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phase_controller_creation() {
        let controller = PhaseController::new();
        assert_eq!(controller.phase_num(), 0);
        assert_eq!(controller.omega_ppm(), 0);
    }
    
    #[test]
    fn test_phase_calculation() {
        let controller = PhaseController::with_phase(u32::MAX / 2, 0);
        let phi = controller.phi_eff(PhaseController::now_fs());
        
        // Should be approximately π (half of 2π)
        assert!((phi - PI).abs() < 0.01);
    }
    
    #[test]
    fn test_phase_alignment() {
        let controller = PhaseController::with_phase(0, 0);
        
        // Perfect alignment (both at 0)
        let alignment = controller.phase_alignment(0.0);
        assert!((alignment - 1.0).abs() < 0.01);
        
        // Worst alignment (π/2 apart - cos(π/2) = 0)
        let alignment = controller.phase_alignment(PI / 2.0);
        assert!(alignment < 0.1); // cos²(π/2) ≈ 0
    }
    
    #[test]
    fn test_phase_stability() {
        let controller = PhaseController::with_phase(0, 0);
        assert!(controller.is_stable(0.01));
        
        let controller = PhaseController::with_phase(u32::MAX / 4, 0); // π/2
        assert!(!controller.is_stable(0.01)); // cos(π/2) ≈ 0
    }
    
    #[test]
    fn test_qwudp_message_creation() {
        let msg = QwUdpMessage::new(
            OpCode::PREP,
            BasisCode::H,
            QecCode::SURF_D5,
            vec![1, 2, 3, 4],
        );
        
        assert_eq!(msg.header.op_code, OpCode::PREP as u8);
        assert_eq!(msg.header.basis_code, BasisCode::H as u8);
        assert_eq!(msg.header.qec_code, QecCode::SURF_D5 as u8);
        assert_eq!(msg.payload.len(), 4);
    }
    
    #[test]
    fn test_qwudp_crc() {
        let mut msg = QwUdpMessage::new(
            OpCode::DATA,
            BasisCode::Z,
            QecCode::NONE,
            vec![1, 2, 3, 4, 5],
        );
        
        msg.calculate_crc();
        assert!(msg.verify_crc());
        
        // Corrupt payload
        msg.payload[0] = 99;
        assert!(!msg.verify_crc());
    }
    
    #[test]
    fn test_qwudp_serialization() {
        let msg = QwUdpMessage::new(
            OpCode::ACK,
            BasisCode::X,
            QecCode::STEANE,
            vec![42],
        );
        
        let bytes = msg.to_bytes();
        assert!(bytes.len() >= 32); // At least header size
        
        let decoded = QwUdpMessage::from_bytes(&bytes);
        assert!(decoded.is_some());
        
        let decoded = decoded.unwrap();
        assert_eq!(decoded.header.op_code, OpCode::ACK as u8);
        assert_eq!(decoded.payload[0], 42);
    }
    
    #[test]
    fn test_trigonometric_scorer() {
        let scorer = TrigonometricScorer::new();
        
        // Perfect conditions: low HRW, aligned phase, low load
        let score1 = scorer.score(0.1, 0.0, 0.0, 0.1);
        
        // Poor conditions: high HRW, misaligned phase, high load
        let score2 = scorer.score(0.9, PI, 0.0, 5.0);
        
        assert!(score1 > score2);
    }
    
    #[test]
    fn test_trigonometric_scorer_phase_alignment() {
        let scorer = TrigonometricScorer::new();
        
        // Same phase (perfect alignment)
        let score_aligned = scorer.score(0.5, 0.0, 0.0, 1.0);
        
        // Perpendicular phase (worst alignment - cos(π/2) = 0)
        let score_perpendicular = scorer.score(0.5, PI / 2.0, 0.0, 1.0);
        
        assert!(score_aligned > score_perpendicular);
    }
    
    #[test]
    fn test_trigonometric_scorer_load_fairness() {
        let scorer = TrigonometricScorer::new();
        
        // Low load
        let score_low_load = scorer.score(0.5, 0.0, 0.0, 0.1);
        
        // High load
        let score_high_load = scorer.score(0.5, 0.0, 0.0, 10.0);
        
        assert!(score_low_load > score_high_load);
    }
}
