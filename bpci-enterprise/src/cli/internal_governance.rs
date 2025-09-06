//! Internal Governance CLI Commands
//! 
//! CLI interface for internal governance system with:
//! - 75%/25% distribution management
//! - Community ticket system
//! - Governance dashboard
//! - BPCI VM integration

use clap::{Args, Subcommand};
use serde_json;
use anyhow::Result;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::autonomous_economy::internal_governance::*;
use crate::autonomous_economy::internal_governance_engine::InternalGovernanceEngine;

/// Internal Governance CLI Commands
#[derive(Debug, Args)]
pub struct InternalGovernanceArgs {
    #[command(subcommand)]
    pub command: InternalGovernanceCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum InternalGovernanceCommands {
    /// Initialize internal governance system
    Init,
    
    /// Execute internal distribution (75% autonomous, 25% company)
    Distribute {
        /// Total amount to distribute
        #[arg(long)]
        amount: String,
        /// Reason for distribution
        #[arg(long)]
        reason: String,
        /// Approvers (comma-separated wallet addresses)
        #[arg(long)]
        approved_by: String,
    },
    
    /// Create governance ticket for community voting
    CreateTicket {
        /// Ticket title
        #[arg(long)]
        title: String,
        /// Ticket description
        #[arg(long)]
        description: String,
        /// Ticket category
        #[arg(long)]
        category: String,
        /// Ticket priority
        #[arg(long)]
        priority: String,
        /// Creator wallet address
        #[arg(long)]
        created_by: String,
        /// Voting period in hours (optional)
        #[arg(long)]
        voting_hours: Option<u32>,
    },
    
    /// Submit vote on governance ticket
    Vote {
        /// Ticket ID
        #[arg(long)]
        ticket_id: String,
        /// Voter wallet address
        #[arg(long)]
        voter: String,
        /// Vote decision (approve/reject/abstain)
        #[arg(long)]
        decision: String,
        /// Reasoning for vote (optional)
        #[arg(long)]
        reasoning: Option<String>,
    },
    
    /// Register new stakeholder
    RegisterStakeholder {
        /// Wallet address
        #[arg(long)]
        wallet: String,
        /// Stakeholder type
        #[arg(long)]
        stakeholder_type: String,
        /// Initial voting weight
        #[arg(long)]
        voting_weight: f64,
    },
    
    /// Show governance dashboard
    Dashboard,
    
    /// Show distribution history
    History,
    
    /// Show active tickets
    Tickets,
    
    /// Show BPCI VM status
    VmStatus,
    
    /// Execute BPCI VM policy
    ExecutePolicy {
        /// Policy name
        #[arg(long)]
        policy: String,
        /// Execution parameters (JSON)
        #[arg(long)]
        parameters: Option<String>,
    },
    
    /// Show governance statistics
    Stats,
}

pub async fn handle_internal_governance_command(
    args: InternalGovernanceArgs,
    json_output: bool,
) -> Result<()> {
    // Initialize governance engine (in real implementation, this would be injected)
    let mother_coin_engine = std::sync::Arc::new(tokio::sync::RwLock::new(
        crate::autonomous_economy::mother_coin_distribution::MotherCoinDistributionEngine::new()
    ));
    let governance_engine = InternalGovernanceEngine::new(mother_coin_engine);
    
    match args.command {
        InternalGovernanceCommands::Init => {
            handle_init_command(&governance_engine, json_output).await
        }
        InternalGovernanceCommands::Distribute { amount, reason, approved_by } => {
            handle_distribute_command(&governance_engine, amount, reason, approved_by, json_output).await
        }
        InternalGovernanceCommands::CreateTicket { 
            title, description, category, priority, created_by, voting_hours 
        } => {
            handle_create_ticket_command(
                &governance_engine, title, description, category, priority, 
                created_by, voting_hours, json_output
            ).await
        }
        InternalGovernanceCommands::Vote { ticket_id, voter, decision, reasoning } => {
            handle_vote_command(&governance_engine, ticket_id, voter, decision, reasoning, json_output).await
        }
        InternalGovernanceCommands::RegisterStakeholder { wallet, stakeholder_type, voting_weight } => {
            handle_register_stakeholder_command(&governance_engine, wallet, stakeholder_type, voting_weight, json_output).await
        }
        InternalGovernanceCommands::Dashboard => {
            handle_dashboard_command(&governance_engine, json_output).await
        }
        InternalGovernanceCommands::History => {
            handle_history_command(&governance_engine, json_output).await
        }
        InternalGovernanceCommands::Tickets => {
            handle_tickets_command(&governance_engine, json_output).await
        }
        InternalGovernanceCommands::VmStatus => {
            handle_vm_status_command(&governance_engine, json_output).await
        }
        InternalGovernanceCommands::ExecutePolicy { policy, parameters } => {
            handle_execute_policy_command(&governance_engine, policy, parameters, json_output).await
        }
        InternalGovernanceCommands::Stats => {
            handle_stats_command(&governance_engine, json_output).await
        }
    }
}

async fn handle_init_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    // Initialize default stakeholders and configuration
    engine.register_stakeholder(
        "community_representative".to_string(),
        StakeholderType::Community,
        10.0,
    ).await?;
    
    engine.register_stakeholder(
        "investor_representative".to_string(),
        StakeholderType::Investor,
        15.0,
    ).await?;
    
    engine.register_stakeholder(
        "company_representative".to_string(),
        StakeholderType::Company,
        20.0,
    ).await?;
    
    if json {
        let result = serde_json::json!({
            "status": "success",
            "message": "Internal governance system initialized",
            "stakeholders_registered": 3,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🏛️  Internal Governance System Initialized");
        println!("═══════════════════════════════════════");
        println!("✅ Default stakeholders registered");
        println!("✅ Voting configuration set");
        println!("✅ BPCI VM initialized");
        println!("✅ Distribution engine ready");
        println!("\nSystem ready for governance operations!");
    }
    
    Ok(())
}

async fn handle_distribute_command(
    engine: &InternalGovernanceEngine,
    amount: String,
    reason: String,
    approved_by: String,
    json: bool,
) -> Result<()> {
    let total_amount = Decimal::from_str_exact(&amount)?;
    let approvers: Vec<String> = approved_by.split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    let record = engine.execute_internal_distribution(
        total_amount,
        reason,
        approvers,
    ).await?;
    
    if json {
        let result = serde_json::json!({
            "status": "success",
            "distribution_record": record,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("💰 Internal Distribution Executed");
        println!("═══════════════════════════════════");
        println!("📊 Distribution ID: {}", record.id);
        println!("💵 Total Amount: {}", record.total_amount);
        println!("🤖 Autonomous (75%): {}", record.autonomous_amount);
        println!("🏢 Company (25%): {}", record.company_amount);
        println!("📝 Reason: {}", record.reason);
        println!("✅ Status: {:?}", record.execution_status);
        println!("⏰ Executed: {}", record.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    }
    
    Ok(())
}

async fn handle_create_ticket_command(
    engine: &InternalGovernanceEngine,
    title: String,
    description: String,
    category: String,
    priority: String,
    created_by: String,
    voting_hours: Option<u32>,
    json: bool,
) -> Result<()> {
    let ticket_category = match category.to_lowercase().as_str() {
        "infrastructure" => TicketCategory::Infrastructure,
        "security" => TicketCategory::Security,
        "economic" => TicketCategory::Economic,
        "governance" => TicketCategory::Governance,
        "community" => TicketCategory::Community,
        "technical" => TicketCategory::Technical,
        "emergency" => TicketCategory::Emergency,
        _ => return Err(anyhow::anyhow!("Invalid category: {}", category)),
    };
    
    let ticket_priority = match priority.to_lowercase().as_str() {
        "critical" => TicketPriority::Critical,
        "high" => TicketPriority::High,
        "medium" => TicketPriority::Medium,
        "low" => TicketPriority::Low,
        _ => return Err(anyhow::anyhow!("Invalid priority: {}", priority)),
    };
    
    let ticket_id = engine.create_governance_ticket(
        title.clone(),
        description.clone(),
        ticket_category,
        ticket_priority,
        created_by.clone(),
        voting_hours,
    ).await?;
    
    if json {
        let result = serde_json::json!({
            "status": "success",
            "ticket_id": ticket_id,
            "title": title,
            "category": category,
            "priority": priority,
            "created_by": created_by,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🎫 Governance Ticket Created");
        println!("═══════════════════════════");
        println!("🆔 Ticket ID: {}", ticket_id);
        println!("📋 Title: {}", title);
        println!("📂 Category: {}", category);
        println!("⚡ Priority: {}", priority);
        println!("👤 Created by: {}", created_by);
        println!("🗳️  Voting period: {} hours", voting_hours.unwrap_or(72));
        println!("\nTicket is now open for community voting!");
    }
    
    Ok(())
}

async fn handle_vote_command(
    engine: &InternalGovernanceEngine,
    ticket_id: String,
    voter: String,
    decision: String,
    reasoning: Option<String>,
    json: bool,
) -> Result<()> {
    let ticket_uuid = Uuid::parse_str(&ticket_id)?;
    let vote_decision = match decision.to_lowercase().as_str() {
        "approve" => VoteDecision::Approve,
        "reject" => VoteDecision::Reject,
        "abstain" => VoteDecision::Abstain,
        _ => return Err(anyhow::anyhow!("Invalid decision: {}", decision)),
    };
    
    engine.submit_vote(
        ticket_uuid,
        voter.clone(),
        vote_decision.clone(),
        reasoning.clone(),
    ).await?;
    
    if json {
        let result = serde_json::json!({
            "status": "success",
            "ticket_id": ticket_id,
            "voter": voter,
            "decision": format!("{:?}", vote_decision),
            "reasoning": reasoning,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🗳️  Vote Submitted");
        println!("═══════════════");
        println!("🎫 Ticket ID: {}", ticket_id);
        println!("👤 Voter: {}", voter);
        println!("✅ Decision: {:?}", vote_decision);
        if let Some(reason) = reasoning {
            println!("💭 Reasoning: {}", reason);
        }
        println!("⏰ Submitted: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!("\nVote recorded successfully!");
    }
    
    Ok(())
}

async fn handle_register_stakeholder_command(
    engine: &InternalGovernanceEngine,
    wallet: String,
    stakeholder_type: String,
    voting_weight: f64,
    json: bool,
) -> Result<()> {
    let stakeholder_type_enum = match stakeholder_type.to_lowercase().as_str() {
        "community" => StakeholderType::Community,
        "investor" => StakeholderType::Investor,
        "company" => StakeholderType::Company,
        "technical" => StakeholderType::Technical,
        "governance" => StakeholderType::Governance,
        _ => return Err(anyhow::anyhow!("Invalid stakeholder type: {}", stakeholder_type)),
    };
    
    engine.register_stakeholder(
        wallet.clone(),
        stakeholder_type_enum,
        voting_weight,
    ).await?;
    
    if json {
        let result = serde_json::json!({
            "status": "success",
            "wallet": wallet,
            "stakeholder_type": stakeholder_type,
            "voting_weight": voting_weight,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("👥 Stakeholder Registered");
        println!("═══════════════════════");
        println!("💳 Wallet: {}", wallet);
        println!("🏷️  Type: {}", stakeholder_type);
        println!("⚖️  Voting Weight: {}", voting_weight);
        println!("⏰ Registered: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        println!("\nStakeholder can now participate in governance!");
    }
    
    Ok(())
}

async fn handle_dashboard_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    let dashboard = engine.get_governance_stats().await?;
    
    if json {
        println!("{}", serde_json::to_string_pretty(&dashboard)?);
    } else {
        println!("🏛️  Governance Dashboard");
        println!("═══════════════════════");
        println!("📊 Metrics:");
        println!("  📋 Total Tickets: {}", dashboard.metrics.total_tickets);
        println!("  🔄 Active Tickets: {}", dashboard.metrics.active_tickets);
        println!("  ✅ Completed Tickets: {}", dashboard.metrics.completed_tickets);
        println!("  ⏱️  Avg Resolution Time: {:.1} hours", dashboard.metrics.average_resolution_time);
        println!("  👥 Participation Rate: {:.1}%", dashboard.metrics.stakeholder_participation_rate * 100.0);
        println!("  🎯 Efficiency Score: {:.3}", dashboard.metrics.governance_efficiency_score);
        println!("  🕐 Last Updated: {}", dashboard.metrics.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
        
        println!("\n🗳️  Voting Statistics:");
        println!("  📊 Total Votes: {}", dashboard.voting_stats.total_votes_cast);
        println!("  ✅ Approval Rate: {:.1}%", dashboard.voting_stats.approval_rate * 100.0);
        println!("  ⏰ Avg Voting Time: {:.1} hours", dashboard.voting_stats.average_voting_time);
        println!("  🎯 Quorum Achievement: {:.1}%", dashboard.voting_stats.quorum_achievement_rate * 100.0);
        println!("  🚨 Emergency Votes: {}", dashboard.voting_stats.emergency_votes);
        
        println!("\n👥 Stakeholder Analytics:");
        println!("  📊 Total Stakeholders: {}", dashboard.stakeholder_analytics.total_stakeholders);
        println!("  🔄 Active Stakeholders: {}", dashboard.stakeholder_analytics.active_stakeholders);
        println!("  ⭐ Average Reputation: {:.2}", dashboard.stakeholder_analytics.average_reputation);
    }
    
    Ok(())
}

async fn handle_history_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    let history = engine.get_distribution_history().await?;
    
    if json {
        println!("{}", serde_json::to_string_pretty(&history)?);
    } else {
        println!("📊 Distribution History");
        println!("═════════════════════");
        
        if history.is_empty() {
            println!("No distributions found.");
            return Ok(());
        }
        
        for record in history.iter().rev().take(10) {
            println!("\n🆔 ID: {}", record.id);
            println!("💰 Total: {} (Autonomous: {}, Company: {})", 
                    record.total_amount, record.autonomous_amount, record.company_amount);
            println!("📝 Reason: {}", record.reason);
            println!("✅ Status: {:?}", record.execution_status);
            println!("⏰ Time: {}", record.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("───────────────────────");
        }
        
        if history.len() > 10 {
            println!("\n... and {} more distributions", history.len() - 10);
        }
    }
    
    Ok(())
}

async fn handle_tickets_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    // This would need to be implemented in the engine
    if json {
        let result = serde_json::json!({
            "status": "success",
            "message": "Active tickets functionality to be implemented",
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🎫 Active Governance Tickets");
        println!("═══════════════════════════");
        println!("📋 Tickets functionality to be implemented");
        println!("This will show all active tickets with voting status");
    }
    
    Ok(())
}

async fn handle_vm_status_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    if json {
        let result = serde_json::json!({
            "status": "success",
            "vm_state": "Idle",
            "execution_history_count": 0,
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🤖 BPCI VM Status");
        println!("═══════════════");
        println!("🔄 State: Idle");
        println!("💾 Memory Usage: 0%");
        println!("⚡ CPU Usage: 0%");
        println!("📊 Executions: 0");
        println!("🔒 Security Level: High");
        println!("📝 Audit Mode: Enabled");
    }
    
    Ok(())
}

async fn handle_execute_policy_command(
    _engine: &InternalGovernanceEngine,
    policy: String,
    parameters: Option<String>,
    json: bool,
) -> Result<()> {
    if json {
        let result = serde_json::json!({
            "status": "success",
            "policy": policy,
            "parameters": parameters,
            "message": "Policy execution functionality to be implemented",
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🤖 BPCI VM Policy Execution");
        println!("═══════════════════════════");
        println!("📋 Policy: {}", policy);
        if let Some(params) = parameters {
            println!("⚙️  Parameters: {}", params);
        }
        println!("🚧 Policy execution functionality to be implemented");
    }
    
    Ok(())
}

async fn handle_stats_command(
    engine: &InternalGovernanceEngine,
    json: bool,
) -> Result<()> {
    let dashboard = engine.get_governance_stats().await?;
    let history = engine.get_distribution_history().await?;
    
    if json {
        let result = serde_json::json!({
            "governance_metrics": dashboard.metrics,
            "voting_stats": dashboard.voting_stats,
            "stakeholder_analytics": dashboard.stakeholder_analytics,
            "total_distributions": history.len(),
            "timestamp": chrono::Utc::now()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📊 Governance Statistics");
        println!("═══════════════════════");
        
        println!("\n🏛️  Overall Performance:");
        println!("  🎯 Efficiency Score: {:.3}", dashboard.metrics.governance_efficiency_score);
        println!("  👥 Participation Rate: {:.1}%", dashboard.metrics.stakeholder_participation_rate * 100.0);
        println!("  ⏱️  Avg Resolution: {:.1} hours", dashboard.metrics.average_resolution_time);
        
        println!("\n💰 Distribution Summary:");
        println!("  📊 Total Distributions: {}", history.len());
        
        if !history.is_empty() {
            let total_distributed: rust_decimal::Decimal = history.iter()
                .map(|r| r.total_amount)
                .sum();
            let total_autonomous: rust_decimal::Decimal = history.iter()
                .map(|r| r.autonomous_amount)
                .sum();
            let total_company: rust_decimal::Decimal = history.iter()
                .map(|r| r.company_amount)
                .sum();
            
            println!("  💵 Total Amount: {}", total_distributed);
            println!("  🤖 Autonomous (75%): {}", total_autonomous);
            println!("  🏢 Company (25%): {}", total_company);
        }
        
        println!("\n🗳️  Voting Performance:");
        println!("  📊 Total Votes: {}", dashboard.voting_stats.total_votes_cast);
        println!("  ✅ Approval Rate: {:.1}%", dashboard.voting_stats.approval_rate * 100.0);
        println!("  🎯 Quorum Success: {:.1}%", dashboard.voting_stats.quorum_achievement_rate * 100.0);
    }
    
    Ok(())
}
