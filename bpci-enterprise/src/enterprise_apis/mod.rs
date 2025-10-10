//! Enterprise APIs Module
//! 
//! Provides comprehensive API endpoints for enterprise owner dashboard,
//! company management, and SAPI mesh integration. Built on top of existing
//! infrastructure including ComprehensiveWalletRegistry, AutonomousEconomy,
//! and Central Orchestration systems.

pub mod owner_dashboard;
pub mod company_management;
pub mod sapi_mesh_management;
pub mod company_registry;

// Re-export main API structures
pub use owner_dashboard::{OwnerDashboardAPI, DashboardOverview, DashboardMetrics};
pub use company_management::{CompanyManagementAPI, Company, CompanyAuditRecord};
pub use sapi_mesh_management::{SAPIMeshManagementAPI, MeshStatus, MeshNode};
pub use company_registry::{CompanyRegistry, CompanyMetadata};

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::wallet_registry::ComprehensiveWalletRegistry;
use crate::central_orchestration::BPCICentralOrchestrator;

/// Enterprise API Server state combining all enterprise services
#[derive(Clone)]
pub struct EnterpriseAPIState {
    pub owner_dashboard: Arc<OwnerDashboardAPI>,
    pub company_management: Arc<CompanyManagementAPI>,
    pub sapi_mesh_management: Arc<SAPIMeshManagementAPI>,
    pub company_registry: Arc<CompanyRegistry>,
}

impl EnterpriseAPIState {
    /// Create new enterprise API state with all services
    pub async fn new(
        wallet_registry: Arc<ComprehensiveWalletRegistry>,
        orchestration: Arc<BPCICentralOrchestrator>,
    ) -> Result<Self> {
        // Create company registry first (needed by other services)
        let company_registry = Arc::new(CompanyRegistry::new().await?);
        
        // Create individual API services
        let owner_dashboard = Arc::new(
            OwnerDashboardAPI::new(
                wallet_registry.clone(),
                orchestration.clone(),
                company_registry.clone(),
            ).await?
        );
        
        let company_management = Arc::new(
            CompanyManagementAPI::new(
                company_registry.clone(),
                wallet_registry.clone(),
            ).await?
        );
        
        let sapi_mesh_management = Arc::new(
            SAPIMeshManagementAPI::new().await?
        );
        
        Ok(Self {
            owner_dashboard,
            company_management,
            sapi_mesh_management,
            company_registry,
        })
    }
}
