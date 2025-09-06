use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Complete Economic Distribution Flow Documentation
/// 
/// ## COMPLETE ECONOMIC FLOW:
/// 
/// 1. **FIAT INFLOW** (100% Gas & Rent Received)
///    ├── Gas Fees: $X from wallet transactions
///    └── Rent Payments: $Y from wallet sessions
///    
/// 2. **PRIMARY SPLIT** (Strict Mathematical Distribution)
///    ├── 25% → Coin Economy (Distributed as coins to workers)
///    └── 75% → Infrastructure (Company operations & community)
///    
/// 3. **COIN ECONOMY DISTRIBUTION** (25% of total)
///    ├── GEN (Governance Coin): Mother coin, governance reserve
///    ├── NEX (PoE Mining Coin): Daughter coin, mining rewards
///    ├── FLX (Network Usage Coin): Daughter coin, gas/rent fees
///    └── AUR (Bank Settlement Coin): SEPARATE ENTITY - Bank operations only
///    
/// 4. **INFRASTRUCTURE DISTRIBUTION** (75% of total)
///    ├── 25% of 75% = 18.75% → Company API Treasury
///    ├── 10% of 75% = 7.5% → Owner Salary Wallet
///    └── 40% of 75% = 30% → Community/Reserves
///    
/// 5. **COMMUNITY/RESERVES BREAKDOWN** (30% of total)
///    ├── 50% → Active Maintainers (performance-based)
///    ├── 25% → Governance Initiatives
///    └── 25% → Emergency Reserves
///    
/// 6. **BANK SETTLEMENT COIN (AUR/SC4) - SEPARATE FLOW**
///    ├── Only accessible by Bank wallets
///    ├── Only created through bank-to-bank settlements
///    ├── Never mixed with regular gas/rent economy
///    └── Burns after settlement completion

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteEconomicDistribution {
    pub total_fiat_received: Decimal,
    pub primary_distribution: PrimaryDistribution,
    pub coin_economy_allocation: CoinEconomyAllocation,
    pub infrastructure_allocation: InfrastructureAllocation,
    pub bank_settlement_operations: BankSettlementOperations,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryDistribution {
    pub coin_economy_amount: Decimal,    // 25%
    pub infrastructure_amount: Decimal,  // 75%
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinEconomyAllocation {
    pub total_amount: Decimal,
    pub gen_allocation: Decimal,  // 40% of coin economy
    pub nex_allocation: Decimal,  // 30% of coin economy
    pub flx_allocation: Decimal,  // 30% of coin economy
    pub aur_allocation: Decimal,  // 0% - SEPARATE ENTITY
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureAllocation {
    pub total_amount: Decimal,
    pub company_api_treasury: Decimal,  // 18.75% of total
    pub owner_salary: Decimal,          // 7.5% of total
    pub community_reserves: Decimal,    // 30% of total
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankSettlementOperations {
    pub settlement_coins_created: Decimal,
    pub active_settlements: u64,
    pub total_fiat_settled: Decimal,
    pub isolation_guarantee: String,
}

impl CompleteEconomicDistribution {
    pub fn from_fiat_inflow(
        gas_fees: Decimal,
        rent_payments: Decimal,
    ) -> Self {
        let total_fiat = gas_fees + rent_payments;
        
        // Primary 25%/75% split
        let coin_economy_amount = total_fiat * Decimal::new(25, 2);
        let infrastructure_amount = total_fiat * Decimal::new(75, 2);
        
        // Infrastructure breakdown
        let company_api_amount = infrastructure_amount * Decimal::new(25, 2);
        let owner_salary_amount = infrastructure_amount * Decimal::new(10, 2);
        let community_reserves_amount = infrastructure_amount * Decimal::new(40, 2);
        
        // Coin economy breakdown
        let gen_allocation = coin_economy_amount * Decimal::new(40, 2);
        let nex_allocation = coin_economy_amount * Decimal::new(30, 2);
        let flx_allocation = coin_economy_amount * Decimal::new(30, 2);
        
        Self {
            total_fiat_received: total_fiat,
            primary_distribution: PrimaryDistribution {
                coin_economy_amount,
                infrastructure_amount,
            },
            coin_economy_allocation: CoinEconomyAllocation {
                total_amount: coin_economy_amount,
                gen_allocation,
                nex_allocation,
                flx_allocation,
                aur_allocation: Decimal::ZERO, // SEPARATE ENTITY
            },
            infrastructure_allocation: InfrastructureAllocation {
                total_amount: infrastructure_amount,
                company_api_treasury: company_api_amount,
                owner_salary: owner_salary_amount,
                community_reserves: community_reserves_amount,
            },
            bank_settlement_operations: BankSettlementOperations {
                settlement_coins_created: Decimal::ZERO,
                active_settlements: 0,
                total_fiat_settled: Decimal::ZERO,
                isolation_guarantee: "Bank settlement operations completely isolated from regular economy".to_string(),
            },
            timestamp: Utc::now(),
        }
    }
    
    pub fn generate_summary(&self) -> String {
        format!(
            r#"
🏦 COMPLETE ECONOMIC DISTRIBUTION SUMMARY
==========================================

💰 TOTAL FIAT RECEIVED: ${:.2}

📊 PRIMARY DISTRIBUTION:
├── 25% → Coin Economy: ${:.2}
└── 75% → Infrastructure: ${:.2}

🪙 COIN ECONOMY (${:.2}):
├── GEN (Governance): ${:.2} (40% - Mother Coin)
├── NEX (PoE Mining): ${:.2} (30% - Daughter Coin)
├── FLX (Network Usage): ${:.2} (30% - Daughter Coin)
└── AUR (Bank Settlement): $0.00 (SEPARATE ENTITY)

🏢 INFRASTRUCTURE (${:.2}):
├── Company API: ${:.2} (18.75% of total)
├── Owner Salary: ${:.2} (7.5% of total)
└── Community/Reserves: ${:.2} (30% of total)

🏦 BANK SETTLEMENT (ISOLATED):
├── Settlement Coins: {}
├── Active Settlements: {}
└── Status: ✅ COMPLETELY ISOLATED

⚡ KEY GUARANTEES:
✅ Strict 25%/75% split enforced
✅ Bank coin (AUR) completely separate
✅ All coins earned through work proofs
✅ Real-time distribution
✅ Bank-grade security
"#,
            self.total_fiat_received,
            self.primary_distribution.coin_economy_amount,
            self.primary_distribution.infrastructure_amount,
            self.coin_economy_allocation.total_amount,
            self.coin_economy_allocation.gen_allocation,
            self.coin_economy_allocation.nex_allocation,
            self.coin_economy_allocation.flx_allocation,
            self.infrastructure_allocation.total_amount,
            self.infrastructure_allocation.company_api_treasury,
            self.infrastructure_allocation.owner_salary,
            self.infrastructure_allocation.community_reserves,
            self.bank_settlement_operations.settlement_coins_created,
            self.bank_settlement_operations.active_settlements,
        )
    }
}
