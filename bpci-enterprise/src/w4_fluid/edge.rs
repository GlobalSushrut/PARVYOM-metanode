//! Edge State for Fluid Dynamics
//! 
//! Tracks fluid properties (w, ν, Θ, κ, ρ) and telemetry for network edges.

use super::ema::ExponentialMovingAverage;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Edge identifier (source → destination)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId {
    pub source: u64,
    pub dest: u64,
}

impl EdgeId {
    pub fn new(source: u64, dest: u64) -> Self {
        Self { source, dest }
    }
}

/// Edge state with fluid properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeState {
    /// Edge identifier
    pub id: EdgeId,
    
    /// Weight (w) - metric for routing
    pub weight: f64,
    
    /// Viscosity (ν) - resistance to flow
    pub viscosity: f64,
    
    /// Temperature (Θ) - disorder/jitter
    pub temperature: f64,
    
    /// Curvature (κ) - Forman/Ollivier-Ricci curvature
    pub curvature: f64,
    
    /// Density (ρ) - traffic load
    pub density: f64,
    
    /// Current load (0.0 - 1.0)
    pub load: f64,
    
    /// Telemetry data
    pub telemetry: EdgeTelemetry,
    
    /// Last update timestamp
    #[serde(skip)]
    pub last_update: Option<Instant>,
}

/// Edge telemetry with EMA tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTelemetry {
    /// Latency EMA (milliseconds)
    pub latency_ema: ExponentialMovingAverage,
    
    /// Packet loss EMA (0.0 - 1.0)
    pub loss_ema: ExponentialMovingAverage,
    
    /// Jitter EMA (milliseconds)
    pub jitter_ema: ExponentialMovingAverage,
    
    /// Queue depth EMA (packets)
    pub queue_depth_ema: ExponentialMovingAverage,
    
    /// Capacity (Mbps)
    pub capacity: f64,
    
    /// Total packets sent
    pub packets_sent: u64,
    
    /// Total packets lost
    pub packets_lost: u64,
}

impl EdgeState {
    /// Create a new edge state
    pub fn new(id: EdgeId, capacity: f64) -> Self {
        Self {
            id,
            weight: 1.0,
            viscosity: 0.03,      // ν_base
            temperature: 0.0,
            curvature: 0.0,
            density: 0.0,
            load: 0.0,
            telemetry: EdgeTelemetry::new(capacity),
            last_update: Some(Instant::now()),
        }
    }
    
    /// Update telemetry with new measurements
    pub fn update_telemetry(
        &mut self,
        latency_ms: f64,
        loss_rate: f64,
        jitter_ms: f64,
        queue_depth: f64,
    ) {
        self.telemetry.latency_ema.update(latency_ms);
        self.telemetry.loss_ema.update(loss_rate);
        self.telemetry.jitter_ema.update(jitter_ms);
        self.telemetry.queue_depth_ema.update(queue_depth);
        
        // Update temperature from jitter (disorder)
        self.temperature = self.telemetry.jitter_ema.value();
        
        // Update load from queue depth
        self.load = (queue_depth / 100.0).min(1.0); // Normalize to 0-1
        
        self.last_update = Some(Instant::now());
    }
    
    /// Calculate viscosity from telemetry
    /// 
    /// ν = ν_base + a·queue + b·jitter
    pub fn calculate_viscosity(&mut self, nu_base: f64, a: f64, b: f64) {
        let queue_term = a * self.telemetry.queue_depth_ema.value();
        let jitter_term = b * self.telemetry.jitter_ema.value();
        
        self.viscosity = (nu_base + queue_term + jitter_term)
            .max(0.01)  // min viscosity
            .min(1.0);  // max viscosity
    }
    
    /// Check if edge needs healing (high temperature)
    pub fn needs_healing(&self, threshold: f64) -> bool {
        self.temperature > threshold
    }
    
    /// Get current latency
    pub fn latency(&self) -> f64 {
        self.telemetry.latency_ema.value()
    }
    
    /// Get current loss rate
    pub fn loss_rate(&self) -> f64 {
        self.telemetry.loss_ema.value()
    }
    
    /// Get current jitter
    pub fn jitter(&self) -> f64 {
        self.telemetry.jitter_ema.value()
    }
    
    /// Get current queue depth
    pub fn queue_depth(&self) -> f64 {
        self.telemetry.queue_depth_ema.value()
    }
}

impl EdgeTelemetry {
    /// Create new telemetry tracker
    pub fn new(capacity: f64) -> Self {
        Self {
            latency_ema: ExponentialMovingAverage::new(0.3),
            loss_ema: ExponentialMovingAverage::new(0.3),
            jitter_ema: ExponentialMovingAverage::new(0.3),
            queue_depth_ema: ExponentialMovingAverage::new(0.3),
            capacity,
            packets_sent: 0,
            packets_lost: 0,
        }
    }
    
    /// Record packet transmission
    pub fn record_packet(&mut self, lost: bool) {
        self.packets_sent += 1;
        if lost {
            self.packets_lost += 1;
        }
    }
    
    /// Get actual loss rate from counters
    pub fn actual_loss_rate(&self) -> f64 {
        if self.packets_sent == 0 {
            0.0
        } else {
            self.packets_lost as f64 / self.packets_sent as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edge_creation() {
        let id = EdgeId::new(1, 2);
        let edge = EdgeState::new(id, 1000.0);
        
        assert_eq!(edge.id.source, 1);
        assert_eq!(edge.id.dest, 2);
        assert_eq!(edge.weight, 1.0);
        assert_eq!(edge.viscosity, 0.03);
        assert_eq!(edge.telemetry.capacity, 1000.0);
    }
    
    #[test]
    fn test_telemetry_update() {
        let id = EdgeId::new(1, 2);
        let mut edge = EdgeState::new(id, 1000.0);
        
        edge.update_telemetry(10.0, 0.01, 2.0, 5.0);
        
        assert_eq!(edge.latency(), 10.0);
        assert_eq!(edge.loss_rate(), 0.01);
        assert_eq!(edge.jitter(), 2.0);
        assert_eq!(edge.queue_depth(), 5.0);
        assert_eq!(edge.temperature, 2.0); // Temperature = jitter
    }
    
    #[test]
    fn test_viscosity_calculation() {
        let id = EdgeId::new(1, 2);
        let mut edge = EdgeState::new(id, 1000.0);
        
        edge.update_telemetry(10.0, 0.01, 5.0, 10.0);
        edge.calculate_viscosity(0.03, 0.01, 0.02);
        
        // ν = 0.03 + 0.01*10 + 0.02*5 = 0.03 + 0.1 + 0.1 = 0.23
        assert!((edge.viscosity - 0.23).abs() < 0.01);
    }
    
    #[test]
    fn test_viscosity_bounds() {
        let id = EdgeId::new(1, 2);
        let mut edge = EdgeState::new(id, 1000.0);
        
        // Test max bound
        edge.update_telemetry(10.0, 0.01, 100.0, 100.0);
        edge.calculate_viscosity(0.03, 1.0, 1.0);
        assert_eq!(edge.viscosity, 1.0); // Capped at max
        
        // Test min bound
        edge.update_telemetry(10.0, 0.01, 0.0, 0.0);
        edge.calculate_viscosity(0.0, 0.0, 0.0);
        assert_eq!(edge.viscosity, 0.01); // Capped at min
    }
    
    #[test]
    fn test_healing_trigger() {
        let id = EdgeId::new(1, 2);
        let mut edge = EdgeState::new(id, 1000.0);
        
        edge.update_telemetry(10.0, 0.01, 5.0, 10.0);
        assert!(!edge.needs_healing(10.0));
        
        edge.update_telemetry(10.0, 0.01, 15.0, 10.0);
        assert!(edge.needs_healing(10.0));
    }
    
    #[test]
    fn test_packet_tracking() {
        let mut telemetry = EdgeTelemetry::new(1000.0);
        
        telemetry.record_packet(false);
        telemetry.record_packet(false);
        telemetry.record_packet(true);
        telemetry.record_packet(false);
        
        assert_eq!(telemetry.packets_sent, 4);
        assert_eq!(telemetry.packets_lost, 1);
        assert_eq!(telemetry.actual_loss_rate(), 0.25);
    }
    
    #[test]
    fn test_load_calculation() {
        let id = EdgeId::new(1, 2);
        let mut edge = EdgeState::new(id, 1000.0);
        
        edge.update_telemetry(10.0, 0.01, 2.0, 50.0);
        assert_eq!(edge.load, 0.5); // 50/100 = 0.5
        
        edge.update_telemetry(10.0, 0.01, 2.0, 150.0);
        assert_eq!(edge.load, 1.0); // Capped at 1.0
    }
}
