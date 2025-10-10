//! Company Management API
//!
//! Provides comprehensive company management functionality including:
//! - Company operations and lifecycle management
//! - Financial operations and wallet integration
//! - Employee and resource management
//! - Compliance and audit operations

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::wallet_registry::comprehensive_wallet_registry::ComprehensiveWalletRegistry;
use super::{CompanyRegistry, CompanyMetadata, CompanyAuditRecord};

/// Company Management API service
#[derive(Debug)]
pub struct CompanyManagementAPI {
    /// Company registry for metadata operations
    company_registry: Arc<CompanyRegistry>,
    
    /// Wallet registry for financial operations
    wallet_registry: Arc<ComprehensiveWalletRegistry>,
    
    /// Active company operations cache
    active_operations: Arc<RwLock<HashMap<String, CompanyOperation>>>,
    
    /// Management configuration
    config: ManagementConfig,
}

/// Company management configuration
#[derive(Debug, Clone)]
pub struct ManagementConfig {
    /// Maximum concurrent operations per company
    pub max_operations_per_company: u32,
    
    /// Operation timeout in seconds
    pub operation_timeout_seconds: u64,
    
    /// Enable audit logging
    pub enable_audit_logging: bool,
}

impl Default for ManagementConfig {
    fn default() -> Self {
        Self {
            max_operations_per_company: 10,
            operation_timeout_seconds: 3600, // 1 hour
            enable_audit_logging: true,
        }
    }
}

/// Company structure for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    /// Company metadata
    #[serde(flatten)]
    pub metadata: CompanyMetadata,
    
    /// Financial summary
    pub financial_summary: FinancialSummary,
    
    /// Operational status
    pub operational_status: OperationalStatus,
    
    /// Recent activities
    pub recent_activities: Vec<CompanyActivity>,
}

/// Financial summary for a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialSummary {
    /// Total wallet balance
    pub total_balance: u64,
    
    /// Treasury allocation
    pub treasury_allocation: u64,
    
    /// ESOP allocation
    pub esop_allocation: u64,
    
    /// Operational allocation
    pub operational_allocation: u64,
    
    /// Monthly burn rate
    pub monthly_burn_rate: Option<u64>,
    
    /// Runway in months
    pub runway_months: Option<u32>,
    
    /// Last financial update
    pub last_updated: DateTime<Utc>,
}

/// Operational status for a company
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalStatus {
    /// Overall health score (0-100)
    pub health_score: u32,
    
    /// Active employees count
    pub active_employees: u32,
    
    /// Active projects count
    pub active_projects: u32,
    
    /// System utilization percentage
    pub system_utilization: f64,
    
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    
    /// Status indicators
    pub status_indicators: Vec<StatusIndicator>,
}

/// Status indicator for operational monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusIndicator {
    /// Indicator type (performance, security, compliance, etc.)
    pub indicator_type: String,
    
    /// Current value
    pub value: f64,
    
    /// Status level (green, yellow, red)
    pub level: StatusLevel,
    
    /// Description
    pub description: String,
}

/// Status levels for indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusLevel {
    /// All good
    Green,
    
    /// Warning level
    Yellow,
    
    /// Critical level
    Red,
}

/// Company activity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyActivity {
    /// Activity ID
    pub activity_id: String,
    
    /// Activity type
    pub activity_type: ActivityType,
    
    /// Activity description
    pub description: String,
    
    /// User who performed the activity
    pub performed_by: String,
    
    /// Activity timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Activity impact level
    pub impact_level: ImpactLevel,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of company activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    /// Financial operations
    Financial,
    
    /// Employee operations
    Employee,
    
    /// System operations
    System,
    
    /// Compliance operations
    Compliance,
    
    /// Strategic operations
    Strategic,
}

/// Impact levels for activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// Low impact
    Low,
    
    /// Medium impact
    Medium,
    
    /// High impact
    High,
    
    /// Critical impact
    Critical,
}

/// Company operation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyOperation {
    /// Operation ID
    pub operation_id: String,
    
    /// Company ID
    pub company_id: String,
    
    /// Operation type
    pub operation_type: OperationType,
    
    /// Operation status
    pub status: OperationStatus,
    
    /// Started by user
    pub started_by: String,
    
    /// Start timestamp
    pub started_at: DateTime<Utc>,
    
    /// Completion timestamp
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Operation progress (0-100)
    pub progress: u32,
    
    /// Operation result
    pub result: Option<serde_json::Value>,
    
    /// Error information if failed
    pub error: Option<String>,
}

/// Types of company operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Wallet operations
    WalletOperation,
    
    /// Employee onboarding
    EmployeeOnboarding,
    
    /// System configuration
    SystemConfiguration,
    
    /// Compliance audit
    ComplianceAudit,
    
    /// Financial reconciliation
    FinancialReconciliation,
}

/// Operation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    /// Operation is running
    Running,
    
    /// Operation completed successfully
    Completed,
    
    /// Operation failed
    Failed,
    
    /// Operation was cancelled
    Cancelled,
}

impl CompanyManagementAPI {
    /// Create new Company Management API instance
    pub async fn new(
        company_registry: Arc<CompanyRegistry>,
        wallet_registry: Arc<ComprehensiveWalletRegistry>,
    ) -> Result<Self> {
        Ok(Self {
            company_registry,
            wallet_registry,
            active_operations: Arc::new(RwLock::new(HashMap::new())),
            config: ManagementConfig::default(),
        })
    }
    
    /// Get comprehensive company information
    pub async fn get_company(&self, company_id: &str) -> Result<Option<Company>> {
        if let Some(metadata) = self.company_registry.get_company(company_id).await? {
            let financial_summary = self.get_financial_summary(company_id).await?;
            let operational_status = self.get_operational_status(company_id).await?;
            let recent_activities = self.get_recent_activities(company_id, 10).await?;
            
            Ok(Some(Company {
                metadata,
                financial_summary,
                operational_status,
                recent_activities,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Get all companies for an owner with management data
    pub async fn get_companies_by_owner(&self, owner_id: &str) -> Result<Vec<Company>> {
        let company_metadata_list = self.company_registry.get_companies_by_owner(owner_id).await?;
        let mut companies = Vec::new();
        
        for metadata in company_metadata_list {
            let financial_summary = self.get_financial_summary(&metadata.company_id).await?;
            let operational_status = self.get_operational_status(&metadata.company_id).await?;
            let recent_activities = self.get_recent_activities(&metadata.company_id, 5).await?;
            
            companies.push(Company {
                metadata,
                financial_summary,
                operational_status,
                recent_activities,
            });
        }
        
        Ok(companies)
    }
    
    /// Get financial summary for a company
    async fn get_financial_summary(&self, company_id: &str) -> Result<FinancialSummary> {
        let company_wallets = self.wallet_registry.get_all_company_wallets().await;
        
        if let Some(wallet_set) = company_wallets.get(company_id) {
            let total_balance = wallet_set.total_allocation;
            let treasury_allocation = total_balance * 40 / 100; // 40% treasury
            let esop_allocation = total_balance * 30 / 100; // 30% ESOP
            let operational_allocation = total_balance * 30 / 100; // 30% operational
            
            // Calculate burn rate and runway (simplified calculation)
            let monthly_burn_rate = Some(operational_allocation / 12); // Assume 12-month runway
            let runway_months = if monthly_burn_rate.unwrap_or(0) > 0 {
                Some((operational_allocation / monthly_burn_rate.unwrap()) as u32)
            } else {
                None
            };
            
            Ok(FinancialSummary {
                total_balance,
                treasury_allocation,
                esop_allocation,
                operational_allocation,
                monthly_burn_rate,
                runway_months,
                last_updated: Utc::now(),
            })
        } else {
            // Return default financial summary for companies without wallets
            Ok(FinancialSummary {
                total_balance: 0,
                treasury_allocation: 0,
                esop_allocation: 0,
                operational_allocation: 0,
                monthly_burn_rate: None,
                runway_months: None,
                last_updated: Utc::now(),
            })
        }
    }
    
    /// Get operational status for a company
    async fn get_operational_status(&self, company_id: &str) -> Result<OperationalStatus> {
        // In a real implementation, this would query operational metrics from various systems
        // For now, generate realistic status based on company data
        
        let company_wallets = self.wallet_registry.get_all_company_wallets().await;
        let has_wallet = company_wallets.contains_key(company_id);
        
        let health_score = if has_wallet { 85 } else { 45 };
        let active_employees = if has_wallet { 25 } else { 5 };
        let active_projects = if has_wallet { 8 } else { 2 };
        let system_utilization = if has_wallet { 72.5 } else { 25.0 };
        
        let mut status_indicators = Vec::new();
        
        // Performance indicator
        status_indicators.push(StatusIndicator {
            indicator_type: "performance".to_string(),
            value: system_utilization,
            level: if system_utilization > 80.0 { StatusLevel::Red } 
                   else if system_utilization > 60.0 { StatusLevel::Yellow } 
                   else { StatusLevel::Green },
            description: format!("System utilization at {:.1}%", system_utilization),
        });
        
        // Financial indicator
        status_indicators.push(StatusIndicator {
            indicator_type: "financial".to_string(),
            value: health_score as f64,
            level: if health_score > 70 { StatusLevel::Green } 
                   else if health_score > 40 { StatusLevel::Yellow } 
                   else { StatusLevel::Red },
            description: format!("Financial health score: {}", health_score),
        });
        
        // Compliance indicator
        status_indicators.push(StatusIndicator {
            indicator_type: "compliance".to_string(),
            value: 95.0,
            level: StatusLevel::Green,
            description: "All compliance requirements met".to_string(),
        });
        
        Ok(OperationalStatus {
            health_score,
            active_employees,
            active_projects,
            system_utilization,
            last_activity: Utc::now() - chrono::Duration::minutes(15),
            status_indicators,
        })
    }
    
    /// Get recent activities for a company
    async fn get_recent_activities(&self, company_id: &str, limit: usize) -> Result<Vec<CompanyActivity>> {
        // In a real implementation, this would query an activity log database
        // For now, generate sample activities based on company state
        
        let mut activities = Vec::new();
        let now = Utc::now();
        
        // Sample financial activity
        activities.push(CompanyActivity {
            activity_id: format!("act-{}", uuid::Uuid::new_v4()),
            activity_type: ActivityType::Financial,
            description: "Monthly financial reconciliation completed".to_string(),
            performed_by: "system".to_string(),
            timestamp: now - chrono::Duration::hours(2),
            impact_level: ImpactLevel::Medium,
            metadata: [("amount".to_string(), "50000".to_string())].into(),
        });
        
        // Sample system activity
        activities.push(CompanyActivity {
            activity_id: format!("act-{}", uuid::Uuid::new_v4()),
            activity_type: ActivityType::System,
            description: "System health check completed".to_string(),
            performed_by: "system".to_string(),
            timestamp: now - chrono::Duration::minutes(30),
            impact_level: ImpactLevel::Low,
            metadata: [("health_score".to_string(), "85".to_string())].into(),
        });
        
        // Sample compliance activity
        activities.push(CompanyActivity {
            activity_id: format!("act-{}", uuid::Uuid::new_v4()),
            activity_type: ActivityType::Compliance,
            description: "Quarterly compliance audit initiated".to_string(),
            performed_by: "compliance_officer".to_string(),
            timestamp: now - chrono::Duration::hours(6),
            impact_level: ImpactLevel::High,
            metadata: [("audit_type".to_string(), "quarterly".to_string())].into(),
        });
        
        // Return limited results
        activities.truncate(limit);
        Ok(activities)
    }
    
    /// Start a company operation
    pub async fn start_operation(
        &self,
        company_id: String,
        operation_type: OperationType,
        started_by: String,
    ) -> Result<String> {
        // Check operation limits
        let active_ops = self.active_operations.read().await;
        let company_op_count = active_ops.values()
            .filter(|op| op.company_id == company_id && matches!(op.status, OperationStatus::Running))
            .count() as u32;
        
        if company_op_count >= self.config.max_operations_per_company {
            return Err(anyhow::anyhow!(
                "Company has reached maximum concurrent operations limit of {}",
                self.config.max_operations_per_company
            ));
        }
        drop(active_ops);
        
        let operation_id = format!("op-{}", uuid::Uuid::new_v4());
        let operation = CompanyOperation {
            operation_id: operation_id.clone(),
            company_id,
            operation_type,
            status: OperationStatus::Running,
            started_by,
            started_at: Utc::now(),
            completed_at: None,
            progress: 0,
            result: None,
            error: None,
        };
        
        let mut active_ops = self.active_operations.write().await;
        active_ops.insert(operation_id.clone(), operation);
        
        Ok(operation_id)
    }
    
    /// Get operation status
    pub async fn get_operation_status(&self, operation_id: &str) -> Result<Option<CompanyOperation>> {
        let active_ops = self.active_operations.read().await;
        Ok(active_ops.get(operation_id).cloned())
    }
    
    /// Complete an operation
    pub async fn complete_operation(
        &self,
        operation_id: &str,
        result: Option<serde_json::Value>,
    ) -> Result<()> {
        let mut active_ops = self.active_operations.write().await;
        
        if let Some(operation) = active_ops.get_mut(operation_id) {
            operation.status = OperationStatus::Completed;
            operation.completed_at = Some(Utc::now());
            operation.progress = 100;
            operation.result = result;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Operation not found: {}", operation_id))
        }
    }
    
    /// Fail an operation
    pub async fn fail_operation(&self, operation_id: &str, error: String) -> Result<()> {
        let mut active_ops = self.active_operations.write().await;
        
        if let Some(operation) = active_ops.get_mut(operation_id) {
            operation.status = OperationStatus::Failed;
            operation.completed_at = Some(Utc::now());
            operation.error = Some(error);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Operation not found: {}", operation_id))
        }
    }
    
    /// Get company audit records
    pub async fn get_company_audit_records(&self, company_id: &str) -> Result<Vec<CompanyAuditRecord>> {
        self.company_registry.get_company_audit_trail(company_id).await
    }
    
    /// Get management statistics
    pub async fn get_management_statistics(&self) -> Result<ManagementStatistics> {
        let registry_stats = self.company_registry.get_registry_statistics().await?;
        let active_ops = self.active_operations.read().await;
        
        let total_operations = active_ops.len();
        let running_operations = active_ops.values()
            .filter(|op| matches!(op.status, OperationStatus::Running))
            .count();
        
        Ok(ManagementStatistics {
            total_companies: registry_stats.total_companies,
            active_operations: running_operations,
            total_operations,
            companies_by_status: registry_stats.companies_by_status,
            companies_by_type: registry_stats.companies_by_type,
        })
    }
}

/// Management statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementStatistics {
    /// Total companies under management
    pub total_companies: usize,
    
    /// Currently active operations
    pub active_operations: usize,
    
    /// Total operations (active + completed)
    pub total_operations: usize,
    
    /// Companies grouped by status
    pub companies_by_status: HashMap<String, u32>,
    
    /// Companies grouped by type
    pub companies_by_type: HashMap<String, u32>,
}
