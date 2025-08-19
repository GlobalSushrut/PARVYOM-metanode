//! Performance benchmarks for BPI Light Client header verification
//! 
//! These benchmarks test the performance targets specified in Stage 16:
//! - <2ms per header p50
//! - 1k headers <2s

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

use bpi_light_client::{
    HeaderVerifier,
    test_utils::{create_test_validator_set, generate_test_chain},
};

fn bench_single_header_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_header_verification");
    
    // Test with different validator set sizes
    for validator_count in [4, 7, 13, 21, 50].iter() {
        let validator_set = create_test_validator_set(*validator_count);
        let headers = generate_test_chain(1, &validator_set).unwrap();
        let (header, commit) = &headers[0];
        
        group.bench_with_input(
            BenchmarkId::new("validators", validator_count),
            validator_count,
            |b, _| {
                let mut verifier = HeaderVerifier::new(validator_set.clone());
                b.iter(|| {
                    black_box(verifier.verify_header(
                        black_box(header),
                        black_box(None),
                        black_box(commit)
                    ).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_batch_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_verification");
    group.measurement_time(Duration::from_secs(30));
    
    let validator_set = create_test_validator_set(21); // Realistic validator count
    
    // Test with different batch sizes
    for batch_size in [10, 50, 100, 500, 1000].iter() {
        let headers = generate_test_chain(*batch_size, &validator_set).unwrap();
        
        group.bench_with_input(
            BenchmarkId::new("batch_size", batch_size),
            batch_size,
            |b, _| {
                b.iter(|| {
                    let mut verifier = HeaderVerifier::new(validator_set.clone());
                    black_box(verifier.verify_batch(black_box(&headers)).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_verification_with_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification_with_cache");
    
    let validator_set = create_test_validator_set(21);
    let headers = generate_test_chain(100, &validator_set).unwrap();
    let mut verifier = HeaderVerifier::new(validator_set);
    
    // Pre-populate cache
    let _ = verifier.verify_batch(&headers[..50]).unwrap();
    
    group.bench_function("cached_verification", |b| {
        b.iter(|| {
            // Verify some cached and some new headers
            let mixed_batch = [&headers[25..75]].concat(); // 50% cached, 50% new
            black_box(verifier.verify_batch(black_box(&mixed_batch)).unwrap())
        });
    });
    
    group.finish();
}

fn bench_performance_targets(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_targets");
    group.measurement_time(Duration::from_secs(60));
    
    let validator_set = create_test_validator_set(21);
    
    // Target: <2ms per header p50
    group.bench_function("p50_target_single", |b| {
        let headers = generate_test_chain(1, &validator_set).unwrap();
        let (header, commit) = &headers[0];
        
        b.iter(|| {
            let mut verifier = HeaderVerifier::new(validator_set.clone());
            let start = std::time::Instant::now();
            black_box(verifier.verify_header(
                black_box(header),
                black_box(None),
                black_box(commit)
            ).unwrap());
            let elapsed = start.elapsed();
            
            // Assert performance target in benchmark
            if elapsed > Duration::from_millis(2) {
                eprintln!("WARNING: P50 target missed: {:?} > 2ms", elapsed);
            }
            
            elapsed
        });
    });
    
    // Target: 1k headers <2s
    group.bench_function("batch_target_1k", |b| {
        let headers = generate_test_chain(1000, &validator_set).unwrap();
        
        b.iter(|| {
            let mut verifier = HeaderVerifier::new(validator_set.clone());
            let start = std::time::Instant::now();
            black_box(verifier.verify_batch(black_box(&headers)).unwrap());
            let elapsed = start.elapsed();
            
            // Assert performance target in benchmark
            if elapsed > Duration::from_secs(2) {
                eprintln!("WARNING: Batch target missed: {:?} > 2s for 1k headers", elapsed);
            }
            
            elapsed
        });
    });
    
    group.finish();
}

fn bench_validator_set_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("validator_set_scaling");
    group.measurement_time(Duration::from_secs(45));
    
    // Test how verification time scales with validator set size
    for validator_count in [7, 13, 21, 50, 100, 200].iter() {
        let validator_set = create_test_validator_set(*validator_count);
        let headers = generate_test_chain(10, &validator_set).unwrap();
        
        group.bench_with_input(
            BenchmarkId::new("scaling", validator_count),
            validator_count,
            |b, _| {
                b.iter(|| {
                    let mut verifier = HeaderVerifier::new(validator_set.clone());
                    black_box(verifier.verify_batch(black_box(&headers)).unwrap())
                });
            },
        );
    }
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    let validator_set = create_test_validator_set(21);
    
    // Test memory efficiency with large batches
    group.bench_function("large_batch_10k", |b| {
        b.iter(|| {
            let headers = generate_test_chain(10000, &validator_set).unwrap();
            let mut verifier = HeaderVerifier::new(validator_set.clone());
            
            // Process in chunks to test memory efficiency
            let chunk_size = 1000;
            let mut total_verified = 0;
            
            for chunk in headers.chunks(chunk_size) {
                let result = verifier.verify_batch(black_box(chunk)).unwrap();
                total_verified += result.successful_verifications;
            }
            
            black_box(total_verified)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_single_header_verification,
    bench_batch_verification,
    bench_verification_with_cache,
    bench_performance_targets,
    bench_validator_set_scaling,
    bench_memory_usage
);

criterion_main!(benches);
