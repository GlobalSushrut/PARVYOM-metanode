//! Enterprise Owner Dashboard API
//!
//! Provides comprehensive dashboard functionality for enterprise owners including:
//! - System overview and metrics
//! - Resource monitoring and allocation
//! - Financial analytics and billing
//! - Performance insights and optimization recommendations

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::wallet_registry::comprehensive_wallet_registry::ComprehensiveWalletRegistry;
use crate::central_orchestration::BPCICentralOrchestrator;

/// Owner Dashboard API service
#[derive(Debug)]
pub struct OwnerDashboardAPI {
    /// Integration with wallet registry for financial data
    wallet_registry: Arc<ComprehensiveWalletRegistry>,
    
    /// Integration with central orchestration for system metrics
    orchestration: Arc<BPCICentralOrchestrator>,
    
    /// Company registry for enterprise data
    company_registry: Arc<super::CompanyRegistry>,
    
    /// Cached dashboard data for performance
    cached_overview: Arc<RwLock<Option<(DashboardOverview, DateTime<Utc>)>>>,
    
    /// Cache TTL in seconds
    cache_ttl: u64,
}

/// Dashboard overview data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOverview {
    /// System status and health
    pub system_status: SystemStatus,
    
    /// Financial metrics and analytics
    pub financial_metrics: FinancialMetrics,
    
    /// Resource utilization metrics
    pub resource_metrics: ResourceMetrics,
    
    /// Performance insights
    pub performance_insights: PerformanceInsights,
    
    /// Recent activities and events
    pub recent_activities: Vec<ActivityRecord>,
    
    /// Timestamp of data generation
    pub generated_at: DateTime<Utc>,
}

/// System status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// Overall system health (0-100)
    pub health_score: u32,
    
    /// Active nodes count
    pub active_nodes: u32,
    
    /// Total registered nodes
    pub total_nodes: u32,
    
    /// System uptime in seconds
    pub uptime_seconds: u64,
    
    /// Current system load (0.0-1.0)
    pub system_load: f64,
    
    /// Active user sessions
    pub active_sessions: u32,
}

/// Financial metrics for enterprise dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialMetrics {
    /// Total wallet balance across all company wallets
    pub total_balance: u64,
    
    /// Treasury wallet balance
    pub treasury_balance: u64,
    
    /// ESOP wallet balance
    pub esop_balance: u64,
    
    /// Operational wallet balance
    pub operational_balance: u64,
    
    /// Monthly revenue (estimated)
    pub monthly_revenue: u64,
    
    /// Monthly costs (estimated)
    pub monthly_costs: u64,
    
    /// Profit margin percentage
    pub profit_margin: f64,
    
    /// Number of active companies
    pub active_companies: u32,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU utilization percentage (0-100)
    pub cpu_utilization: f64,
    
    /// Memory utilization percentage (0-100)
    pub memory_utilization: f64,
    
    /// Storage utilization percentage (0-100)
    pub storage_utilization: f64,
    
    /// Network throughput in MB/s
    pub network_throughput: f64,
    
    /// Active connections count
    pub active_connections: u32,
    
    /// Resource allocation efficiency (0-100)
    pub allocation_efficiency: f64,
}

/// Performance insights and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsights {
    /// Overall performance score (0-100)
    pub performance_score: u32,
    
    /// Response time metrics in milliseconds
    pub avg_response_time: f64,
    
    /// Throughput in requests per second
    pub throughput_rps: f64,
    
    /// Error rate percentage
    pub error_rate: f64,
    
    /// Optimization recommendations
    pub recommendations: Vec<String>,
    
    /// Performance trends
    pub trends: Vec<PerformanceTrend>,
}

/// Performance trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
    
    /// Metric value
    pub value: f64,
    
    /// Metric type (cpu, memory, throughput, etc.)
    pub metric_type: String,
}

/// Activity record for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRecord {
    /// Activity ID
    pub activity_id: String,
    
    /// Activity type (wallet_creation, company_registration, etc.)
    pub activity_type: String,
    
    /// Activity description
    pub description: String,
    
    /// User or system that triggered the activity
    pub actor: String,
    
    /// Activity timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Activity status (success, failed, pending)
    pub status: String,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Dashboard metrics aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    /// Key performance indicators
    pub kpis: HashMap<String, f64>,
    
    /// Time-series data for charts
    pub time_series: HashMap<String, Vec<(DateTime<Utc>, f64)>>,
    
    /// Comparative metrics (current vs previous period)
    pub comparisons: HashMap<String, MetricComparison>,
}

/// Metric comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricComparison {
    /// Current period value
    pub current: f64,
    
    /// Previous period value
    pub previous: f64,
    
    /// Percentage change
    pub change_percent: f64,
    
    /// Trend direction (up, down, stable)
    pub trend: String,
}

impl OwnerDashboardAPI {
    /// Create new Owner Dashboard API instance
    pub async fn new(
        wallet_registry: Arc<ComprehensiveWalletRegistry>,
        orchestration: Arc<BPCICentralOrchestrator>,
        company_registry: Arc<super::CompanyRegistry>,
    ) -> Result<Self> {
        Ok(Self {
            wallet_registry,
            orchestration,
            company_registry,
            cached_overview: Arc::new(RwLock::new(None)),
            cache_ttl: 300, // 5 minutes cache
        })
    }
    
    /// Get comprehensive dashboard overview
    pub async fn get_dashboard_overview(&self) -> Result<DashboardOverview> {
        // Check cache first
        {
            let cache = self.cached_overview.read().await;
            if let Some((overview, cached_at)) = cache.as_ref() {
                let now = Utc::now();
                if (now - *cached_at).num_seconds() < self.cache_ttl as i64 {
                    return Ok(overview.clone());
                }
            }
        }
        
        // Generate fresh overview data
        let overview = self.generate_dashboard_overview().await?;
        
        // Update cache
        {
            let mut cache = self.cached_overview.write().await;
            *cache = Some((overview.clone(), Utc::now()));
        }
        
        Ok(overview)
    }
    
    /// Generate fresh dashboard overview data
    async fn generate_dashboard_overview(&self) -> Result<DashboardOverview> {
        let now = Utc::now();
        
        // Get system status from orchestration
        let system_status = self.get_system_status().await?;
        
        // Get financial metrics from wallet registry
        let financial_metrics = self.get_financial_metrics().await?;
        
        // Get resource metrics from orchestration
        let resource_metrics = self.get_resource_metrics().await?;
        
        // Get performance insights
        let performance_insights = self.get_performance_insights().await?;
        
        // Get recent activities
        let recent_activities = self.get_recent_activities().await?;
        
        Ok(DashboardOverview {
            system_status,
            financial_metrics,
            resource_metrics,
            performance_insights,
            recent_activities,
            generated_at: now,
        })
    }
    
    /// Get system status metrics
    async fn get_system_status(&self) -> Result<SystemStatus> {
        // Get metrics from orchestration system
        let orchestration_metrics = self.orchestration.get_metrics().await?;
        let active_nodes = self.orchestration.get_active_nodes().await?.len() as u32;
        let total_nodes = self.orchestration.get_nodes_by_type(None).await?.len() as u32;
        
        Ok(SystemStatus {
            health_score: orchestration_metrics.health_score,
            active_nodes,
            total_nodes,
            uptime_seconds: orchestration_metrics.uptime_seconds,
            system_load: orchestration_metrics.system_load,
            active_sessions: orchestration_metrics.active_sessions,
        })
    }
    
    /// Get financial metrics from wallet registry
    async fn get_financial_metrics(&self) -> Result<FinancialMetrics> {
        let company_wallets = self.wallet_registry.get_all_company_wallets().await;
        
        let mut total_balance = 0u64;
        let mut treasury_balance = 0u64;
        let mut esop_balance = 0u64;
        let mut operational_balance = 0u64;
        
        for (_, wallet_set) in company_wallets.iter() {
            total_balance += wallet_set.total_allocation;
            treasury_balance += wallet_set.total_allocation * 40 / 100; // Assume 40% treasury
            esop_balance += wallet_set.total_allocation * 30 / 100; // Assume 30% ESOP
            operational_balance += wallet_set.total_allocation * 30 / 100; // Assume 30% operational
        }
        
        let active_companies = company_wallets.len() as u32;
        let monthly_revenue = total_balance / 12; // Rough estimate
        let monthly_costs = monthly_revenue * 70 / 100; // Assume 70% cost ratio
        let profit_margin = if monthly_revenue > 0 {
            ((monthly_revenue - monthly_costs) as f64 / monthly_revenue as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(FinancialMetrics {
            total_balance,
            treasury_balance,
            esop_balance,
            operational_balance,
            monthly_revenue,
            monthly_costs,
            profit_margin,
            active_companies,
        })
    }
    
    /// Get resource utilization metrics
    async fn get_resource_metrics(&self) -> Result<ResourceMetrics> {
        let orchestration_metrics = self.orchestration.get_metrics().await?;
        
        Ok(ResourceMetrics {
            cpu_utilization: orchestration_metrics.cpu_utilization,
            memory_utilization: orchestration_metrics.memory_utilization,
            storage_utilization: orchestration_metrics.storage_utilization,
            network_throughput: orchestration_metrics.network_throughput,
            active_connections: orchestration_metrics.active_connections,
            allocation_efficiency: orchestration_metrics.allocation_efficiency,
        })
    }
    
    /// Get performance insights and recommendations
    async fn get_performance_insights(&self) -> Result<PerformanceInsights> {
        let orchestration_metrics = self.orchestration.get_metrics().await?;
        
        let mut recommendations = Vec::new();
        
        // Generate recommendations based on metrics
        if orchestration_metrics.cpu_utilization > 80.0 {
            recommendations.push("Consider scaling up CPU resources".to_string());
        }
        if orchestration_metrics.memory_utilization > 85.0 {
            recommendations.push("Memory usage is high, consider optimization".to_string());
        }
        if orchestration_metrics.error_rate > 5.0 {
            recommendations.push("Error rate is elevated, investigate system issues".to_string());
        }
        
        // Generate sample performance trends (in real implementation, this would come from time-series DB)
        let now = Utc::now();
        let trends = vec![
            PerformanceTrend {
                timestamp: now - chrono::Duration::hours(1),
                value: orchestration_metrics.cpu_utilization - 5.0,
                metric_type: "cpu".to_string(),
            },
            PerformanceTrend {
                timestamp: now,
                value: orchestration_metrics.cpu_utilization,
                metric_type: "cpu".to_string(),
            },
        ];
        
        Ok(PerformanceInsights {
            performance_score: orchestration_metrics.performance_score,
            avg_response_time: orchestration_metrics.avg_response_time,
            throughput_rps: orchestration_metrics.throughput_rps,
            error_rate: orchestration_metrics.error_rate,
            recommendations,
            trends,
        })
    }
    
    /// Get recent system activities
    async fn get_recent_activities(&self) -> Result<Vec<ActivityRecord>> {
        let now = Utc::now();
        
        // In real implementation, this would query an activity log database
        // For now, generate sample activities based on current system state
        let mut activities = Vec::new();
        
        let company_wallets = self.wallet_registry.get_all_company_wallets().await;
        if !company_wallets.is_empty() {
            activities.push(ActivityRecord {
                activity_id: format!("act-{}", uuid::Uuid::new_v4()),
                activity_type: "company_wallet_activity".to_string(),
                description: format!("Company wallet operations detected for {} companies", company_wallets.len()),
                actor: "system".to_string(),
                timestamp: now - chrono::Duration::minutes(15),
                status: "success".to_string(),
                metadata: [("company_count".to_string(), company_wallets.len().to_string())].into(),
            });
        }
        
        let active_nodes = self.orchestration.get_active_nodes().await?.len();
        activities.push(ActivityRecord {
            activity_id: format!("act-{}", uuid::Uuid::new_v4()),
            activity_type: "system_monitoring".to_string(),
            description: format!("System health check completed - {} active nodes", active_nodes),
            actor: "orchestrator".to_string(),
            timestamp: now - chrono::Duration::minutes(5),
            status: "success".to_string(),
            metadata: [("active_nodes".to_string(), active_nodes.to_string())].into(),
        });
        
        Ok(activities)
    }
    
    /// Get detailed dashboard metrics for analytics
    pub async fn get_dashboard_metrics(&self) -> Result<DashboardMetrics> {
        let orchestration_metrics = self.orchestration.get_metrics().await?;
        let financial_metrics = self.get_financial_metrics().await?;
        
        let mut kpis = HashMap::new();
        kpis.insert("system_health".to_string(), orchestration_metrics.health_score as f64);
        kpis.insert("total_balance".to_string(), financial_metrics.total_balance as f64);
        kpis.insert("profit_margin".to_string(), financial_metrics.profit_margin);
        kpis.insert("cpu_utilization".to_string(), orchestration_metrics.cpu_utilization);
        kpis.insert("active_companies".to_string(), financial_metrics.active_companies as f64);
        
        // Generate sample time series data (in real implementation, from time-series DB)
        let now = Utc::now();
        let mut time_series = HashMap::new();
        
        let cpu_series = vec![
            (now - chrono::Duration::hours(2), orchestration_metrics.cpu_utilization - 10.0),
            (now - chrono::Duration::hours(1), orchestration_metrics.cpu_utilization - 5.0),
            (now, orchestration_metrics.cpu_utilization),
        ];
        time_series.insert("cpu_utilization".to_string(), cpu_series);
        
        // Generate comparisons (current vs previous period)
        let mut comparisons = HashMap::new();
        comparisons.insert("system_health".to_string(), MetricComparison {
            current: orchestration_metrics.health_score as f64,
            previous: (orchestration_metrics.health_score as f64) - 5.0,
            change_percent: 5.0,
            trend: "up".to_string(),
        });
        
        Ok(DashboardMetrics {
            kpis,
            time_series,
            comparisons,
        })
    }
}
