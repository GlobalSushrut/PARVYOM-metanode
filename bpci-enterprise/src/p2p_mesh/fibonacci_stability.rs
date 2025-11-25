//! Fibonacci-Stability P2P Handshake Protocol
//! 
//! Uses Fibonacci sequence and golden ratio (φ) for stability-based mesh formation.
//! 
//! # Key Concepts
//! 
//! - **Fibonacci Windows**: Rolling stability metrics over F_k second windows (2,3,5,8,13,21,34,...)
//! - **Golden Ratio Thresholds**: Accept at φ⁻¹ ≈ 0.618, probation at φ⁻² ≈ 0.382
//! - **Witness Quorums**: F_m endorsements (2,3,5) for admission confidence
//! - **Resource Credits**: Fibonacci-graded lease duration and credits
//! - **Fibonacci Backoff**: Exponential backoff using F_k sequence
//! 
//! # Stability Score
//! 
//! ```text
//! S_ij = Σ(F_w · s_w) / Σ(F_w)
//! 
//! where s_w = exp(-α·σ_w/(R_w+ε)) · (1-L_w)^β · U_w^γ · exp(-δ·C_w)
//! ```

use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Fibonacci sequence (first 15 terms)
pub const FIBONACCI: [u64; 15] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];

/// Golden ratio φ ≈ 1.618
pub const PHI: f64 = 1.618033988749895;

/// φ⁻¹ ≈ 0.618 (accept threshold)
pub const PHI_INV: f64 = 0.618033988749895;

/// φ⁻² ≈ 0.382 (probation threshold)
pub const PHI_INV2: f64 = 0.381966011250105;

/// Stability score parameters
#[derive(Debug, Clone)]
pub struct StabilityParams {
    /// Jitter sensitivity (α)
    pub alpha: f64,
    
    /// Loss sensitivity (β)
    pub beta: f64,
    
    /// Uptime sensitivity (γ)
    pub gamma: f64,
    
    /// Churn sensitivity (δ)
    pub delta: f64,
    
    /// Witness blend weight (λ)
    pub lambda: f64,
}

impl Default for StabilityParams {
    fn default() -> Self {
        Self {
            alpha: 1.2,
            beta: 1.0,
            gamma: 0.8,
            delta: 0.5,
            lambda: 0.7,
        }
    }
}

/// Window metrics for stability calculation
#[derive(Debug, Clone)]
pub struct WindowMetrics {
    /// Window duration (Fibonacci seconds)
    pub window: u64,
    
    /// Packet loss fraction [0,1]
    pub loss: f64,
    
    /// RTT median (milliseconds)
    pub rtt_median: f64,
    
    /// RTT MAD (median absolute deviation)
    pub rtt_mad: f64,
    
    /// Uptime fraction [0,1]
    pub uptime: f64,
    
    /// Reconnections per minute
    pub churn: f64,
}

/// Rolling stability tracker
#[derive(Debug, Clone)]
pub struct StabilityTracker {
    /// Stability parameters
    params: StabilityParams,
    
    /// Fibonacci windows to track (indices into FIBONACCI)
    window_indices: Vec<usize>,
    
    /// Metrics per window
    window_metrics: Vec<WindowMetrics>,
    
    /// Last update time
    #[allow(dead_code)]
    last_update: Instant,
}

impl StabilityTracker {
    /// Create a new stability tracker
    pub fn new(params: StabilityParams) -> Self {
        // Use windows: F_2, F_3, F_4, F_5, F_6 = 2, 3, 5, 8, 13 seconds
        let window_indices = vec![2, 3, 4, 5, 6];
        
        let window_metrics = window_indices
            .iter()
            .map(|&idx| WindowMetrics {
                window: FIBONACCI[idx],
                loss: 0.0,
                rtt_median: 0.0,
                rtt_mad: 0.0,
                uptime: 1.0,
                churn: 0.0,
            })
            .collect();
        
        Self {
            params,
            window_indices,
            window_metrics,
            last_update: Instant::now(),
        }
    }
    
    /// Update metrics for a specific window
    pub fn update_window(
        &mut self,
        window_idx: usize,
        loss: f64,
        rtt_median: f64,
        rtt_mad: f64,
        uptime: f64,
        churn: f64,
    ) {
        if let Some(metrics) = self.window_metrics.get_mut(window_idx) {
            metrics.loss = loss.clamp(0.0, 1.0);
            metrics.rtt_median = rtt_median.max(0.0);
            metrics.rtt_mad = rtt_mad.max(0.0);
            metrics.uptime = uptime.clamp(0.0, 1.0);
            metrics.churn = churn.max(0.0);
        }
        self.last_update = Instant::now();
    }
    
    /// Calculate per-window score
    fn window_score(&self, metrics: &WindowMetrics) -> f64 {
        let epsilon = 1e-6;
        
        // Jitter term: exp(-α·σ/(R+ε))
        let jitter_term = (-self.params.alpha * metrics.rtt_mad / (metrics.rtt_median + epsilon)).exp();
        
        // Loss term: (1-L)^β
        let loss_term = (1.0 - metrics.loss).powf(self.params.beta);
        
        // Uptime term: U^γ
        let uptime_term = metrics.uptime.powf(self.params.gamma);
        
        // Churn term: exp(-δ·C)
        let churn_term = (-self.params.delta * metrics.churn).exp();
        
        jitter_term * loss_term * uptime_term * churn_term
    }
    
    /// Calculate Fibonacci-weighted stability score
    pub fn calculate_score(&self) -> f64 {
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        
        for (idx, metrics) in self.window_metrics.iter().enumerate() {
            let window_idx = self.window_indices[idx];
            let weight = FIBONACCI[window_idx] as f64;
            let score = self.window_score(metrics);
            
            numerator += weight * score;
            denominator += weight;
        }
        
        if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        }
    }
}

/// Witness endorsement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessEndorsement {
    /// Witness node ID
    pub witness_id: String,
    
    /// Target node ID being endorsed
    pub target_id: String,
    
    /// Witness's stability score for target
    pub stability_score: f64,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// HMAC signature
    pub signature: [u8; 32],
}

/// Calculate blended score with witness quorum
pub fn blended_score(
    link_score: f64,
    witnesses: &[WitnessEndorsement],
    lambda: f64,
) -> f64 {
    if witnesses.is_empty() {
        return link_score;
    }
    
    // Qi = 1 - Π(1 - S_wi)
    let mut product = 1.0;
    for witness in witnesses {
        product *= 1.0 - witness.stability_score;
    }
    let qi = 1.0 - product;
    
    // Blend: λ·S_ij + (1-λ)·Q_i
    lambda * link_score + (1.0 - lambda) * qi
}

/// Admission decision based on golden ratio thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionDecision {
    /// Full accept (S ≥ φ⁻¹)
    Accept,
    
    /// Probation (φ⁻² ≤ S < φ⁻¹)
    Probation,
    
    /// Reject (S < φ⁻²)
    Reject,
}

/// Determine admission decision
pub fn admission_decision(score: f64) -> AdmissionDecision {
    if score >= PHI_INV {
        AdmissionDecision::Accept
    } else if score >= PHI_INV2 {
        AdmissionDecision::Probation
    } else {
        AdmissionDecision::Reject
    }
}

/// Calculate Fibonacci lease duration
pub fn lease_duration(score: f64) -> Duration {
    if score < PHI_INV2 {
        return Duration::from_secs(0);
    }
    
    // Map score to Fibonacci step (5..9 for 8s..89s)
    let normalized = (score - PHI_INV2) / PHI_INV;
    let step = 5 + (4.0 * normalized).floor() as usize;
    let step = step.clamp(5, 9);
    
    Duration::from_secs(FIBONACCI[step])
}

/// Calculate resource credits
pub fn resource_credits(score: f64, max_credits: f64, eta: f64) -> f64 {
    if score < PHI_INV2 {
        return 0.0;
    }
    
    max_credits * score.powf(eta)
}

/// Fibonacci backoff calculator
#[derive(Debug, Clone)]
pub struct FibonacciBackoff {
    /// Consecutive failures
    failures: usize,
}

impl FibonacciBackoff {
    /// Create a new backoff calculator
    pub fn new() -> Self {
        Self { failures: 0 }
    }
    
    /// Record a failure
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    
    /// Record a success (reset)
    pub fn record_success(&mut self) {
        self.failures = 0;
    }
    
    /// Get next backoff duration
    pub fn next_backoff(&self) -> Duration {
        // F_1, F_2, F_3, ... capped at F_10 = 55s
        let idx = (1 + self.failures).min(10);
        Duration::from_secs(FIBONACCI[idx])
    }
}

/// Fibonacci fanout calculator
pub fn fibonacci_fanout(under_connected: bool) -> usize {
    if under_connected {
        FIBONACCI[3] as usize // F_3 = 3
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci_constants() {
        assert_eq!(FIBONACCI[0], 1);
        assert_eq!(FIBONACCI[5], 8);
        assert_eq!(FIBONACCI[10], 89);
        
        assert!((PHI - 1.618).abs() < 0.001);
        assert!((PHI_INV - 0.618).abs() < 0.001);
        assert!((PHI_INV2 - 0.382).abs() < 0.001);
    }
    
    #[test]
    fn test_stability_tracker() {
        let params = StabilityParams::default();
        let mut tracker = StabilityTracker::new(params);
        
        // Update with good metrics
        tracker.update_window(0, 0.01, 25.0, 2.0, 0.99, 0.0);
        
        let score = tracker.calculate_score();
        assert!(score > 0.8);
        assert!(score <= 1.0);
    }
    
    #[test]
    fn test_admission_decision() {
        assert_eq!(admission_decision(0.7), AdmissionDecision::Accept);
        assert_eq!(admission_decision(0.5), AdmissionDecision::Probation);
        assert_eq!(admission_decision(0.3), AdmissionDecision::Reject);
    }
    
    #[test]
    fn test_blended_score() {
        let link_score = 0.84;
        let witnesses = vec![
            WitnessEndorsement {
                witness_id: "w1".to_string(),
                target_id: "t1".to_string(),
                stability_score: 0.9,
                timestamp: 0,
                signature: [0u8; 32],
            },
            WitnessEndorsement {
                witness_id: "w2".to_string(),
                target_id: "t1".to_string(),
                stability_score: 0.85,
                timestamp: 0,
                signature: [0u8; 32],
            },
        ];
        
        let blended = blended_score(link_score, &witnesses, 0.7);
        
        // Should be between link_score and 1.0
        assert!(blended >= link_score);
        assert!(blended <= 1.0);
    }
    
    #[test]
    fn test_lease_duration() {
        let duration_high = lease_duration(0.9);
        let duration_mid = lease_duration(0.5);
        let duration_low = lease_duration(0.3);
        
        assert!(duration_high > duration_mid);
        assert_eq!(duration_low.as_secs(), 0);
    }
    
    #[test]
    fn test_resource_credits() {
        let credits_high = resource_credits(0.9, 1000.0, 1.5);
        let credits_mid = resource_credits(0.5, 1000.0, 1.5);
        let credits_low = resource_credits(0.3, 1000.0, 1.5);
        
        assert!(credits_high > credits_mid);
        assert_eq!(credits_low, 0.0);
    }
    
    #[test]
    fn test_fibonacci_backoff() {
        let mut backoff = FibonacciBackoff::new();
        
        let b1 = backoff.next_backoff();
        backoff.record_failure();
        let b2 = backoff.next_backoff();
        backoff.record_failure();
        let b3 = backoff.next_backoff();
        
        assert!(b2 > b1);
        assert!(b3 > b2);
        
        backoff.record_success();
        let b4 = backoff.next_backoff();
        assert_eq!(b4, b1);
    }
    
    #[test]
    fn test_fibonacci_fanout() {
        assert_eq!(fibonacci_fanout(true), 3);
        assert_eq!(fibonacci_fanout(false), 1);
    }
    
    #[test]
    fn test_worked_example() {
        // From the spec: windows 5,8,13,21 with specific metrics
        let params = StabilityParams::default();
        let mut tracker = StabilityTracker::new(params);
        
        // Window 5s: loss=0.00, rtt=24, mad=2, uptime=0.99, churn=0.0
        tracker.update_window(0, 0.00, 24.0, 2.0, 0.99, 0.0);
        
        // Window 8s: loss=0.02, rtt=26, mad=3, uptime=0.98, churn=0.0
        tracker.update_window(1, 0.02, 26.0, 3.0, 0.98, 0.0);
        
        // Window 13s: loss=0.01, rtt=27, mad=4, uptime=0.97, churn=0.1
        tracker.update_window(2, 0.01, 27.0, 4.0, 0.97, 0.1);
        
        let score = tracker.calculate_score();
        
        // Should be around 0.84 per spec
        assert!(score > 0.75);
        assert!(score < 0.95);
    }
}
