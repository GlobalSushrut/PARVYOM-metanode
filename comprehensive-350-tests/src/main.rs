// Main entry point for Comprehensive 350-Test Suite
use comprehensive_350_tests::Comprehensive350TestSuite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 METANODE COMPREHENSIVE 350-TEST SUITE");
    println!("🏗️ VALIDATING COMPLETE ARCHITECTURE");
    println!("{}", "=".repeat(80));
    
    let mut suite = Comprehensive350TestSuite::new();
    suite.run_all_350_tests().await?;
    
    println!("\n✅ ALL 350 TESTS COMPLETED!");
    Ok(())
}
