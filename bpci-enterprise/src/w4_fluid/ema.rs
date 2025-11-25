//! Exponential Moving Average (EMA) for Telemetry
//! 
//! Provides smoothed telemetry tracking for fluid dynamics.
//! Used to track latency, loss, jitter, and queue depth over time.

use serde::{Deserialize, Serialize};

/// Exponential Moving Average tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExponentialMovingAverage {
    /// Current EMA value
    value: f64,
    
    /// Decay factor (alpha): 0.0 - 1.0
    /// Higher alpha = more weight to recent samples
    alpha: f64,
    
    /// Number of samples processed
    sample_count: u64,
}

impl ExponentialMovingAverage {
    /// Create a new EMA with specified decay factor
    /// 
    /// # Arguments
    /// * `alpha` - Decay factor (0.0 - 1.0). Typical values:
    ///   - 0.1: Slow adaptation (10% weight to new samples)
    ///   - 0.3: Medium adaptation
    ///   - 0.5: Fast adaptation (50% weight to new samples)
    pub fn new(alpha: f64) -> Self {
        assert!(alpha > 0.0 && alpha <= 1.0, "Alpha must be in (0, 1]");
        
        Self {
            value: 0.0,
            alpha,
            sample_count: 0,
        }
    }
    
    /// Create a new EMA with default alpha (0.3)
    pub fn default() -> Self {
        Self::new(0.3)
    }
    
    /// Update the EMA with a new sample
    /// 
    /// Formula: EMA_new = alpha * sample + (1 - alpha) * EMA_old
    pub fn update(&mut self, sample: f64) {
        if self.sample_count == 0 {
            // First sample: initialize to sample value
            self.value = sample;
        } else {
            // EMA update
            self.value = self.alpha * sample + (1.0 - self.alpha) * self.value;
        }
        
        self.sample_count += 1;
    }
    
    /// Get the current EMA value
    pub fn value(&self) -> f64 {
        self.value
    }
    
    /// Get the number of samples processed
    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }
    
    /// Get the alpha (decay factor)
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    
    /// Reset the EMA to initial state
    pub fn reset(&mut self) {
        self.value = 0.0;
        self.sample_count = 0;
    }
    
    /// Set a new alpha value
    pub fn set_alpha(&mut self, alpha: f64) {
        assert!(alpha > 0.0 && alpha <= 1.0, "Alpha must be in (0, 1]");
        self.alpha = alpha;
    }
}

/// EMA configuration presets
pub struct EmaPresets;

impl EmaPresets {
    /// Slow adaptation (alpha = 0.1)
    /// Good for: Long-term trends, stable metrics
    pub fn slow() -> ExponentialMovingAverage {
        ExponentialMovingAverage::new(0.1)
    }
    
    /// Medium adaptation (alpha = 0.3)
    /// Good for: General purpose, balanced responsiveness
    pub fn medium() -> ExponentialMovingAverage {
        ExponentialMovingAverage::new(0.3)
    }
    
    /// Fast adaptation (alpha = 0.5)
    /// Good for: Rapid changes, congestion detection
    pub fn fast() -> ExponentialMovingAverage {
        ExponentialMovingAverage::new(0.5)
    }
    
    /// Very fast adaptation (alpha = 0.7)
    /// Good for: Real-time response, immediate feedback
    pub fn very_fast() -> ExponentialMovingAverage {
        ExponentialMovingAverage::new(0.7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ema_creation() {
        let ema = ExponentialMovingAverage::new(0.3);
        assert_eq!(ema.value(), 0.0);
        assert_eq!(ema.sample_count(), 0);
        assert_eq!(ema.alpha(), 0.3);
    }
    
    #[test]
    fn test_ema_first_sample() {
        let mut ema = ExponentialMovingAverage::new(0.3);
        ema.update(10.0);
        
        assert_eq!(ema.value(), 10.0);
        assert_eq!(ema.sample_count(), 1);
    }
    
    #[test]
    fn test_ema_updates() {
        let mut ema = ExponentialMovingAverage::new(0.5);
        
        ema.update(10.0);
        assert_eq!(ema.value(), 10.0);
        
        ema.update(20.0);
        // 0.5 * 20.0 + 0.5 * 10.0 = 15.0
        assert_eq!(ema.value(), 15.0);
        
        ema.update(10.0);
        // 0.5 * 10.0 + 0.5 * 15.0 = 12.5
        assert_eq!(ema.value(), 12.5);
    }
    
    #[test]
    fn test_ema_convergence() {
        let mut ema = ExponentialMovingAverage::new(0.3);
        
        // Feed constant value
        for _ in 0..100 {
            ema.update(50.0);
        }
        
        // Should converge to 50.0
        assert!((ema.value() - 50.0).abs() < 0.01);
    }
    
    #[test]
    fn test_ema_reset() {
        let mut ema = ExponentialMovingAverage::new(0.3);
        
        ema.update(10.0);
        ema.update(20.0);
        assert_ne!(ema.value(), 0.0);
        
        ema.reset();
        assert_eq!(ema.value(), 0.0);
        assert_eq!(ema.sample_count(), 0);
    }
    
    #[test]
    fn test_ema_alpha_change() {
        let mut ema = ExponentialMovingAverage::new(0.3);
        
        ema.update(10.0);
        ema.set_alpha(0.7);
        
        assert_eq!(ema.alpha(), 0.7);
        
        ema.update(20.0);
        // 0.7 * 20.0 + 0.3 * 10.0 = 17.0
        assert_eq!(ema.value(), 17.0);
    }
    
    #[test]
    fn test_ema_presets() {
        let slow = EmaPresets::slow();
        assert_eq!(slow.alpha(), 0.1);
        
        let medium = EmaPresets::medium();
        assert_eq!(medium.alpha(), 0.3);
        
        let fast = EmaPresets::fast();
        assert_eq!(fast.alpha(), 0.5);
        
        let very_fast = EmaPresets::very_fast();
        assert_eq!(very_fast.alpha(), 0.7);
    }
    
    #[test]
    fn test_ema_responsiveness() {
        let mut slow = EmaPresets::slow();
        let mut fast = EmaPresets::fast();
        
        // Initialize both to 10.0
        slow.update(10.0);
        fast.update(10.0);
        
        // Sudden spike to 100.0
        slow.update(100.0);
        fast.update(100.0);
        
        // Fast should respond more
        assert!(fast.value() > slow.value());
        
        // Slow: 0.1 * 100 + 0.9 * 10 = 19.0
        assert_eq!(slow.value(), 19.0);
        
        // Fast: 0.5 * 100 + 0.5 * 10 = 55.0
        assert_eq!(fast.value(), 55.0);
    }
    
    #[test]
    #[should_panic(expected = "Alpha must be in (0, 1]")]
    fn test_invalid_alpha_zero() {
        ExponentialMovingAverage::new(0.0);
    }
    
    #[test]
    #[should_panic(expected = "Alpha must be in (0, 1]")]
    fn test_invalid_alpha_negative() {
        ExponentialMovingAverage::new(-0.1);
    }
    
    #[test]
    #[should_panic(expected = "Alpha must be in (0, 1]")]
    fn test_invalid_alpha_too_large() {
        ExponentialMovingAverage::new(1.5);
    }
}
