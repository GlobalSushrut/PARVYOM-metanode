// Basic integration tests that don't crash the system
// These tests focus on core functionality without complex struct creation

use bpi_core::pravyom_integration::*;
use bpi_core::pravyom_integration::pipeline_coordinator::PipelineCoordinator;

#[test]
fn test_pravyom_config_creation() {
    let config = PravyomConfig::default();
    // Just verify the config was created successfully
    assert!(!config.bpi_endpoint.is_empty());
}

#[test] 
fn test_action_record_adapter_creation() {
    let config = PravyomConfig::default();
    let result = ActionRecordAdapter::new(&config);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_coordinator_creation() {
    let config = PravyomConfig::default();
    let result = PipelineCoordinator::new(&config);
    assert!(result.is_ok());
}

#[test]
fn test_segment_threshold_manager_creation() {
    let config = PravyomConfig::default();
    let result = SegmentThresholdManager::new(&config);
    assert!(result.is_ok());
}

#[test]
fn test_summary_ticket_generator_creation() {
    let config = PravyomConfig::default();
    let result = SummaryTicketGenerator::new(&config);
    assert!(result.is_ok());
}
