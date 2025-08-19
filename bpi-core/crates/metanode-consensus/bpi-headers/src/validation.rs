//! Header validation utilities and chain validation logic

use chrono::{Utc, Duration};

use crate::Header;

/// Header validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum allowed timestamp drift from current time
    pub max_timestamp_drift: Duration,
    /// Minimum time between blocks
    pub min_block_time: Duration,
    /// Maximum time between blocks
    pub max_block_time: Duration,
    /// Whether to enforce strict timestamp validation
    pub strict_timestamps: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_timestamp_drift: Duration::minutes(5),
            min_block_time: Duration::seconds(1),
            max_block_time: Duration::minutes(10),
            strict_timestamps: true,
        }
    }
}

/// Header chain validator
pub struct HeaderValidator {
    config: ValidationConfig,
}

/// Validation result with detailed error information
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
}

impl HeaderValidator {
    /// Create a new header validator with default configuration
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }
    
    /// Create a new header validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }
    
    /// Validate a single header
    pub fn validate_header(&self, header: &Header) -> ValidationResult {
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        // Basic header validation
        if let Err(e) = header.validate() {
            result.errors.push(format!("Header validation failed: {e}"));
            result.is_valid = false;
        }
        
        // Timestamp validation
        if self.config.strict_timestamps {
            self.validate_timestamp(header, &mut result);
        }
        
        result
    }
    
    /// Validate header chain continuity
    pub fn validate_chain(&self, headers: &[Header]) -> ValidationResult {
        let mut result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
        if headers.is_empty() {
            result.errors.push("Empty header chain".to_string());
            result.is_valid = false;
            return result;
        }
        
        // Validate first header (should be genesis or valid standalone)
        let first_result = self.validate_header(&headers[0]);
        result.merge(first_result);
        
        // Validate chain continuity
        for i in 1..headers.len() {
            let prev_header = &headers[i - 1];
            let current_header = &headers[i];
            
            // Validate current header
            let header_result = self.validate_header(current_header);
            result.merge(header_result);
            
            // Validate chain continuity
            if let Err(e) = current_header.validate_chain_continuity(prev_header) {
                result.errors.push(format!("Chain continuity failed at height {}: {}", current_header.height, e));
                result.is_valid = false;
            }
            
            // Validate block timing
            self.validate_block_timing(prev_header, current_header, &mut result);
        }
        
        result
    }
    
    /// Validate header against parent
    pub fn validate_with_parent(&self, header: &Header, parent: &Header) -> ValidationResult {
        let mut result = self.validate_header(header);
        
        // Validate chain continuity
        if let Err(e) = header.validate_chain_continuity(parent) {
            result.errors.push(format!("Chain continuity failed: {e}"));
            result.is_valid = false;
        }
        
        // Validate block timing
        self.validate_block_timing(parent, header, &mut result);
        
        result
    }
    
    fn validate_timestamp(&self, header: &Header, result: &mut ValidationResult) {
        let now = Utc::now();
        let timestamp_diff = header.timestamp.signed_duration_since(now);
        
        // Check if timestamp is too far in the future
        if timestamp_diff > self.config.max_timestamp_drift {
            result.errors.push(format!(
                "Header timestamp too far in future: {} > {} seconds",
                timestamp_diff.num_seconds(),
                self.config.max_timestamp_drift.num_seconds()
            ));
            result.is_valid = false;
        }
        
        // Check if timestamp is too far in the past (warning only)
        if timestamp_diff < -self.config.max_timestamp_drift {
            result.warnings.push(format!(
                "Header timestamp significantly in past: {} seconds ago",
                -timestamp_diff.num_seconds()
            ));
        }
    }
    
    fn validate_block_timing(&self, prev_header: &Header, current_header: &Header, result: &mut ValidationResult) {
        let time_diff = current_header.timestamp.signed_duration_since(prev_header.timestamp);
        
        // Check minimum block time
        if time_diff < self.config.min_block_time {
            result.errors.push(format!(
                "Block time too short: {} < {} seconds",
                time_diff.num_seconds(),
                self.config.min_block_time.num_seconds()
            ));
            result.is_valid = false;
        }
        
        // Check maximum block time (warning)
        if time_diff > self.config.max_block_time {
            result.warnings.push(format!(
                "Block time very long: {} > {} seconds",
                time_diff.num_seconds(),
                self.config.max_block_time.num_seconds()
            ));
        }
    }
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    /// Create a failed validation result with error
    pub fn error(message: String) -> Self {
        Self {
            is_valid: false,
            errors: vec![message],
            warnings: Vec::new(),
        }
    }
    
    /// Merge another validation result into this one
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
    
    /// Check if validation has any issues (errors or warnings)
    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
    
    /// Get all issues as formatted string
    pub fn format_issues(&self) -> String {
        let mut issues = Vec::new();
        
        for error in &self.errors {
            issues.push(format!("ERROR: {error}"));
        }
        
        for warning in &self.warnings {
            issues.push(format!("WARNING: {warning}"));
        }
        
        issues.join("\n")
    }
}

impl Default for HeaderValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use crate::{GenesisConfig, ConsensusMode, HeaderConfig};
    use chrono::TimeZone;
    
    fn create_test_genesis() -> Header {
        let config = GenesisConfig {
            timestamp: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            validator_set_hash: [7u8; 32],
            chain_id: 1,
        };
        Header::genesis(&config)
    }
    
    fn create_test_header(height: u64, prev_hash: [u8; 32], timestamp: DateTime<Utc>) -> Header {
        let config = HeaderConfig {
            version: 1,
            height,
            prev_hash,
            poh_root: [2u8; 32],
            receipts_root: [3u8; 32],
            da_root: [4u8; 32],
            xcmp_root: [5u8; 32],
            validator_set_hash: [6u8; 32],
            mode: ConsensusMode::Ibft,
            round: 0,
        };
        let mut header = Header::new(config);
        header.timestamp = timestamp;
        header
    }
    
    #[test]
    fn test_validator_creation() {
        let validator = HeaderValidator::new();
        assert!(validator.config.strict_timestamps);
        
        let custom_config = ValidationConfig {
            max_timestamp_drift: Duration::minutes(1),
            min_block_time: Duration::seconds(5),
            max_block_time: Duration::minutes(1),
            strict_timestamps: false,
        };
        let custom_validator = HeaderValidator::with_config(custom_config);
        assert!(!custom_validator.config.strict_timestamps);
    }
    
    #[test]
    fn test_single_header_validation() {
        let validator = HeaderValidator::new();
        let genesis = create_test_genesis();
        
        let result = validator.validate_header(&genesis);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }
    
    #[test]
    fn test_chain_validation() {
        let validator = HeaderValidator::new();
        let genesis = create_test_genesis();
        let genesis_hash = genesis.hash().unwrap().0;
        
        let next_header = create_test_header(
            1,
            genesis_hash,
            Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap(),
        );
        
        let chain = vec![genesis, next_header];
        let result = validator.validate_chain(&chain);
        
        assert!(result.is_valid, "Chain validation failed: {}", result.format_issues());
    }
    
    #[test]
    fn test_chain_validation_failure() {
        let validator = HeaderValidator::new();
        let genesis = create_test_genesis();
        
        // Create header with wrong prev_hash
        let bad_header = create_test_header(
            1,
            [1u8; 32], // Wrong prev_hash
            Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap(),
        );
        
        let chain = vec![genesis, bad_header];
        let result = validator.validate_chain(&chain);
        
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }
    
    #[test]
    fn test_validation_result_merge() {
        let mut result1 = ValidationResult::success();
        let result2 = ValidationResult::error("Test error".to_string());
        
        assert!(result1.is_valid);
        result1.merge(result2);
        assert!(!result1.is_valid);
        assert_eq!(result1.errors.len(), 1);
    }
    
    #[test]
    fn test_validation_result_formatting() {
        let mut result = ValidationResult::success();
        result.errors.push("Error 1".to_string());
        result.warnings.push("Warning 1".to_string());
        result.is_valid = false;
        
        let formatted = result.format_issues();
        assert!(formatted.contains("ERROR: Error 1"));
        assert!(formatted.contains("WARNING: Warning 1"));
    }
}
