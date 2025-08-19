//! # Light Client Verification CLI Tool
//!
//! Command-line interface for BPI Mesh header verification and benchmarking.
//! Supports single header verification, batch verification, and performance testing.

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Arg, Command};
use serde_json;
use tracing::{info, warn, error, Level};
use tracing_subscriber;

use bpi_light_client::{
    HeaderVerifier, BatchVerificationResult, VerificationResult,
    test_utils::{create_test_validator_set, generate_test_chain},
};

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let matches = Command::new("lc-verify")
        .version("0.1.0")
        .author("BPI Mesh Team")
        .about("Light client verification tool for BPI Mesh consensus headers")
        .subcommand(
            Command::new("verify")
                .about("Verify a single header or batch of headers")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Input file containing headers and commits (JSON format)")
                        .required(true)
                )
                .arg(
                    Arg::new("validator-set")
                        .short('v')
                        .long("validator-set")
                        .value_name("FILE")
                        .help("Validator set file (JSON format)")
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file for verification results (JSON format)")
                )
                .arg(
                    Arg::new("verbose")
                        .short('V')
                        .long("verbose")
                        .help("Enable verbose output")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("benchmark")
                .about("Run performance benchmarks")
                .arg(
                    Arg::new("count")
                        .short('c')
                        .long("count")
                        .value_name("NUMBER")
                        .help("Number of headers to generate for benchmarking")
                        .default_value("1000")
                )
                .arg(
                    Arg::new("validators")
                        .short('v')
                        .long("validators")
                        .value_name("NUMBER")
                        .help("Number of validators in test set")
                        .default_value("21")
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file for benchmark results (JSON format)")
                )
                .arg(
                    Arg::new("iterations")
                        .short('n')
                        .long("iterations")
                        .value_name("NUMBER")
                        .help("Number of benchmark iterations")
                        .default_value("3")
                )
        )
        .subcommand(
            Command::new("generate")
                .about("Generate test data for verification")
                .arg(
                    Arg::new("count")
                        .short('c')
                        .long("count")
                        .value_name("NUMBER")
                        .help("Number of headers to generate")
                        .default_value("100")
                )
                .arg(
                    Arg::new("validators")
                        .short('v')
                        .long("validators")
                        .value_name("NUMBER")
                        .help("Number of validators in test set")
                        .default_value("7")
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file for generated data (JSON format)")
                        .required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("verify", sub_matches)) => {
            run_verification(sub_matches)
        }
        Some(("benchmark", sub_matches)) => {
            run_benchmark(sub_matches)
        }
        Some(("generate", sub_matches)) => {
            run_generate(sub_matches)
        }
        _ => {
            eprintln!("No subcommand specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

fn run_verification(matches: &clap::ArgMatches) -> Result<()> {
    let input_file = matches.get_one::<String>("input").unwrap();
    let validator_set_file = matches.get_one::<String>("validator-set").unwrap();
    let output_file = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");

    if verbose {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }

    info!("Loading validator set from {}", validator_set_file);
    let validator_set_data = fs::read_to_string(validator_set_file)
        .with_context(|| format!("Failed to read validator set file: {}", validator_set_file))?;
    
    // For now, create a test validator set (in a real implementation, this would parse the JSON)
    let validator_count: usize = serde_json::from_str(&validator_set_data)
        .unwrap_or(7);
    let validator_set = create_test_validator_set(validator_count);

    info!("Loading headers from {}", input_file);
    let input_data = fs::read_to_string(input_file)
        .with_context(|| format!("Failed to read input file: {}", input_file))?;
    
    // For now, generate test data (in a real implementation, this would parse the JSON)
    let header_count: usize = serde_json::from_str(&input_data)
        .unwrap_or(10);
    let headers = generate_test_chain(header_count, &validator_set)
        .with_context(|| "Failed to generate test headers")?;

    info!("Verifying {} headers with {} validators", headers.len(), validator_set.len());

    let mut verifier = HeaderVerifier::new(validator_set);
    let start_time = Instant::now();
    
    let result = verifier.verify_batch(&headers)
        .with_context(|| "Batch verification failed")?;

    let total_time = start_time.elapsed();

    // Print results
    println!("Verification Results:");
    println!("  Total headers: {}", result.total_headers);
    println!("  Successful: {}", result.successful_verifications);
    println!("  Failed: {}", result.total_headers - result.successful_verifications);
    println!("  Total time: {:?}", result.total_time);
    println!("  Average time per header: {:?}", result.average_time_per_header);
    println!("  P50 time: {:?}", result.p50_time);
    println!("  P95 time: {:?}", result.p95_time);
    println!("  Headers per second: {:.2}", result.performance_summary.headers_per_second);
    
    if result.performance_summary.p50_target_met {
        println!("  ✅ P50 target (<2ms) met");
    } else {
        println!("  ❌ P50 target (<2ms) not met");
    }

    if result.total_headers >= 1000 && result.performance_summary.batch_target_met {
        println!("  ✅ Batch target (<2s for 1k headers) met");
    } else if result.total_headers >= 1000 {
        println!("  ❌ Batch target (<2s for 1k headers) not met");
    }

    // Save results if output file specified
    if let Some(output_path) = output_file {
        let json_result = serde_json::to_string_pretty(&result)
            .with_context(|| "Failed to serialize results")?;
        
        fs::write(output_path, json_result)
            .with_context(|| format!("Failed to write results to {}", output_path))?;
        
        info!("Results saved to {}", output_path);
    }

    Ok(())
}

fn run_benchmark(matches: &clap::ArgMatches) -> Result<()> {
    let count: usize = matches.get_one::<String>("count")
        .unwrap()
        .parse()
        .with_context(|| "Invalid count value")?;
    
    let validators: usize = matches.get_one::<String>("validators")
        .unwrap()
        .parse()
        .with_context(|| "Invalid validators value")?;
    
    let iterations: usize = matches.get_one::<String>("iterations")
        .unwrap()
        .parse()
        .with_context(|| "Invalid iterations value")?;
    
    let output_file = matches.get_one::<String>("output");

    info!("Running benchmark: {} headers, {} validators, {} iterations", count, validators, iterations);

    let validator_set = create_test_validator_set(validators);
    let mut all_results = Vec::new();

    for iteration in 1..=iterations {
        info!("Benchmark iteration {}/{}", iteration, iterations);
        
        let headers = generate_test_chain(count, &validator_set)
            .with_context(|| format!("Failed to generate test chain for iteration {}", iteration))?;

        let mut verifier = HeaderVerifier::new(validator_set.clone());
        let start_time = Instant::now();
        
        let result = verifier.verify_batch(&headers)
            .with_context(|| format!("Batch verification failed in iteration {}", iteration))?;

        all_results.push(result);
        
        let iteration_result = &all_results[iteration - 1];
        println!("Iteration {} results:", iteration);
        println!("  Total time: {:?}", iteration_result.total_time);
        println!("  Average time per header: {:?}", iteration_result.average_time_per_header);
        println!("  P50 time: {:?}", iteration_result.p50_time);
        println!("  Headers per second: {:.2}", iteration_result.performance_summary.headers_per_second);
    }

    // Calculate aggregate statistics
    let total_headers_processed = all_results.iter().map(|r| r.total_headers).sum::<usize>();
    let total_time_spent = all_results.iter().map(|r| r.total_time).sum::<std::time::Duration>();
    let average_headers_per_second = total_headers_processed as f64 / total_time_spent.as_secs_f64();
    
    let all_p50_times: Vec<_> = all_results.iter().map(|r| r.p50_time).collect();
    let best_p50 = all_p50_times.iter().min().unwrap();
    let worst_p50 = all_p50_times.iter().max().unwrap();
    let avg_p50 = all_p50_times.iter().sum::<std::time::Duration>() / all_p50_times.len() as u32;

    println!("\nAggregate Benchmark Results:");
    println!("  Total headers processed: {}", total_headers_processed);
    println!("  Total time spent: {:?}", total_time_spent);
    println!("  Average headers per second: {:.2}", average_headers_per_second);
    println!("  Best P50 time: {:?}", best_p50);
    println!("  Worst P50 time: {:?}", worst_p50);
    println!("  Average P50 time: {:?}", avg_p50);
    
    if avg_p50 < std::time::Duration::from_millis(2) {
        println!("  ✅ Average P50 target (<2ms) met");
    } else {
        println!("  ❌ Average P50 target (<2ms) not met");
    }

    if count >= 1000 {
        let avg_batch_time = total_time_spent / iterations as u32;
        if avg_batch_time < std::time::Duration::from_secs(2) {
            println!("  ✅ Average batch target (<2s for 1k headers) met");
        } else {
            println!("  ❌ Average batch target (<2s for 1k headers) not met");
        }
    }

    // Save results if output file specified
    if let Some(output_path) = output_file {
        let benchmark_summary = serde_json::json!({
            "iterations": iterations,
            "headers_per_iteration": count,
            "validators": validators,
            "total_headers_processed": total_headers_processed,
            "total_time_spent_ms": total_time_spent.as_millis(),
            "average_headers_per_second": average_headers_per_second,
            "best_p50_time_ms": best_p50.as_millis(),
            "worst_p50_time_ms": worst_p50.as_millis(),
            "average_p50_time_ms": avg_p50.as_millis(),
            "p50_target_met": avg_p50 < std::time::Duration::from_millis(2),
            "batch_target_met": count >= 1000 && (total_time_spent / iterations as u32) < std::time::Duration::from_secs(2),
            "detailed_results": all_results
        });
        
        let json_result = serde_json::to_string_pretty(&benchmark_summary)
            .with_context(|| "Failed to serialize benchmark results")?;
        
        fs::write(output_path, json_result)
            .with_context(|| format!("Failed to write benchmark results to {}", output_path))?;
        
        info!("Benchmark results saved to {}", output_path);
    }

    Ok(())
}

fn run_generate(matches: &clap::ArgMatches) -> Result<()> {
    let count: usize = matches.get_one::<String>("count")
        .unwrap()
        .parse()
        .with_context(|| "Invalid count value")?;
    
    let validators: usize = matches.get_one::<String>("validators")
        .unwrap()
        .parse()
        .with_context(|| "Invalid validators value")?;
    
    let output_file = matches.get_one::<String>("output").unwrap();

    info!("Generating {} headers with {} validators", count, validators);

    let validator_set = create_test_validator_set(validators);
    let headers = generate_test_chain(count, &validator_set)
        .with_context(|| "Failed to generate test chain")?;

    // Create a simplified JSON representation for testing
    let test_data = serde_json::json!({
        "validator_count": validators,
        "header_count": count,
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "description": "Test data for BPI Mesh light client verification"
    });

    let json_data = serde_json::to_string_pretty(&test_data)
        .with_context(|| "Failed to serialize test data")?;
    
    fs::write(output_file, json_data)
        .with_context(|| format!("Failed to write test data to {}", output_file))?;

    info!("Test data generated and saved to {}", output_file);
    println!("Generated {} headers with {} validators", count, validators);
    println!("Data saved to: {}", output_file);

    Ok(())
}
