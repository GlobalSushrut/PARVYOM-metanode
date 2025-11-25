use std::collections::{BTreeMap, BTreeSet};

use bpi_core::distributed_storage::{BpiDistributedStorage, DistributedStorageConfig, CloudProvider};
use bpi_core::enhanced_cdn_storage::{EnhancedCdnStorage, ContentType};
use sha2::{Digest, Sha256};

#[tokio::test(flavor = "current_thread")]
async fn bpi_core_15_multi_cloud_storage_layout_sanity() {
    println!("=== Test: BPI-CORE-15: Multi-cloud storage layout sanity ===");

    // Configure distributed storage with a reasonable provider range
    let storage_config = DistributedStorageConfig {
        min_cloud_providers: 3,
        max_cloud_providers: 6,
        block_size_kb: 1024,
        redundancy_factor: 2,
        instant_backup_threshold_ms: 5_000,
        vm_audit_required: true,
    };

    let base_storage = BpiDistributedStorage::new(storage_config.clone());
    let cdn_storage = EnhancedCdnStorage::new(base_storage.clone());

    // Synthetic bundle payload
    let payload = b"BPI-CORE-15 multi-cloud storage layout test payload - verifying hash, size and provider map";
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let payload_hash = format!("{:x}", hasher.finalize());

    println!("bundle_bytes_len: {}", payload.len());
    println!("bundle_sha256: {}", payload_hash);

    // Store payload via EnhancedCdnStorage on top of distributed storage
    let content_id = cdn_storage
        .store_big_data(payload, ContentType::Document, "bpi_core_15_multi_cloud_test_bundle")
        .await
        .expect("failed to store bundle in multi-cloud storage");

    println!("content_id: {}", content_id);

    // Inspect underlying container block to see multi-cloud layout
    let block_opt = base_storage
        .get_container_block(&content_id)
        .await
        .expect("failed to query container block by ID");

    let block = block_opt.expect("expected container block for stored content_id");

    println!("container_block:");
    println!("  block_id: {}", block.block_id);
    println!("  size_bytes: {}", block.size_bytes);
    println!("  data_hash: {}", block.data_hash);
    println!("  proof_hash: {}", block.proof_hash);
    println!("  created_at: {}", block.created_at);
    println!("  vm_signature: {}", block.vm_signature);
    println!("  distribution_map_len: {}", block.distribution_map.len());

    let mut provider_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut provider_set: BTreeSet<&'static str> = BTreeSet::new();

    for loc in &block.distribution_map {
        let provider_str: &'static str = match loc.cloud_provider {
            CloudProvider::AWS => "AWS",
            CloudProvider::GCP => "GCP",
            CloudProvider::Azure => "Azure",
            CloudProvider::DigitalOcean => "DigitalOcean",
            CloudProvider::Linode => "Linode",
            CloudProvider::Vultr => "Vultr",
            CloudProvider::Hetzner => "Hetzner",
            CloudProvider::OVH => "OVH",
            CloudProvider::Cloudflare => "Cloudflare",
            CloudProvider::Local => "Local",
        };

        *provider_counts.entry(provider_str).or_insert(0) += 1;
        provider_set.insert(provider_str);

        println!(
            "  - location_id={} provider={} region={} encrypted_path={} backup_locations={} verification_hash={}",
            loc.location_id,
            provider_str,
            loc.region,
            loc.encrypted_path,
            loc.backup_locations.len(),
            loc.verification_hash,
        );
    }

    println!("provider_distribution:");
    for (provider, count) in &provider_counts {
        println!("  - provider={} count={}", provider, count);
    }

    // Retrieve data back through base storage to ensure integrity pipeline works
    let retrieved = base_storage
        .retrieve_data(&content_id)
        .await
        .expect("failed to retrieve bundle from distributed storage");

    println!("retrieved_bytes_len: {}", retrieved.len());

    // Invariants
    // 1. Stored size must be positive and no larger than original payload
    assert!(
        block.size_bytes > 0,
        "container block size_bytes must be positive",
    );
    assert!(
        block.size_bytes as usize <= payload.len(),
        "container block size_bytes should not exceed original payload size",
    );

    // 2. Distribution map should honor configured provider range and use multiple distinct providers
    assert!(
        block.distribution_map.len() >= storage_config.min_cloud_providers
            && block.distribution_map.len() <= storage_config.max_cloud_providers,
        "distribution_map length {} must be between min={} and max={}",
        block.distribution_map.len(),
        storage_config.min_cloud_providers,
        storage_config.max_cloud_providers,
    );
    assert!(
        provider_set.len() >= 2,
        "expected data to be spread across at least 2 distinct cloud providers, found {}",
        provider_set.len(),
    );

    // 3. Every storage location must reference the same data hash as the container block
    for loc in &block.distribution_map {
        assert_eq!(
            loc.verification_hash, block.data_hash,
            "location {} verification_hash must match container block data_hash",
            loc.location_id,
        );
    }

    // 4. Retrieved data must pass integrity verification in the pipeline
    assert!(
        !retrieved.is_empty(),
        "retrieved data must not be empty",
    );

    println!("status: OK");
}
