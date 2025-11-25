use clap::{Args, Subcommand};
use serde_json;
use anyhow::Result;
use rust_decimal::Decimal;

use crate::autonomous_economy::mother_coin_distribution::MotherCoinDistributionEngine;

/// Mother Coin (GEN) Distribution CLI Commands
/// Target: Raise $1M safely while maintaining decentralization
#[derive(Debug, Args)]
pub struct MotherCoinArgs {
    #[command(subcommand)]
    pub command: MotherCoinCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MotherCoinCommands {
    /// Initialize mother coin distribution system
    Init,
    /// Register community installer (25k GEN @ $10/coin)
    RegisterCommunity {
        /// Wallet address
        #[arg(long)]
        wallet: String,
        /// Country code (ISO)
        #[arg(long)]
        country: String,
        /// Node set proof (5 node IDs)
        #[arg(long, value_delimiter = ',')]
        nodes: Vec<String>,
    },
    /// Register early investor (25k GEN @ $30/coin)
    RegisterInvestor {
        /// Wallet address
        #[arg(long)]
        wallet: String,
        /// Investment amount in USD
        #[arg(long)]
        amount: f64,
        /// Country code (ISO)
        #[arg(long)]
        country: String,
        /// KYC proof hash
        #[arg(long)]
        kyc_proof: String,
    },
    /// Get distribution status
    Status,
    /// Get fundraising progress
    Fundraising,
    /// Get decentralization metrics
    Decentralization,
    /// Activate next fundraising phase
    ActivatePhase,
    /// Generate distribution report
    Report,
    /// Simulate distribution scenarios
    Simulate {
        /// Number of community participants
        #[arg(long, default_value = "1000")]
        community_count: u32,
        /// Number of investors
        #[arg(long, default_value = "100")]
        investor_count: u32,
        /// Average investment amount
        #[arg(long, default_value = "10000")]
        avg_investment: f64,
    },
}

pub async fn handle_mother_coin_command(
    args: MotherCoinArgs,
    json_output: bool,
) -> Result<()> {
    let mut engine = MotherCoinDistributionEngine::new();

    match args.command {
        MotherCoinCommands::Init => {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&engine.get_distribution_status())?);
            } else {
                println!("🪙 Mother Coin Distribution System Initialized");
                println!("📊 Total Supply: 100,000 GEN");
                println!("💰 Fundraising Target: $1,000,000");
                println!("🏘️  Community Phase: 25,000 GEN @ $10/coin = $250,000");
                println!("💼 Investment Phase: 25,000 GEN @ $30/coin = $750,000");
                println!("🔒 Decentralization: Max $50k per wallet");
                println!("✅ System ready for community registration!");
            }
        },

        MotherCoinCommands::RegisterCommunity { wallet, country, nodes } => {
            if nodes.len() != 5 {
                return Err(anyhow::anyhow!("Community installer requires exactly 5 node IDs"));
            }

            match engine.register_community_installer(wallet.clone(), country, nodes) {
                Ok(allocation) => {
                    if json_output {
                        println!("{}", serde_json::to_string_pretty(&allocation)?);
                    } else {
                        println!("✅ Community Installer Registered!");
                        println!("👤 Wallet: {}", wallet);
                        println!("🪙 GEN Allocated: {} coins", allocation.gen_amount);
                        println!("💰 Investment: ${}", allocation.investment_amount);
                        println!("📅 Vesting: {} months ({}m cliff)", 
                            allocation.vesting.total_months, 
                            allocation.vesting.cliff_months);
                        println!("🗳️  Governance Weight: {}", allocation.governance_weight);
                    }
                },
                Err(e) => {
                    if json_output {
                        println!("{}", serde_json::json!({"error": e.to_string()}));
                    } else {
                        println!("❌ Registration Failed: {}", e);
                    }
                    return Err(e.into());
                }
            }
        },

        MotherCoinCommands::RegisterInvestor { wallet, amount, country, kyc_proof } => {
            let investment_amount = Decimal::try_from(amount)?;
            
            match engine.register_early_investor(wallet.clone(), investment_amount, country, kyc_proof) {
                Ok(allocation) => {
                    if json_output {
                        println!("{}", serde_json::to_string_pretty(&allocation)?);
                    } else {
                        println!("✅ Early Investor Registered!");
                        println!("👤 Wallet: {}", wallet);
                        println!("🪙 GEN Allocated: {} coins", allocation.gen_amount);
                        println!("💰 Investment: ${}", allocation.investment_amount);
                        println!("📅 Vesting: {} months ({}m cliff)", 
                            allocation.vesting.total_months, 
                            allocation.vesting.cliff_months);
                        println!("🗳️  Governance Weight: {}", allocation.governance_weight);
                    }
                },
                Err(e) => {
                    if json_output {
                        println!("{}", serde_json::json!({"error": e.to_string()}));
                    } else {
                        println!("❌ Investment Failed: {}", e);
                    }
                    return Err(e.into());
                }
            }
        },

        MotherCoinCommands::Status => {
            let status = engine.get_distribution_status();
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(status)?);
            } else {
                println!("🪙 Mother Coin Distribution Status");
                println!("═══════════════════════════════════");
                println!("📊 Total Supply: {} GEN", status.total_supply);
                
                for (tier, state) in &status.distribution_tiers {
                    let completion = (state.distributed as f64 / state.total_allocation as f64) * 100.0;
                    println!("\n{:?}:", tier);
                    println!("  📈 Progress: {}/{} ({:.1}%)", 
                        state.distributed, state.total_allocation, completion);
                    println!("  💰 Price: ${}/coin", state.price_per_coin);
                    println!("  🎯 Target: ${}", state.target_amount);
                    println!("  💵 Raised: ${}", state.raised_amount);
                    println!("  🔄 Active: {}", if state.active { "✅" } else { "❌" });
                }
            }
        },

        MotherCoinCommands::Fundraising => {
            let fundraising = engine.get_fundraising_progress();
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(fundraising)?);
            } else {
                println!("💰 Fundraising Progress");
                println!("═══════════════════════");
                let completion = (fundraising.total_raised / fundraising.total_target * Decimal::from(100)).to_string().parse::<f64>().unwrap_or(0.0);
                println!("🎯 Target: ${}", fundraising.total_target);
                println!("💵 Raised: ${} ({:.1}%)", fundraising.total_raised, completion);
                println!("📊 Current Phase: {}", fundraising.current_phase + 1);
                
                println!("\n📋 Phases:");
                for (i, phase) in fundraising.phases.iter().enumerate() {
                    let phase_completion = (phase.raised / phase.target * Decimal::from(100)).to_string().parse::<f64>().unwrap_or(0.0);
                    let status_icon = match phase.status {
                        crate::autonomous_economy::mother_coin_distribution::PhaseStatus::Active => "🔄",
                        crate::autonomous_economy::mother_coin_distribution::PhaseStatus::Completed => "✅",
                        crate::autonomous_economy::mother_coin_distribution::PhaseStatus::Planned => "📋",
                        _ => "❌",
                    };
                    println!("  {} Phase {}: {} ({:.1}%)", 
                        status_icon, i + 1, phase.name, phase_completion);
                    println!("    💰 ${}/{} @ ${}/GEN", phase.raised, phase.target, phase.price);
                }

                println!("\n🛡️  Safety Mechanisms:");
                println!("  💰 Max per wallet: ${}", fundraising.safety_mechanisms.max_investment_per_wallet);
                println!("  🏘️  Min community: {:.0}%", fundraising.safety_mechanisms.min_community_participation * 100.0);
                println!("  🔒 Escrow: {}", if fundraising.safety_mechanisms.escrow_enabled { "✅" } else { "❌" });
                println!("  📋 KYC Required: {}", if fundraising.safety_mechanisms.kyc_required { "✅" } else { "❌" });
            }
        },

        MotherCoinCommands::Decentralization => {
            let metrics = engine.get_decentralization_metrics();
            
            if json_output {
                println!("{}", serde_json::to_string_pretty(metrics)?);
            } else {
                println!("🌐 Decentralization Metrics");
                println!("══════════════════════════");
                println!("📊 Gini Coefficient: {:.3} (0=perfect equality)", metrics.gini_coefficient);
                println!("👥 Unique Holders: {}", metrics.unique_holders);
                println!("🐋 Largest Holder: {:.2}%", metrics.largest_holder_percentage);
                println!("🔟 Top 10 Holders: {:.2}%", metrics.top_10_percentage);
                println!("🏘️  Community Ratio: {:.1}%", metrics.community_ratio * 100.0);
                
                if metrics.gini_coefficient < 0.3 {
                    println!("✅ Excellent decentralization!");
                } else if metrics.gini_coefficient < 0.5 {
                    println!("⚠️  Moderate centralization");
                } else {
                    println!("❌ High centralization risk");
                }
            }
        },

        MotherCoinCommands::ActivatePhase => {
            match engine.activate_next_phase() {
                Ok(_) => {
                    if json_output {
                        println!("{}", serde_json::json!({"success": true, "message": "Next phase activated"}));
                    } else {
                        println!("✅ Next fundraising phase activated!");
                        println!("🔄 Early investor phase is now open");
                    }
                },
                Err(e) => {
                    if json_output {
                        println!("{}", serde_json::json!({"error": e.to_string()}));
                    } else {
                        println!("❌ Phase activation failed: {}", e);
                    }
                    return Err(e.into());
                }
            }
        },

        MotherCoinCommands::Report => {
            let status = engine.get_distribution_status();
            let fundraising = engine.get_fundraising_progress();
            let metrics = engine.get_decentralization_metrics();
            
            if json_output {
                let report = serde_json::json!({
                    "distribution": status,
                    "fundraising": fundraising,
                    "decentralization": metrics,
                    "timestamp": chrono::Utc::now()
                });
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("📊 Mother Coin Distribution Report");
                println!("═══════════════════════════════════");
                println!("📅 Generated: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
                
                // Summary
                let total_distributed: u64 = status.distribution_tiers.values().map(|t| t.distributed).sum();
                let total_raised: Decimal = status.distribution_tiers.values().map(|t| t.raised_amount).sum();
                
                println!("\n📈 Summary:");
                println!("  🪙 GEN Distributed: {}/{} ({:.1}%)", 
                    total_distributed, status.total_supply,
                    (total_distributed as f64 / status.total_supply as f64) * 100.0);
                println!("  💰 Funds Raised: ${}/{} ({:.1}%)", 
                    total_raised, fundraising.total_target,
                    (total_raised / fundraising.total_target * Decimal::from(100)).to_string().parse::<f64>().unwrap_or(0.0));
                println!("  🌐 Decentralization: {:.3} Gini", metrics.gini_coefficient);
                println!("  👥 Participants: {}", metrics.unique_holders);
                
                // Risk Assessment
                println!("\n🛡️  Risk Assessment:");
                if metrics.gini_coefficient < 0.3 && metrics.largest_holder_percentage < 10.0 {
                    println!("  ✅ Low centralization risk");
                } else if metrics.largest_holder_percentage > 20.0 {
                    println!("  ⚠️  High centralization risk");
                } else {
                    println!("  🟡 Moderate centralization risk");
                }
                
                if fundraising.total_raised > Decimal::from(500_000) {
                    println!("  ✅ Fundraising on track");
                } else {
                    println!("  🟡 Fundraising needs acceleration");
                }
            }
        },

        MotherCoinCommands::Simulate { community_count, investor_count, avg_investment } => {
            if json_output {
                let simulation = serde_json::json!({
                    "community_participants": community_count,
                    "investor_participants": investor_count,
                    "avg_investment": avg_investment,
                    "projected_community_raise": community_count * 10, // $10 per community
                    "projected_investor_raise": (investor_count as f64) * avg_investment,
                    "total_projected_raise": (community_count * 10) as f64 + (investor_count as f64) * avg_investment,
                    "gen_distribution": {
                        "community": community_count,
                        "investors": ((investor_count as f64) * avg_investment / 30.0) as u32, // $30 per GEN
                    }
                });
                println!("{}", serde_json::to_string_pretty(&simulation)?);
            } else {
                println!("🎯 Distribution Simulation");
                println!("═════════════════════════");
                println!("👥 Community Participants: {}", community_count);
                println!("💼 Investor Participants: {}", investor_count);
                println!("💰 Average Investment: ${}", avg_investment);
                
                let community_raise = community_count * 10; // $10 per community installer
                let investor_raise = (investor_count as f64) * avg_investment;
                let total_raise = community_raise as f64 + investor_raise;
                
                println!("\n📊 Projected Results:");
                println!("  🏘️  Community Raise: ${}", community_raise);
                println!("  💼 Investor Raise: ${}", investor_raise);
                println!("  💰 Total Raise: ${}", total_raise);
                
                let community_gen = community_count; // 1 GEN per installer
                let investor_gen = (investor_raise / 30.0) as u32; // $30 per GEN
                
                println!("\n🪙 GEN Distribution:");
                println!("  🏘️  Community GEN: {}", community_gen);
                println!("  💼 Investor GEN: {}", investor_gen);
                println!("  📊 Total GEN: {}", community_gen + investor_gen);
                
                if total_raise >= 1_000_000.0 {
                    println!("\n✅ Simulation meets $1M fundraising target!");
                } else {
                    println!("\n⚠️  Simulation falls short of $1M target by ${}", 1_000_000.0 - total_raise);
                }
            }
        },
    }

    Ok(())
}
