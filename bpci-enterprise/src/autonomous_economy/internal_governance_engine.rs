//! Internal Governance Engine Implementation
//! 
//! Core business logic for the internal governance system

use super::internal_governance::*;
use crate::autonomous_economy::mother_coin_distribution::MotherCoinDistributionEngine;
use anyhow::{Result, anyhow};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Main Internal Governance Engine
#[derive(Debug)]
pub struct InternalGovernanceEngine {
    /// Distribution engine for 75%/25% split
    pub distribution_engine: Arc<RwLock<InternalDistributionEngine>>,
    /// Community ticket system
    pub ticket_system: Arc<RwLock<CommunityTicketSystem>>,
    /// Governance dashboard
    pub dashboard: Arc<RwLock<GovernanceDashboard>>,
    /// BPCI VM for autonomous execution
    pub bpci_vm: Arc<RwLock<BpciVirtualMachine>>,
    /// Integration with mother coin system
    pub mother_coin_engine: Arc<RwLock<MotherCoinDistributionEngine>>,
}

impl InternalGovernanceEngine {
    /// Initialize new governance engine
    pub fn new(mother_coin_engine: Arc<RwLock<MotherCoinDistributionEngine>>) -> Self {
        Self {
            distribution_engine: Arc::new(RwLock::new(InternalDistributionEngine::default())),
            ticket_system: Arc::new(RwLock::new(CommunityTicketSystem::default())),
            dashboard: Arc::new(RwLock::new(GovernanceDashboard::default())),
            bpci_vm: Arc::new(RwLock::new(BpciVirtualMachine::default())),
            mother_coin_engine,
        }
    }

    /// Calculate and execute internal distribution (75% autonomous, 25% company)
    pub async fn execute_internal_distribution(
        &self,
        total_amount: Decimal,
        reason: String,
        approved_by: Vec<String>,
    ) -> Result<DistributionRecord> {
        let mut engine = self.distribution_engine.write().await;
        
        // Calculate 75%/25% split
        let autonomous_amount = total_amount * Decimal::from_str_exact("0.75")?;
        let company_amount = total_amount * Decimal::from_str_exact("0.25")?;
        
        // Create distribution record
        let record = DistributionRecord {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            total_amount,
            autonomous_amount,
            company_amount,
            reason,
            approved_by,
            execution_status: ExecutionStatus::Completed,
        };
        
        // Update engine state
        engine.total_funds += total_amount;
        engine.autonomous_allocation += autonomous_amount;
        engine.company_allocation += company_amount;
        engine.distribution_history.push(record.clone());
        
        // Update performance metrics
        self.update_performance_metrics().await?;
        
        // Log to BPCI VM
        self.log_vm_execution(
            "internal_distribution".to_string(),
            format!("Distributed {} total: {} autonomous, {} company", 
                   total_amount, autonomous_amount, company_amount),
        ).await?;
        
        Ok(record)
    }

    /// Create new governance ticket
    pub async fn create_governance_ticket(
        &self,
        title: String,
        description: String,
        category: TicketCategory,
        priority: TicketPriority,
        created_by: String,
        voting_period_hours: Option<u32>,
    ) -> Result<Uuid> {
        let mut system = self.ticket_system.write().await;
        
        // Calculate voting deadline
        let voting_hours = voting_period_hours.unwrap_or(
            match priority {
                TicketPriority::Critical => system.voting_config.min_voting_period,
                TicketPriority::High => 48,
                TicketPriority::Medium => 72,
                TicketPriority::Low => system.voting_config.max_voting_period,
            }
        );
        
        let voting_deadline = Utc::now() + Duration::hours(voting_hours as i64);
        
        // Determine approval threshold based on category
        let required_threshold = match category {
            TicketCategory::Emergency => system.voting_config.emergency_threshold,
            TicketCategory::Security => 0.7,
            TicketCategory::Economic => 0.65,
            _ => system.voting_config.approval_threshold,
        };
        
        let ticket = GovernanceTicket {
            id: Uuid::new_v4(),
            title,
            description,
            category,
            priority,
            created_by: created_by.clone(),
            created_at: Utc::now(),
            voting_deadline,
            required_approval_threshold: required_threshold,
            votes: HashMap::new(),
            status: TicketStatus::Open,
            execution_plan: None,
            audit_trail: vec![AuditEntry {
                timestamp: Utc::now(),
                actor: created_by,
                action: "ticket_created".to_string(),
                details: HashMap::new(),
                result: "success".to_string(),
            }],
        };
        
        let ticket_id = ticket.id;
        system.active_tickets.insert(ticket_id, ticket);
        
        // Update dashboard metrics
        self.update_dashboard_metrics().await?;
        
        Ok(ticket_id)
    }

    /// Submit vote on governance ticket
    pub async fn submit_vote(
        &self,
        ticket_id: Uuid,
        voter: String,
        decision: VoteDecision,
        reasoning: Option<String>,
    ) -> Result<()> {
        let mut system = self.ticket_system.write().await;
        
        // Get stakeholder voting weight
        let voting_weight = system.stakeholders
            .get(&voter)
            .map(|s| s.voting_weight)
            .unwrap_or(1.0);
        
        // Find ticket
        let ticket = system.active_tickets
            .get_mut(&ticket_id)
            .ok_or_else(|| anyhow!("Ticket not found: {}", ticket_id))?;
        
        // Check if voting is still open
        if ticket.status != TicketStatus::Open && ticket.status != TicketStatus::Voting {
            return Err(anyhow!("Voting is closed for ticket {}", ticket_id));
        }
        
        if Utc::now() > ticket.voting_deadline {
            return Err(anyhow!("Voting deadline has passed for ticket {}", ticket_id));
        }
        
        // Submit vote
        let vote = Vote {
            voter: voter.clone(),
            decision: decision.clone(),
            weight: voting_weight,
            timestamp: Utc::now(),
            reasoning,
        };
        
        ticket.votes.insert(voter.clone(), vote);
        ticket.status = TicketStatus::Voting;
        
        // Add audit entry
        ticket.audit_trail.push(AuditEntry {
            timestamp: Utc::now(),
            actor: voter,
            action: "vote_submitted".to_string(),
            details: [("decision".to_string(), format!("{:?}", decision))]
                .iter().cloned().collect(),
            result: "success".to_string(),
        });
        
        // Check if voting is complete
        self.check_voting_completion(ticket_id).await?;
        
        Ok(())
    }

    /// Check if voting is complete and process results
    async fn check_voting_completion(&self, ticket_id: Uuid) -> Result<()> {
        let (total_weight, votes_weight, approve_weight, quorum_threshold, voting_deadline, required_threshold) = {
            let system = self.ticket_system.read().await;
            let ticket = system.active_tickets
                .get(&ticket_id)
                .ok_or_else(|| anyhow!("Ticket not found: {}", ticket_id))?;
            
            // Calculate voting results
            let total_weight: f64 = system.stakeholders.values()
                .map(|s| s.voting_weight)
                .sum();
            
            let votes_weight: f64 = ticket.votes.values()
                .map(|v| v.weight)
                .sum();
            
            let approve_weight: f64 = ticket.votes.values()
                .filter(|v| matches!(v.decision, VoteDecision::Approve))
                .map(|v| v.weight)
                .sum();
            
            (total_weight, votes_weight, approve_weight, system.voting_config.quorum_threshold, ticket.voting_deadline, ticket.required_approval_threshold)
        };
        
        let quorum_met = votes_weight / total_weight >= quorum_threshold;
        let approval_met = approve_weight / votes_weight >= required_threshold;
        
        // Check if voting should close
        let voting_complete = quorum_met && 
            (approval_met || Utc::now() > voting_deadline);
        
        if voting_complete {
            let mut system = self.ticket_system.write().await;
            if let Some(ticket) = system.active_tickets.get_mut(&ticket_id) {
                ticket.status = if approval_met {
                    TicketStatus::Approved
                } else {
                    TicketStatus::Rejected
                };
                
                // Move to completed tickets
                let completed_ticket = ticket.clone();
                system.completed_tickets.insert(ticket_id, completed_ticket);
                system.active_tickets.remove(&ticket_id);
            }
            
            // Execute if approved
            if approval_met {
                self.execute_approved_ticket(ticket_id).await?;
            }
        }
        
        Ok(())
    }

    /// Execute approved governance ticket
    async fn execute_approved_ticket(&self, ticket_id: Uuid) -> Result<()> {
        let system = self.ticket_system.read().await;
        let ticket = system.completed_tickets
            .get(&ticket_id)
            .ok_or_else(|| anyhow!("Completed ticket not found: {}", ticket_id))?;
        
        // Create execution plan based on ticket category
        let execution_plan = self.create_execution_plan(ticket).await?;
        
        // Execute via BPCI VM
        self.execute_via_vm(ticket_id, execution_plan).await?;
        
        Ok(())
    }

    /// Create execution plan for approved ticket
    async fn create_execution_plan(&self, ticket: &GovernanceTicket) -> Result<ExecutionPlan> {
        let steps = match ticket.category {
            TicketCategory::Infrastructure => vec![
                ExecutionStep {
                    step_id: 1,
                    description: "Prepare infrastructure changes".to_string(),
                    action: ExecutionAction {
                        action_type: ActionType::ExecuteInfrastructure,
                        parameters: [("ticket_id".to_string(), ticket.id.to_string())]
                            .iter().cloned().collect(),
                        priority: 1,
                        rollback_plan: Some("Revert infrastructure changes".to_string()),
                    },
                    dependencies: vec![],
                    estimated_time: 3600, // 1 hour
                },
                ExecutionStep {
                    step_id: 2,
                    description: "Notify stakeholders".to_string(),
                    action: ExecutionAction {
                        action_type: ActionType::NotifyStakeholders,
                        parameters: [("message".to_string(), "Infrastructure updated".to_string())]
                            .iter().cloned().collect(),
                        priority: 2,
                        rollback_plan: None,
                    },
                    dependencies: vec![1],
                    estimated_time: 300, // 5 minutes
                },
            ],
            TicketCategory::Economic => vec![
                ExecutionStep {
                    step_id: 1,
                    description: "Update economic parameters".to_string(),
                    action: ExecutionAction {
                        action_type: ActionType::UpdateAllocation,
                        parameters: [("ticket_id".to_string(), ticket.id.to_string())]
                            .iter().cloned().collect(),
                        priority: 1,
                        rollback_plan: Some("Revert economic changes".to_string()),
                    },
                    dependencies: vec![],
                    estimated_time: 1800, // 30 minutes
                },
            ],
            _ => vec![
                ExecutionStep {
                    step_id: 1,
                    description: "Generate execution report".to_string(),
                    action: ExecutionAction {
                        action_type: ActionType::GenerateReport,
                        parameters: [("ticket_id".to_string(), ticket.id.to_string())]
                            .iter().cloned().collect(),
                        priority: 1,
                        rollback_plan: None,
                    },
                    dependencies: vec![],
                    estimated_time: 600, // 10 minutes
                },
            ],
        };
        
        let total_time: u32 = steps.iter().map(|s| s.estimated_time).sum();
        
        Ok(ExecutionPlan {
            steps,
            estimated_duration: total_time,
            resource_requirements: ResourcePool::default(),
            rollback_plan: "Automated rollback available".to_string(),
            success_criteria: vec![
                "All steps completed successfully".to_string(),
                "No errors reported".to_string(),
                "Stakeholders notified".to_string(),
            ],
        })
    }

    /// Execute plan via BPCI VM
    async fn execute_via_vm(&self, ticket_id: Uuid, plan: ExecutionPlan) -> Result<()> {
        let mut vm = self.bpci_vm.write().await;
        
        // Create execution record
        let execution_record = ExecutionRecord {
            id: Uuid::new_v4(),
            policy_id: ticket_id, // Using ticket_id as policy_id for this execution
            executed_at: Utc::now(),
            trigger_reason: "Governance ticket approved".to_string(),
            actions_taken: plan.steps.iter()
                .map(|s| s.description.clone())
                .collect(),
            execution_status: ExecutionStatus::InProgress,
            resource_usage: ResourceUsage {
                cpu_time: 0.0,
                memory_used: 0,
                network_io: 0,
                storage_io: 0,
                execution_cost: Decimal::ZERO,
            },
            audit_log: vec![AuditEntry {
                timestamp: Utc::now(),
                actor: "BPCI_VM".to_string(),
                action: "execution_started".to_string(),
                details: [("ticket_id".to_string(), ticket_id.to_string())]
                    .iter().cloned().collect(),
                result: "success".to_string(),
            }],
        };
        
        vm.execution_history.push(execution_record);
        vm.execution_context.current_state = VmState::Executing;
        
        // Simulate execution (in real implementation, this would execute the actual steps)
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        vm.execution_context.current_state = VmState::Idle;
        
        Ok(())
    }

    /// Update performance metrics
    async fn update_performance_metrics(&self) -> Result<()> {
        let mut engine = self.distribution_engine.write().await;
        
        // Calculate performance based on recent distributions and outcomes
        let recent_distributions = engine.distribution_history
            .iter()
            .filter(|d| d.timestamp > Utc::now() - Duration::days(30))
            .collect::<Vec<_>>();
        
        // Simple performance calculation (in real implementation, this would be more sophisticated)
        let autonomous_performance = if recent_distributions.is_empty() {
            0.5
        } else {
            0.7 + (recent_distributions.len() as f64 * 0.05).min(0.3)
        };
        
        let company_performance = if recent_distributions.is_empty() {
            0.5
        } else {
            0.6 + (recent_distributions.len() as f64 * 0.03).min(0.4)
        };
        
        engine.performance_metrics = PerformanceMetrics {
            autonomous_performance,
            company_performance,
            community_satisfaction: 0.75, // Would be calculated from stakeholder feedback
            infrastructure_efficiency: 0.8, // Would be calculated from system metrics
            last_updated: Utc::now(),
        };
        
        Ok(())
    }

    /// Update dashboard metrics
    async fn update_dashboard_metrics(&self) -> Result<()> {
        let ticket_system = self.ticket_system.read().await;
        let mut dashboard = self.dashboard.write().await;
        
        let total_tickets = ticket_system.active_tickets.len() + ticket_system.completed_tickets.len();
        let active_tickets = ticket_system.active_tickets.len();
        let completed_tickets = ticket_system.completed_tickets.len();
        
        // Calculate average resolution time
        let avg_resolution_time = if completed_tickets > 0 {
            let total_time: i64 = ticket_system.completed_tickets.values()
                .filter_map(|t| {
                    if let TicketStatus::Completed = t.status {
                        Some((Utc::now() - t.created_at).num_hours())
                    } else {
                        None
                    }
                })
                .sum();
            total_time as f64 / completed_tickets as f64
        } else {
            0.0
        };
        
        // Calculate participation rate
        let total_stakeholders = ticket_system.stakeholders.len();
        let active_stakeholders = ticket_system.stakeholders.values()
            .filter(|s| s.last_active > Utc::now() - Duration::days(30))
            .count();
        
        let participation_rate = if total_stakeholders > 0 {
            active_stakeholders as f64 / total_stakeholders as f64
        } else {
            0.0
        };
        
        dashboard.metrics = GovernanceMetrics {
            total_tickets: total_tickets as u32,
            active_tickets: active_tickets as u32,
            completed_tickets: completed_tickets as u32,
            average_resolution_time: avg_resolution_time,
            stakeholder_participation_rate: participation_rate,
            governance_efficiency_score: 0.8, // Would be calculated from various factors
            last_updated: Utc::now(),
        };
        
        Ok(())
    }

    /// Log execution to BPCI VM
    async fn log_vm_execution(&self, action: String, details: String) -> Result<()> {
        let mut vm = self.bpci_vm.write().await;
        
        let audit_entry = AuditEntry {
            timestamp: Utc::now(),
            actor: "InternalGovernanceEngine".to_string(),
            action,
            details: [("details".to_string(), details)]
                .iter().cloned().collect(),
            result: "logged".to_string(),
        };
        
        // Add to the most recent execution record, or create a new one
        if let Some(last_record) = vm.execution_history.last_mut() {
            last_record.audit_log.push(audit_entry);
        } else {
            let new_record = ExecutionRecord {
                id: Uuid::new_v4(),
                policy_id: Uuid::new_v4(),
                executed_at: Utc::now(),
                trigger_reason: "System logging".to_string(),
                actions_taken: vec!["Log entry".to_string()],
                execution_status: ExecutionStatus::Completed,
                resource_usage: ResourceUsage {
                    cpu_time: 0.1,
                    memory_used: 1024,
                    network_io: 0,
                    storage_io: 512,
                    execution_cost: Decimal::from_str_exact("0.01")?,
                },
                audit_log: vec![audit_entry],
            };
            vm.execution_history.push(new_record);
        }
        
        Ok(())
    }

    /// Get governance statistics
    pub async fn get_governance_stats(&self) -> Result<GovernanceDashboard> {
        self.update_dashboard_metrics().await?;
        let dashboard = self.dashboard.read().await;
        Ok(dashboard.clone())
    }

    /// Get distribution history
    pub async fn get_distribution_history(&self) -> Result<Vec<DistributionRecord>> {
        let engine = self.distribution_engine.read().await;
        Ok(engine.distribution_history.clone())
    }

    /// Register new stakeholder
    pub async fn register_stakeholder(
        &self,
        wallet_address: String,
        stakeholder_type: StakeholderType,
        initial_voting_weight: f64,
    ) -> Result<()> {
        let mut system = self.ticket_system.write().await;
        
        let stakeholder = Stakeholder {
            wallet_address: wallet_address.clone(),
            stakeholder_type,
            voting_weight: initial_voting_weight,
            reputation_score: 1.0, // Starting reputation
            participation_history: Vec::new(),
            last_active: Utc::now(),
        };
        
        system.stakeholders.insert(wallet_address, stakeholder);
        Ok(())
    }
}

impl Default for GovernanceDashboard {
    fn default() -> Self {
        Self {
            metrics: GovernanceMetrics {
                total_tickets: 0,
                active_tickets: 0,
                completed_tickets: 0,
                average_resolution_time: 0.0,
                stakeholder_participation_rate: 0.0,
                governance_efficiency_score: 0.0,
                last_updated: Utc::now(),
            },
            voting_stats: VotingStatistics {
                total_votes_cast: 0,
                approval_rate: 0.0,
                average_voting_time: 0.0,
                quorum_achievement_rate: 0.0,
                emergency_votes: 0,
            },
            stakeholder_analytics: StakeholderAnalytics {
                total_stakeholders: 0,
                active_stakeholders: 0,
                average_reputation: 0.0,
                participation_distribution: HashMap::new(),
                top_contributors: Vec::new(),
            },
            performance_reports: Vec::new(),
        }
    }
}
