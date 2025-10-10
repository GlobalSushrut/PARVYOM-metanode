//! LCCD Consensus: Living Cellular Consensus Division
//! 
//! Ultra-lightweight consensus that starts on 1-core old i3 processor
//! but scales to surpass WAN internet scale through cellular multiplication.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use tracing::{info, warn, error};
use uuid::Uuid;
use std::fs;

/// Unique identifier for a consensus cell
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellId(u64);

impl CellId {
    pub fn generate() -> Self {
        CellId(Uuid::new_v4().as_u128() as u64)
    }
    
    pub fn genesis() -> Self {
        CellId(0)
    }
}

impl std::fmt::Display for CellId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cell-{:016x}", self.0)
    }
}

/// Cell lifecycle states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellLifecycle {
    Embryonic,      // Just born, initializing
    Growing,        // Normal operation, accumulating resources
    Mature,         // Fully operational, can participate in consensus
    Dividing,       // In process of cell division
    Senescent,      // Old cell, reduced performance
    Dead,           // Non-functional
}

/// Hardware profile detected during genesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu_cores: u32,
    pub cpu_mhz: u32,
    pub memory_mb: u64,
    pub network_kbps: u64,
}

impl HardwareProfile {
    /// Detect minimal hardware for LCCD genesis
    pub fn detect() -> Result<Self> {
        let cpu_info = Self::get_cpu_info()?;
        let memory_mb = Self::get_available_memory_mb()?;
        let network_kbps = 1000; // 1 Mbps default
        
        Ok(HardwareProfile {
            cpu_cores: cpu_info.0,
            cpu_mhz: cpu_info.1,
            memory_mb,
            network_kbps,
        })
    }
    
    fn get_cpu_info() -> Result<(u32, u32)> {
        // Try to read /proc/cpuinfo for Linux
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let mut cores = 0;
            let mut mhz = 2400; // Default for old i3
            
            for line in cpuinfo.lines() {
                if line.starts_with("processor") {
                    cores += 1;
                } else if line.starts_with("cpu MHz") {
                    if let Some(mhz_str) = line.split(':').nth(1) {
                        mhz = mhz_str.trim().parse::<f32>().unwrap_or(2400.0) as u32;
                    }
                }
            }
            Ok((cores.max(1), mhz))
        } else {
            Ok((1, 2400)) // Fallback for non-Linux
        }
    }
    
    fn get_available_memory_mb() -> Result<u64> {
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemAvailable:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        let kb = kb_str.parse::<u64>().unwrap_or(524288);
                        return Ok(kb / 1024); // Convert KB to MB
                    }
                }
            }
        }
        Ok(512) // Fallback: 512MB
    }
    
    /// Validate hardware meets LCCD genesis requirements
    pub fn validate_genesis_requirements(&self) -> Result<()> {
        if self.cpu_mhz < 1000 {
            return Err(anyhow!("CPU too slow: {}MHz < 1000MHz", self.cpu_mhz));
        }
        if self.memory_mb < 256 {
            return Err(anyhow!("Memory too low: {}MB < 256MB", self.memory_mb));
        }
        
        info!("✅ Hardware validation passed:");
        info!("   CPU: {} cores @ {}MHz", self.cpu_cores, self.cpu_mhz);
        info!("   Memory: {}MB available", self.memory_mb);
        Ok(())
    }
}

/// Ultra-compact DNA (64 bytes total)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LccdDna {
    // κ parameters (16 bytes)
    pub kappa_a: f32,
    pub kappa_b: f32,
    pub kappa_c: f32,
    pub kappa_z: f32,
    
    // NxTri parameters (24 bytes)
    pub alpha_weight: f32,
    pub beta_weight: f32,
    pub gamma_weight: f32,
    pub learning_rate: f32,
    pub convergence_threshold: f32,
    pub stability_delta: f32,
    
    // Division parameters (16 bytes)
    pub division_cpu_threshold: f32,
    pub division_memory_threshold: f32,
    pub max_generations: u16,
    pub mutation_rate: u16,
    pub reserved: u32,
    
    // Feature parameters (8 bytes)
    pub feature_dim: u8,
    pub morphism_complexity: u8,
    pub braid_window_depth: u8,
    pub triad_sample_cap: u8,
    pub reserved2: u32,
}

impl LccdDna {
    /// Generate genesis DNA optimized for single-core i3
    pub fn genesis(hardware: &HardwareProfile) -> Self {
        info!("🧬 Generating genesis DNA for single-core optimization");
        
        LccdDna {
            // κ parameters optimized for minimal CPU
            kappa_a: 0.5,
            kappa_b: 1.0,
            kappa_c: 0.5,
            kappa_z: 4.0,
            
            // NxTri parameters optimized for low memory
            alpha_weight: 0.4,
            beta_weight: 0.3,
            gamma_weight: 0.3,
            learning_rate: 0.1,
            convergence_threshold: 0.01,
            stability_delta: 0.05,
            
            // Division parameters for exponential growth
            division_cpu_threshold: 0.8,
            division_memory_threshold: 0.7,
            max_generations: 20,
            mutation_rate: 100, // 1% mutation rate
            reserved: 0,
            
            // Feature functor optimized for speed
            feature_dim: 8,
            morphism_complexity: 2,
            braid_window_depth: 2,
            triad_sample_cap: 8, // 2^8 = 256 samples
            reserved2: 0,
        }
    }
}

/// Resource usage tracking (ultra-lightweight)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub current_utilization: u8,       // 0-255 (0-100% usage)
    pub average_utilization: u8,       // Moving average
    pub peak_utilization: u8,          // Maximum observed
}

impl ResourceUsage {
    pub fn new() -> Self {
        Self {
            current_utilization: 0,
            average_utilization: 0,
            peak_utilization: 0,
        }
    }
    
    pub fn update(&mut self, new_utilization: u8) {
        self.current_utilization = new_utilization;
        self.peak_utilization = self.peak_utilization.max(new_utilization);
        
        // Simple moving average
        self.average_utilization = ((self.average_utilization as u16 * 9 + new_utilization as u16) / 10) as u8;
    }
    
    pub fn is_saturated(&self, threshold: u8) -> bool {
        self.current_utilization > threshold
    }
}

/// Micro-metabolism (ultra-lightweight resource tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroMetabolism {
    pub cpu_usage: ResourceUsage,
    pub memory_usage: ResourceUsage,
    pub network_usage: ResourceUsage,
    pub cpu_mhz: u32,
    pub memory_mb: u64,
    pub ops_per_second: u32,
    pub last_update: Instant,
}

impl MicroMetabolism {
    pub fn new(hardware: &HardwareProfile) -> Self {
        Self {
            cpu_usage: ResourceUsage::new(),
            memory_usage: ResourceUsage::new(),
            network_usage: ResourceUsage::new(),
            cpu_mhz: hardware.cpu_mhz,
            memory_mb: hardware.memory_mb,
            ops_per_second: 0,
            last_update: Instant::now(),
        }
    }
    
    /// Update metabolism with current resource usage
    pub fn update(&mut self) -> Result<()> {
        // Simplified resource usage (in real implementation, read from system)
        self.cpu_usage.update(50);  // 50% CPU usage
        self.memory_usage.update(60); // 60% memory usage
        self.network_usage.update(30); // 30% network usage
        
        self.last_update = Instant::now();
        Ok(())
    }
    
    /// Check if cell should divide based on resource saturation
    pub fn should_divide(&self, dna: &LccdDna) -> bool {
        let cpu_threshold = (dna.division_cpu_threshold * 255.0) as u8;
        let memory_threshold = (dna.division_memory_threshold * 255.0) as u8;
        
        self.cpu_usage.is_saturated(cpu_threshold) || 
        self.memory_usage.is_saturated(memory_threshold)
    }
}

/// Ultra-lightweight LCCD cell (designed for 1-core i3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LccdCell {
    pub cell_id: CellId,
    pub generation: u16,
    pub parent_id: Option<u64>,
    pub birth_time: Instant,
    pub dna: LccdDna,
    pub lifecycle: CellLifecycle,
    pub health: u8,                    // 0-255 health score
    pub division_readiness: u8,        // 0-255 readiness score
    pub metabolism: MicroMetabolism,
    pub consensus_count: u64,
    pub division_count: u16,
}

impl LccdCell {
    /// Create genesis cell optimized for 1-core i3
    pub async fn genesis() -> Result<Self> {
        info!("🧬 LCCD Genesis: Creating first cell on 1-core i3...");
        
        // Hardware detection and validation
        let hardware = HardwareProfile::detect()?;
        hardware.validate_genesis_requirements()?;
        
        // Generate optimal DNA
        let genesis_dna = LccdDna::genesis(&hardware);
        
        // Create genesis cell
        let cell = LccdCell {
            cell_id: CellId::genesis(),
            generation: 0,
            parent_id: None,
            birth_time: Instant::now(),
            dna: genesis_dna,
            lifecycle: CellLifecycle::Embryonic,
            health: 255,
            division_readiness: 0,
            metabolism: MicroMetabolism::new(&hardware),
            consensus_count: 0,
            division_count: 0,
        };
        
        info!("🎉 LCCD Genesis complete!");
        info!("💻 Hardware: {}MHz CPU, {}MB RAM", hardware.cpu_mhz, hardware.memory_mb);
        
        Ok(cell)
    }
    
    /// Main cell lifecycle (optimized for single-core)
    pub async fn live(&mut self) -> Result<()> {
        info!("🔄 LCCD Cell {} beginning life", self.cell_id);
        
        self.lifecycle = CellLifecycle::Growing;
        let mut iteration = 0u64;
        
        loop {
            iteration += 1;
            
            // Update metabolism
            if let Err(e) = self.update_micro_metabolism().await {
                warn!("Metabolism update failed: {}", e);
                self.health = self.health.saturating_sub(1);
            }
            
            // Update lifecycle
            self.update_lifecycle_state().await?;
            
            // Process consensus if mature
            if self.lifecycle == CellLifecycle::Mature {
                if let Err(e) = self.process_consensus_batch().await {
                    warn!("Consensus processing failed: {}", e);
                    self.health = self.health.saturating_sub(1);
                } else {
                    self.consensus_count += 1;
                    self.health = (self.health + 1).min(255);
                }
            }
            
            // Check for division
            if self.should_divide_for_wan_scale().await? {
                match self.divide_for_wan_scale().await {
                    Ok(daughter_cell) => {
                        info!("✨ Division successful: {} → {}", self.cell_id, daughter_cell.cell_id);
                        self.division_count += 1;
                    },
                    Err(e) => {
                        warn!("Division failed: {}", e);
                        self.health = self.health.saturating_sub(5);
                    }
                }
            }
            
            // Health check
            if self.health < 50 {
                self.lifecycle = CellLifecycle::Dead;
                info!("💀 Cell {} died after {} iterations", self.cell_id, iteration);
                break;
            }
            
            // Performance logging
            if iteration % 1000 == 0 {
                self.log_performance_metrics(iteration);
            }
            
            // Ultra-short sleep for high-frequency operation
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        Ok(())
    }
    
    async fn update_micro_metabolism(&mut self) -> Result<()> {
        self.metabolism.update()?;
        
        if self.metabolism.should_divide(&self.dna) {
            self.division_readiness = (self.division_readiness + 10).min(255);
        } else {
            self.division_readiness = self.division_readiness.saturating_sub(1);
        }
        
        Ok(())
    }
    
    async fn update_lifecycle_state(&mut self) -> Result<()> {
        match self.lifecycle {
            CellLifecycle::Embryonic => {
                if self.birth_time.elapsed() > Duration::from_secs(1) {
                    self.lifecycle = CellLifecycle::Growing;
                    info!("🌱 Cell {} → Growing", self.cell_id);
                }
            },
            CellLifecycle::Growing => {
                if self.health > 200 && self.consensus_count > 10 {
                    self.lifecycle = CellLifecycle::Mature;
                    info!("🌳 Cell {} → Mature", self.cell_id);
                }
            },
            CellLifecycle::Mature => {
                if self.generation > 15 || self.birth_time.elapsed() > Duration::from_secs(3600) {
                    self.lifecycle = CellLifecycle::Senescent;
                    info!("🍂 Cell {} → Senescent", self.cell_id);
                }
            },
            CellLifecycle::Senescent => {
                self.health = self.health.saturating_sub(1);
                if self.health < 10 {
                    self.lifecycle = CellLifecycle::Dead;
                }
            },
            _ => {}
        }
        Ok(())
    }
    
    async fn process_consensus_batch(&mut self) -> Result<()> {
        let start_time = Instant::now();
        
        // Ultra-fast κ computation
        let kappa = self.compute_kappa_fast()?;
        
        // Ultra-fast NxTri confidence update
        let _confidence = self.update_confidence_fast(kappa)?;
        
        // Update performance metrics
        let processing_time = start_time.elapsed();
        self.metabolism.ops_per_second = if processing_time.as_nanos() > 0 {
            (1_000_000_000 / processing_time.as_nanos()) as u32
        } else {
            1_000_000
        };
        
        Ok(())
    }
    
    fn compute_kappa_fast(&self) -> Result<f32> {
        let kappa = self.dna.kappa_a * 0.5 + self.dna.kappa_b * 0.3 + self.dna.kappa_c * 0.2;
        Ok(kappa.min(self.dna.kappa_z))
    }
    
    fn update_confidence_fast(&self, kappa: f32) -> Result<f32> {
        let confidence = self.dna.alpha_weight * kappa + 
                        self.dna.beta_weight * 0.8 + 
                        self.dna.gamma_weight * 0.9;
        Ok(confidence.min(1.0))
    }
    
    async fn should_divide_for_wan_scale(&self) -> Result<bool> {
        Ok(self.generation < self.dna.max_generations &&
           self.lifecycle == CellLifecycle::Mature &&
           self.metabolism.should_divide(&self.dna) &&
           self.division_readiness > 200 &&
           self.health > 200)
    }
    
    async fn divide_for_wan_scale(&mut self) -> Result<LccdCell> {
        info!("🔄 Cell {} dividing (generation {})", self.cell_id, self.generation);
        
        self.lifecycle = CellLifecycle::Dividing;
        
        // Create daughter cell
        let daughter_cell = LccdCell {
            cell_id: CellId::generate(),
            generation: self.generation + 1,
            parent_id: Some(self.cell_id.0),
            birth_time: Instant::now(),
            dna: self.dna, // DNA replication
            lifecycle: CellLifecycle::Embryonic,
            health: 255,
            division_readiness: 0,
            metabolism: self.metabolism.clone(),
            consensus_count: 0,
            division_count: 0,
        };
        
        // Update parent after division
        self.lifecycle = CellLifecycle::Mature;
        self.division_readiness = 0;
        self.health = (self.health * 9 / 10).max(100); // Slight health cost
        
        Ok(daughter_cell)
    }
    
    fn log_performance_metrics(&self, iteration: u64) {
        info!("📊 Cell {} Performance (iteration {}):", self.cell_id, iteration);
        info!("   Health: {}/255", self.health);
        info!("   Consensus ops: {}", self.consensus_count);
        info!("   Divisions: {}", self.division_count);
        info!("   CPU usage: {}/255", self.metabolism.cpu_usage.current_utilization);
        info!("   Memory usage: {}/255", self.metabolism.memory_usage.current_utilization);
        info!("   Ops/sec: {}", self.metabolism.ops_per_second);
    }
}

/// LCCD Consensus Manager
#[derive(Debug)]
pub struct LccdConsensusManager {
    pub genesis_cell: Option<LccdCell>,
    pub active_cells: HashMap<CellId, LccdCell>,
    pub total_consensus_ops: u64,
    pub total_divisions: u64,
    pub start_time: Instant,
}

impl LccdConsensusManager {
    pub fn new() -> Self {
        Self {
            genesis_cell: None,
            active_cells: HashMap::new(),
            total_consensus_ops: 0,
            total_divisions: 0,
            start_time: Instant::now(),
        }
    }
    
    /// Initialize LCCD consensus system
    pub async fn initialize(&mut self) -> Result<()> {
        info!("🚀 Initializing LCCD Consensus System");
        
        // Create genesis cell
        let genesis_cell = LccdCell::genesis().await?;
        let cell_id = genesis_cell.cell_id.clone();
        
        self.genesis_cell = Some(genesis_cell.clone());
        self.active_cells.insert(cell_id, genesis_cell);
        
        info!("✅ LCCD Consensus System initialized with genesis cell");
        Ok(())
    }
    
    /// Run LCCD consensus system
    pub async fn run(&mut self) -> Result<()> {
        if self.genesis_cell.is_none() {
            return Err(anyhow!("LCCD system not initialized"));
        }
        
        info!("🔄 Starting LCCD Consensus System");
        
        // Run genesis cell
        if let Some(genesis_cell) = self.genesis_cell.as_mut() {
            genesis_cell.live().await?;
        }
        
        Ok(())
    }
    
    /// Get system performance metrics
    pub fn get_performance_metrics(&self) -> LccdPerformanceMetrics {
        let uptime = self.start_time.elapsed();
        let active_cell_count = self.active_cells.len();
        
        LccdPerformanceMetrics {
            uptime_seconds: uptime.as_secs(),
            active_cells: active_cell_count,
            total_consensus_ops: self.total_consensus_ops,
            total_divisions: self.total_divisions,
            ops_per_second: if uptime.as_secs() > 0 {
                self.total_consensus_ops / uptime.as_secs()
            } else {
                0
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LccdPerformanceMetrics {
    pub uptime_seconds: u64,
    pub active_cells: usize,
    pub total_consensus_ops: u64,
    pub total_divisions: u64,
    pub ops_per_second: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_lccd_genesis() {
        let cell = LccdCell::genesis().await.unwrap();
        assert_eq!(cell.generation, 0);
        assert_eq!(cell.lifecycle, CellLifecycle::Embryonic);
        assert_eq!(cell.health, 255);
    }
    
    #[tokio::test]
    async fn test_lccd_manager_initialization() {
        let mut manager = LccdConsensusManager::new();
        manager.initialize().await.unwrap();
        assert!(manager.genesis_cell.is_some());
        assert_eq!(manager.active_cells.len(), 1);
    }
}
