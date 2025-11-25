// BPI Advanced Downloader - Core Module
// Manages the sophisticated BPI Neural Web Infrastructure via Immutable OS

pub mod system_detector;
pub mod immutable_os_manager;
pub mod download_manager;
pub mod config_generator;
pub mod service_orchestrator;
pub mod health_monitor;
pub mod update_manager;

pub use system_detector::SystemDetector;
pub use immutable_os_manager::{ImmutableOsManager, ImmutableOsConfig, BpiSystemStatus, ComponentStatus};
pub use download_manager::DownloadManager;
pub use config_generator::ConfigGenerator;
pub use service_orchestrator::ServiceOrchestrator;
pub use health_monitor::HealthMonitor;
pub use update_manager::UpdateManager;
