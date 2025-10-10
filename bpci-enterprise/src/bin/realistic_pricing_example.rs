use rust_decimal::Decimal;

/// Realistic pricing calculator for BPI/BPCI system
#[derive(Debug, Clone)]
struct RealisticPricingCalculator {
    // Adjusted rates for real-world usage
    poe_base_rate: Decimal,           // Per record
    pow_difficulty_multiplier: Decimal, // Per difficulty unit
    rent_per_hour: Decimal,           // Per hour of runtime
    gas_per_byte: Decimal,            // Per byte of data
}

impl RealisticPricingCalculator {
    fn new() -> Self {
        Self {
            poe_base_rate: Decimal::new(10, 2),      // $0.10 per record
            pow_difficulty_multiplier: Decimal::new(1, 6), // $0.000001 per difficulty unit
            rent_per_hour: Decimal::new(5, 2),       // $0.05 per hour
            gas_per_byte: Decimal::new(1, 6),        // $0.000001 per byte
        }
    }

    fn calculate_charges(&self, records: usize, difficulty: u64, runtime_hours: u32, data_bytes: u64) -> ChargeBreakdown {
        let poe_charge = self.poe_base_rate * Decimal::new(records as i64, 0);
        let pow_charge = self.pow_difficulty_multiplier * Decimal::new(difficulty as i64, 0);
        let rent_charge = self.rent_per_hour * Decimal::new(runtime_hours as i64, 0);
        let gas_charge = self.gas_per_byte * Decimal::new(data_bytes as i64, 0);
        
        let total = poe_charge + pow_charge + rent_charge + gas_charge;

        ChargeBreakdown {
            poe_charge,
            pow_charge,
            rent_charge,
            gas_charge,
            total_charge: total,
        }
    }
}

#[derive(Debug, Clone)]
struct ChargeBreakdown {
    poe_charge: Decimal,
    pow_charge: Decimal,
    rent_charge: Decimal,
    gas_charge: Decimal,
    total_charge: Decimal,
}

fn main() {
    println!("🏷️  REALISTIC BPI/BPCI PRICING EXAMPLES");
    println!("=====================================");
    
    let calculator = RealisticPricingCalculator::new();
    
    // Example 1: Small transaction
    println!("\n📱 SMALL TRANSACTION (Mobile App):");
    let small = calculator.calculate_charges(
        1,      // 1 record
        1000,   // Low difficulty
        1,      // 1 hour runtime
        1024,   // 1KB data
    );
    println!("   Records: 1, Difficulty: 1,000, Runtime: 1h, Data: 1KB");
    println!("   PoE: ${}, PoW: ${}, Rent: ${}, Gas: ${}", 
             small.poe_charge, small.pow_charge, small.rent_charge, small.gas_charge);
    println!("   💰 TOTAL: ${}", small.total_charge);
    
    // Example 2: Medium transaction
    println!("\n💼 MEDIUM TRANSACTION (Business Document):");
    let medium = calculator.calculate_charges(
        10,     // 10 records
        10000,  // Medium difficulty
        8,      // 8 hours runtime
        102400, // 100KB data
    );
    println!("   Records: 10, Difficulty: 10,000, Runtime: 8h, Data: 100KB");
    println!("   PoE: ${}, PoW: ${}, Rent: ${}, Gas: ${}", 
             medium.poe_charge, medium.pow_charge, medium.rent_charge, medium.gas_charge);
    println!("   💰 TOTAL: ${}", medium.total_charge);
    
    // Example 3: Large enterprise transaction
    println!("\n🏢 LARGE ENTERPRISE TRANSACTION:");
    let large = calculator.calculate_charges(
        100,      // 100 records
        100000,   // High difficulty
        24,       // 24 hours runtime
        1048576,  // 1MB data
    );
    println!("   Records: 100, Difficulty: 100,000, Runtime: 24h, Data: 1MB");
    println!("   PoE: ${}, PoW: ${}, Rent: ${}, Gas: ${}", 
             large.poe_charge, large.pow_charge, large.rent_charge, large.gas_charge);
    println!("   💰 TOTAL: ${}", large.total_charge);
    
    // Example 4: Our original test (unrealistic)
    println!("\n❌ ORIGINAL TEST (Unrealistic):");
    let original = RealisticPricingCalculator {
        poe_base_rate: Decimal::new(100, 2),    // $1.00
        pow_difficulty_multiplier: Decimal::new(50, 3), // $0.050
        rent_per_hour: Decimal::new(25, 2),     // $0.25
        gas_per_byte: Decimal::new(1, 4),       // $0.0001
    }.calculate_charges(3, 1000000, 24, 1024000);
    println!("   Records: 3, Difficulty: 1,000,000, Runtime: 24h, Data: 1MB");
    println!("   PoE: ${}, PoW: ${}, Rent: ${}, Gas: ${}", 
             original.poe_charge, original.pow_charge, original.rent_charge, original.gas_charge);
    println!("   💰 TOTAL: ${} (Way too expensive!)", original.total_charge);
    
    println!("\n📊 PRACTICAL MEANING:");
    println!("===================");
    println!("🏠 RENT: Cost of keeping your wallet connected to the blockchain network");
    println!("   - Like paying for server hosting or cloud compute time");
    println!("   - Realistic: $0.05/hour = $1.20/day = $36/month");
    println!("   - Includes: 6D blockchain access, quantum-safe channels, LCCD consensus");
    
    println!("\n⛽ GAS: Cost of processing and storing data on the blockchain");
    println!("   - Like paying for computation and permanent storage");
    println!("   - Realistic: $0.000001/byte = $1.00/MB");
    println!("   - Includes: Cryptographic verification, consensus validation, immutable storage");
    
    println!("\n🔨 PoW: Cost of computational work/mining difficulty");
    println!("   - Like paying for the security and consensus of the network");
    println!("   - Realistic: $0.000001/difficulty unit");
    println!("   - High difficulty = more security but higher cost");
    
    println!("\n✅ CONCLUSION: Rent and Gas are reasonable - PoW difficulty was the expensive part!");
}
