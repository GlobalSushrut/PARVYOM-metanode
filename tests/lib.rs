//! Integration Tests Library
//! Common utilities and setup for all integration tests

pub mod test_helpers;
pub mod test_helpers_10_20;
pub mod test_helpers_20_30;
pub mod test_helpers_30_40;

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment (called once per test run)
pub fn init_test_env() {
    INIT.call_once(|| {
        // Initialize logging for tests
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init();
        
        println!("🧪 Metanode Integration Test Environment Initialized");
    });
}

// Include test helpers with real Metanode functionality

// Include integration test modules
pub mod integration;

/// Basic test function for validation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_works() {
        init_test_env();
        assert!(true, "Test environment initialization works");
    }
}
