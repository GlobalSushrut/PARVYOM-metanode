//! # Unified Manager Module
//! 
//! Provides unified management of all 32 components (BPCI + BPI OS)
//! from a single Tauri-based interface

pub mod component_manager;

pub use component_manager::{
    UnifiedComponentManager,
    Component,
    ComponentCategory,
    ComponentStatus,
    ComponentMetrics,
    Status,
    HealthStatus,
};
