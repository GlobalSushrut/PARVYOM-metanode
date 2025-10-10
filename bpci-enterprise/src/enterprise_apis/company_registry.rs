//! Company Registry API
//!
//! Manages company registration, metadata, and lifecycle operations for the
//! BPCI Enterprise system. Integrates with wallet registry for financial
//! operations and provides comprehensive company management capabilities.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Company Registry service for managing enterprise companies
#[derive(Debug)]
pub struct CompanyRegistry {
    /// In-memory company storage (in production, this would be a database)
    companies: Arc<RwLock<HashMap<String, CompanyMetadata>>>,
    
    /// Company audit trail
    audit_trail: Arc<RwLock<Vec<CompanyAuditRecord>>>,
    
    /// Registry configuration
    config: RegistryConfig,
}

/// Company metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyMetadata {
    /// Unique company identifier
    pub company_id: String,
    
    /// Company name
    pub name: String,
    
    /// Company description
    pub description: String,
    
    /// Company type (startup, enterprise, government, etc.)
    pub company_type: CompanyType,
    
    /// Registration status
    pub status: CompanyStatus,
    
    /// Owner/founder information
    pub owner_id: String,
    
    /// Company contact information
    pub contact_info: ContactInfo,
    
    /// Financial information
    pub financial_info: FinancialInfo,
    
    /// Operational metrics
    pub operational_metrics: OperationalMetrics,
    
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Company types supported by the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanyType {
    /// Early-stage startup
    Startup,
    
    /// Small to medium business
    SMB,
    
    /// Large enterprise
    Enterprise,
    
    /// Government entity
    Government,
    
    /// Non-profit organization
    NonProfit,
    
    /// Educational institution
    Educational,
    
    /// Research organization
    Research,
}

/// Company registration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanyStatus {
    /// Pending registration approval
    Pending,
    
    /// Active and operational
    Active,
    
    /// Temporarily suspended
    Suspended,
    
    /// Permanently deactivated
    Deactivated,
    
    /// Under review
    UnderReview,
}

/// Company contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Primary email address
    pub email: String,
    
    /// Phone number
    pub phone: Option<String>,
    
    /// Physical address
    pub address: Option<Address>,
    
    /// Website URL
    pub website: Option<String>,
    
    /// Primary contact person
    pub primary_contact: String,
}

/// Physical address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Street address
    pub street: String,
    
    /// City
    pub city: String,
    
    /// State or province
    pub state: String,
    
    /// Postal code
    pub postal_code: String,
    
    /// Country
    pub country: String,
}

/// Company financial information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInfo {
    /// Annual revenue (if disclosed)
    pub annual_revenue: Option<u64>,
    
    /// Number of employees
    pub employee_count: u32,
    
    /// Funding stage
    pub funding_stage: FundingStage,
    
    /// Total funding raised
    pub total_funding: Option<u64>,
    
    /// Valuation (if available)
    pub valuation: Option<u64>,
}

/// Funding stages for companies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FundingStage {
    /// Pre-seed funding
    PreSeed,
    
    /// Seed funding
    Seed,
    
    /// Series A
    SeriesA,
    
    /// Series B
    SeriesB,
    
    /// Series C and beyond
    SeriesC,
    
    /// Public company
    Public,
    
    /// Bootstrapped/Self-funded
    Bootstrapped,
}

/// Operational metrics for companies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalMetrics {
    /// Monthly active users (if applicable)
    pub monthly_active_users: Option<u64>,
    
    /// Monthly recurring revenue (if applicable)
    pub monthly_recurring_revenue: Option<u64>,
    
    /// Customer count
    pub customer_count: Option<u32>,
    
    /// Growth rate percentage
    pub growth_rate: Option<f64>,
    
    /// Market presence score (0-100)
    pub market_presence_score: Option<u32>,
}

/// Company audit record for compliance and tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAuditRecord {
    /// Audit record ID
    pub record_id: String,
    
    /// Company ID this record relates to
    pub company_id: String,
    
    /// Action performed
    pub action: AuditAction,
    
    /// User who performed the action
    pub performed_by: String,
    
    /// Timestamp of action
    pub timestamp: DateTime<Utc>,
    
    /// Previous state (for updates)
    pub previous_state: Option<serde_json::Value>,
    
    /// New state (for updates)
    pub new_state: Option<serde_json::Value>,
    
    /// Additional context
    pub context: HashMap<String, String>,
}

/// Audit actions for company operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    /// Company registration
    Registration,
    
    /// Company information update
    Update,
    
    /// Status change
    StatusChange,
    
    /// Financial information update
    FinancialUpdate,
    
    /// Contact information update
    ContactUpdate,
    
    /// Company deactivation
    Deactivation,
    
    /// Company reactivation
    Reactivation,
}

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Maximum companies per owner
    pub max_companies_per_owner: u32,
    
    /// Auto-approval for certain company types
    pub auto_approve_types: Vec<CompanyType>,
    
    /// Require manual review for large companies
    pub require_review_threshold: Option<u64>, // Revenue threshold
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_companies_per_owner: 10,
            auto_approve_types: vec![CompanyType::Startup, CompanyType::SMB],
            require_review_threshold: Some(10_000_000), // $10M revenue
        }
    }
}

impl CompanyRegistry {
    /// Create new company registry
    pub async fn new() -> Result<Self> {
        Ok(Self {
            companies: Arc::new(RwLock::new(HashMap::new())),
            audit_trail: Arc::new(RwLock::new(Vec::new())),
            config: RegistryConfig::default(),
        })
    }
    
    /// Register a new company
    pub async fn register_company(
        &self,
        name: String,
        description: String,
        company_type: CompanyType,
        owner_id: String,
        contact_info: ContactInfo,
        financial_info: FinancialInfo,
    ) -> Result<String> {
        let company_id = format!("comp-{}", Uuid::new_v4());
        let now = Utc::now();
        
        // Check if owner has reached company limit
        let companies = self.companies.read().await;
        let owner_company_count = companies.values()
            .filter(|c| c.owner_id == owner_id)
            .count() as u32;
        
        if owner_company_count >= self.config.max_companies_per_owner {
            return Err(anyhow::anyhow!(
                "Owner has reached maximum company limit of {}",
                self.config.max_companies_per_owner
            ));
        }
        drop(companies);
        
        // Determine initial status based on configuration
        let status = if self.config.auto_approve_types.contains(&company_type) {
            if let Some(threshold) = self.config.require_review_threshold {
                if financial_info.annual_revenue.unwrap_or(0) > threshold {
                    CompanyStatus::UnderReview
                } else {
                    CompanyStatus::Active
                }
            } else {
                CompanyStatus::Active
            }
        } else {
            CompanyStatus::Pending
        };
        
        let company_metadata = CompanyMetadata {
            company_id: company_id.clone(),
            name: name.clone(),
            description,
            company_type: company_type.clone(),
            status: status.clone(),
            owner_id: owner_id.clone(),
            contact_info,
            financial_info,
            operational_metrics: OperationalMetrics {
                monthly_active_users: None,
                monthly_recurring_revenue: None,
                customer_count: None,
                growth_rate: None,
                market_presence_score: None,
            },
            registered_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        };
        
        // Store company
        {
            let mut companies = self.companies.write().await;
            companies.insert(company_id.clone(), company_metadata.clone());
        }
        
        // Record audit trail
        self.record_audit(
            company_id.clone(),
            AuditAction::Registration,
            owner_id,
            None,
            Some(serde_json::to_value(&company_metadata)?),
            [("company_name".to_string(), name)].into(),
        ).await?;
        
        Ok(company_id)
    }
    
    /// Get company by ID
    pub async fn get_company(&self, company_id: &str) -> Result<Option<CompanyMetadata>> {
        let companies = self.companies.read().await;
        Ok(companies.get(company_id).cloned())
    }
    
    /// Get all companies for an owner
    pub async fn get_companies_by_owner(&self, owner_id: &str) -> Result<Vec<CompanyMetadata>> {
        let companies = self.companies.read().await;
        let owner_companies: Vec<CompanyMetadata> = companies.values()
            .filter(|c| c.owner_id == owner_id)
            .cloned()
            .collect();
        Ok(owner_companies)
    }
    
    /// Get all companies with optional filtering
    pub async fn get_all_companies(
        &self,
        status_filter: Option<CompanyStatus>,
        company_type_filter: Option<CompanyType>,
    ) -> Result<Vec<CompanyMetadata>> {
        let companies = self.companies.read().await;
        let mut filtered_companies: Vec<CompanyMetadata> = companies.values().cloned().collect();
        
        if let Some(status) = status_filter {
            filtered_companies.retain(|c| std::mem::discriminant(&c.status) == std::mem::discriminant(&status));
        }
        
        if let Some(company_type) = company_type_filter {
            filtered_companies.retain(|c| std::mem::discriminant(&c.company_type) == std::mem::discriminant(&company_type));
        }
        
        Ok(filtered_companies)
    }
    
    /// Update company information
    pub async fn update_company(
        &self,
        company_id: &str,
        updates: CompanyUpdateRequest,
        updated_by: String,
    ) -> Result<()> {
        let mut companies = self.companies.write().await;
        
        if let Some(company) = companies.get_mut(company_id) {
            let previous_state = serde_json::to_value(company.clone())?;
            
            // Apply updates
            if let Some(name) = updates.name {
                company.name = name;
            }
            if let Some(description) = updates.description {
                company.description = description;
            }
            if let Some(contact_info) = updates.contact_info {
                company.contact_info = contact_info;
            }
            if let Some(financial_info) = updates.financial_info {
                company.financial_info = financial_info;
            }
            if let Some(operational_metrics) = updates.operational_metrics {
                company.operational_metrics = operational_metrics;
            }
            if let Some(metadata) = updates.metadata {
                company.metadata.extend(metadata);
            }
            
            company.updated_at = Utc::now();
            
            let new_state = serde_json::to_value(company.clone())?;
            
            drop(companies);
            
            // Record audit trail
            self.record_audit(
                company_id.to_string(),
                AuditAction::Update,
                updated_by,
                Some(previous_state),
                Some(new_state),
                HashMap::new(),
            ).await?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Company not found: {}", company_id))
        }
    }
    
    /// Update company status
    pub async fn update_company_status(
        &self,
        company_id: &str,
        new_status: CompanyStatus,
        updated_by: String,
        reason: Option<String>,
    ) -> Result<()> {
        let mut companies = self.companies.write().await;
        
        if let Some(company) = companies.get_mut(company_id) {
            let previous_status = company.status.clone();
            company.status = new_status.clone();
            company.updated_at = Utc::now();
            
            drop(companies);
            
            let mut context = HashMap::new();
            context.insert("previous_status".to_string(), format!("{:?}", previous_status));
            context.insert("new_status".to_string(), format!("{:?}", new_status));
            if let Some(reason) = reason {
                context.insert("reason".to_string(), reason);
            }
            
            // Record audit trail
            self.record_audit(
                company_id.to_string(),
                AuditAction::StatusChange,
                updated_by,
                None,
                None,
                context,
            ).await?;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Company not found: {}", company_id))
        }
    }
    
    /// Get company audit trail
    pub async fn get_company_audit_trail(&self, company_id: &str) -> Result<Vec<CompanyAuditRecord>> {
        let audit_trail = self.audit_trail.read().await;
        let company_audit: Vec<CompanyAuditRecord> = audit_trail.iter()
            .filter(|record| record.company_id == company_id)
            .cloned()
            .collect();
        Ok(company_audit)
    }
    
    /// Get registry statistics
    pub async fn get_registry_statistics(&self) -> Result<RegistryStatistics> {
        let companies = self.companies.read().await;
        
        let total_companies = companies.len();
        let mut status_counts = HashMap::new();
        let mut type_counts = HashMap::new();
        
        for company in companies.values() {
            let status_key = format!("{:?}", company.status);
            *status_counts.entry(status_key).or_insert(0) += 1;
            
            let type_key = format!("{:?}", company.company_type);
            *type_counts.entry(type_key).or_insert(0) += 1;
        }
        
        Ok(RegistryStatistics {
            total_companies,
            companies_by_status: status_counts,
            companies_by_type: type_counts,
            total_owners: companies.values().map(|c| &c.owner_id).collect::<std::collections::HashSet<_>>().len(),
        })
    }
    
    /// Record audit trail entry
    async fn record_audit(
        &self,
        company_id: String,
        action: AuditAction,
        performed_by: String,
        previous_state: Option<serde_json::Value>,
        new_state: Option<serde_json::Value>,
        context: HashMap<String, String>,
    ) -> Result<()> {
        let audit_record = CompanyAuditRecord {
            record_id: format!("audit-{}", Uuid::new_v4()),
            company_id,
            action,
            performed_by,
            timestamp: Utc::now(),
            previous_state,
            new_state,
            context,
        };
        
        let mut audit_trail = self.audit_trail.write().await;
        audit_trail.push(audit_record);
        
        Ok(())
    }
}

/// Company update request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyUpdateRequest {
    /// Updated name
    pub name: Option<String>,
    
    /// Updated description
    pub description: Option<String>,
    
    /// Updated contact information
    pub contact_info: Option<ContactInfo>,
    
    /// Updated financial information
    pub financial_info: Option<FinancialInfo>,
    
    /// Updated operational metrics
    pub operational_metrics: Option<OperationalMetrics>,
    
    /// Additional metadata updates
    pub metadata: Option<HashMap<String, String>>,
}

/// Registry statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStatistics {
    /// Total number of registered companies
    pub total_companies: usize,
    
    /// Companies grouped by status
    pub companies_by_status: HashMap<String, u32>,
    
    /// Companies grouped by type
    pub companies_by_type: HashMap<String, u32>,
    
    /// Total number of unique owners
    pub total_owners: usize,
}
