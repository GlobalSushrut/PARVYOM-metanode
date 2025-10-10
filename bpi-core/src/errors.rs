//! Structured Error System for BPI Infrastructure
//! 
//! Provides comprehensive error handling with helpful messages and recovery suggestions

use serde::{Serialize, Deserialize};
use std::fmt;
use thiserror::Error;

/// Main BPI error type with helpful context and suggestions
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum BpiError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        suggestion: String,
        config_file: Option<String>,
        field: Option<String>,
    },
    
    /// Network connectivity errors
    #[error("Network error: {message}")]
    Network {
        message: String,
        endpoint: String,
        suggestion: String,
        retry_possible: bool,
    },
    
    /// Service startup/runtime errors
    #[error("Service error: {message}")]
    Service {
        message: String,
        service_name: String,
        suggestion: String,
        port: Option<u16>,
    },
    
    /// Database operation errors
    #[error("Database error: {message}")]
    Database {
        message: String,
        operation: String,
        suggestion: String,
        collection: Option<String>,
    },
    
    /// VM Server errors
    #[error("VM Server error: {message}")]
    VmServer {
        message: String,
        vm_id: Option<String>,
        suggestion: String,
        recovery_action: String,
    },
    
    /// BPCI Bridge errors
    #[error("BPCI Bridge error: {message}")]
    BpciBridge {
        message: String,
        transaction_id: Option<String>,
        suggestion: String,
        retry_possible: bool,
    },
    
    /// Authentication/Authorization errors
    #[error("Authentication error: {message}")]
    Auth {
        message: String,
        suggestion: String,
        required_permission: Option<String>,
    },
    
    /// Deployment errors
    #[error("Deployment error: {message}")]
    Deployment {
        message: String,
        stage: String,
        suggestion: String,
        rollback_possible: bool,
    },
    
    /// Pilot-specific errors
    #[error("Pilot setup error: {message}")]
    Pilot {
        message: String,
        suggestion: String,
        documentation_link: String,
        quick_fix: Option<String>,
    },
    
    /// Generic errors with context
    #[error("System error: {message}")]
    System {
        message: String,
        suggestion: String,
        error_code: Option<String>,
    },
}

impl BpiError {
    /// Create a configuration error with helpful context
    pub fn config_error(message: &str, config_file: Option<&str>, field: Option<&str>) -> Self {
        let suggestion = match field {
            Some(f) => format!("Check the '{}' field in your configuration file. Ensure it follows the expected format.", f),
            None => "Validate your configuration file syntax and required fields.".to_string(),
        };
        
        Self::Config {
            message: message.to_string(),
            suggestion,
            config_file: config_file.map(|s| s.to_string()),
            field: field.map(|s| s.to_string()),
        }
    }
    
    /// Create a network error with retry information
    pub fn network_error(message: &str, endpoint: &str, retry_possible: bool) -> Self {
        let suggestion = if retry_possible {
            format!("Check if {} is accessible. Verify network connectivity and firewall settings. Retry in a few moments.", endpoint)
        } else {
            format!("Service at {} is not responding. Check if the service is running and the endpoint is correct.", endpoint)
        };
        
        Self::Network {
            message: message.to_string(),
            endpoint: endpoint.to_string(),
            suggestion,
            retry_possible,
        }
    }
    
    /// Create a service error with startup suggestions
    pub fn service_error(message: &str, service_name: &str, port: Option<u16>) -> Self {
        let suggestion = match port {
            Some(p) => format!("Check if {} is running on port {}. Try: 'ps aux | grep {}' or 'netstat -tlnp | grep {}'", service_name, p, service_name, p),
            None => format!("Check if {} service is running. Try restarting with: 'bpi-core {} restart'", service_name, service_name.to_lowercase()),
        };
        
        Self::Service {
            message: message.to_string(),
            service_name: service_name.to_string(),
            suggestion,
            port,
        }
    }
    
    /// Create a database error with operation context
    pub fn database_error(message: &str, operation: &str, collection: Option<&str>) -> Self {
        let suggestion = match collection {
            Some(c) => format!("Database operation '{}' failed on collection '{}'. Check database connectivity and collection permissions.", operation, c),
            None => format!("Database operation '{}' failed. Verify database is running and accessible.", operation),
        };
        
        Self::Database {
            message: message.to_string(),
            operation: operation.to_string(),
            suggestion,
            collection: collection.map(|s| s.to_string()),
        }
    }
    
    /// Create a VM Server error with recovery actions
    pub fn vm_server_error(message: &str, vm_id: Option<&str>) -> Self {
        let (suggestion, recovery_action) = match vm_id {
            Some(id) => (
                format!("VM {} encountered an error. Check VM logs and resource usage.", id),
                format!("Try restarting VM: 'bpi-core vm restart {}'", id)
            ),
            None => (
                "VM Server encountered an error. Check server status and logs.".to_string(),
                "Try restarting VM Server: 'bpi-core vm-server restart'".to_string()
            ),
        };
        
        Self::VmServer {
            message: message.to_string(),
            vm_id: vm_id.map(|s| s.to_string()),
            suggestion,
            recovery_action,
        }
    }
    
    /// Create a BPCI Bridge error with transaction context
    pub fn bpci_bridge_error(message: &str, transaction_id: Option<&str>, retry_possible: bool) -> Self {
        let suggestion = match transaction_id {
            Some(tx_id) => format!("Transaction {} failed. Check transaction status and blockchain connectivity.", tx_id),
            None => "BPCI Bridge operation failed. Verify bridge service is running and blockchain is accessible.".to_string(),
        };
        
        Self::BpciBridge {
            message: message.to_string(),
            transaction_id: transaction_id.map(|s| s.to_string()),
            suggestion,
            retry_possible,
        }
    }
    
    /// Create a pilot-specific error with documentation links
    pub fn pilot_error(message: &str, quick_fix: Option<&str>) -> Self {
        Self::Pilot {
            message: message.to_string(),
            suggestion: "This is a pilot setup issue. Check the getting started guide for common solutions.".to_string(),
            documentation_link: "https://docs.bpi.dev/pilot-setup".to_string(),
            quick_fix: quick_fix.map(|s| s.to_string()),
        }
    }
    
    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            BpiError::Config { .. } => "configuration",
            BpiError::Network { .. } => "network",
            BpiError::Service { .. } => "service",
            BpiError::Database { .. } => "database",
            BpiError::VmServer { .. } => "vm_server",
            BpiError::BpciBridge { .. } => "bpci_bridge",
            BpiError::Auth { .. } => "authentication",
            BpiError::Deployment { .. } => "deployment",
            BpiError::Pilot { .. } => "pilot",
            BpiError::System { .. } => "system",
        }
    }
    
    /// Get the severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            BpiError::Config { .. } => ErrorSeverity::High,
            BpiError::Network { retry_possible, .. } => {
                if *retry_possible { ErrorSeverity::Medium } else { ErrorSeverity::High }
            },
            BpiError::Service { .. } => ErrorSeverity::High,
            BpiError::Database { .. } => ErrorSeverity::High,
            BpiError::VmServer { .. } => ErrorSeverity::Medium,
            BpiError::BpciBridge { retry_possible, .. } => {
                if *retry_possible { ErrorSeverity::Medium } else { ErrorSeverity::High }
            },
            BpiError::Auth { .. } => ErrorSeverity::High,
            BpiError::Deployment { .. } => ErrorSeverity::Critical,
            BpiError::Pilot { .. } => ErrorSeverity::Medium,
            BpiError::System { .. } => ErrorSeverity::Medium,
        }
    }
    
    /// Get suggested recovery actions
    pub fn recovery_actions(&self) -> Vec<String> {
        match self {
            BpiError::Config { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Network { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Service { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Database { suggestion, .. } => vec![suggestion.clone()],
            BpiError::VmServer { suggestion, recovery_action, .. } => {
                vec![suggestion.clone(), recovery_action.clone()]
            },
            BpiError::BpciBridge { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Auth { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Deployment { suggestion, .. } => vec![suggestion.clone()],
            BpiError::Pilot { suggestion, quick_fix, .. } => {
                let mut actions = vec![suggestion.clone()];
                if let Some(fix) = quick_fix {
                    actions.push(fix.clone());
                }
                actions
            },
            BpiError::System { suggestion, .. } => vec![suggestion.clone()],
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "LOW"),
            ErrorSeverity::Medium => write!(f, "MEDIUM"),
            ErrorSeverity::High => write!(f, "HIGH"),
            ErrorSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Error context for detailed reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error: BpiError,
    pub timestamp: u64,
    pub component: String,
    pub trace_id: Option<String>,
    pub user_id: Option<String>,
    pub environment: String,
}

impl ErrorContext {
    pub fn new(error: BpiError, component: &str) -> Self {
        Self {
            error,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            component: component.to_string(),
            trace_id: None,
            user_id: None,
            environment: std::env::var("BPI_ENV").unwrap_or_else(|_| "unknown".to_string()),
        }
    }
    
    pub fn with_trace_id(mut self, trace_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self
    }
    
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }
}

/// Result type alias for BPI operations
pub type BpiResult<T> = Result<T, BpiError>;

/// Convert from anyhow::Error to BpiError
impl From<anyhow::Error> for BpiError {
    fn from(err: anyhow::Error) -> Self {
        BpiError::System {
            message: err.to_string(),
            suggestion: "Check logs for more details and verify system configuration.".to_string(),
            error_code: None,
        }
    }
}

/// Convert from std::io::Error to BpiError
impl From<std::io::Error> for BpiError {
    fn from(err: std::io::Error) -> Self {
        BpiError::System {
            message: format!("IO error: {}", err),
            suggestion: "Check file permissions and disk space.".to_string(),
            error_code: Some(err.kind().to_string()),
        }
    }
}

/// Convert from serde_json::Error to BpiError
impl From<serde_json::Error> for BpiError {
    fn from(err: serde_json::Error) -> Self {
        BpiError::Config {
            message: format!("JSON parsing error: {}", err),
            suggestion: "Check JSON syntax and structure.".to_string(),
            config_file: None,
            field: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_creation() {
        let error = BpiError::config_error("Invalid port", Some("config.toml"), Some("vm_port"));
        assert_eq!(error.category(), "configuration");
        assert_eq!(error.severity(), ErrorSeverity::High);
    }

    #[test]
    fn test_network_error_with_retry() {
        let error = BpiError::network_error("Connection timeout", "http://localhost:8080", true);
        assert_eq!(error.category(), "network");
        assert_eq!(error.severity(), ErrorSeverity::Medium);
    }

    #[test]
    fn test_error_context() {
        let error = BpiError::pilot_error("Setup failed", Some("Run ./deploy.sh"));
        let context = ErrorContext::new(error, "deployment")
            .with_trace_id("trace-123")
            .with_user_id("pilot-user");
        
        assert_eq!(context.component, "deployment");
        assert_eq!(context.trace_id, Some("trace-123".to_string()));
    }
}
