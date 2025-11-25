use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use bpi_core::blockchain_os_kernel::{
    BlockchainOSKernel,
    BlockchainResourceManager,
    ProcessPriority,
    ProcessType,
    OrchestrationMode,
    SecurityLevel as KernelSecurityLevel,
    BpiImmutableOSIntegration,
    LokaType,
};
use bpi_core::agi_digital_nation_storage::{
    AgiDigitalNationStorage, DataQuery, QueryType, SortOrder,
};
use bpi_core::distributed_storage::{
    BpiDistributedStorage, DistributedStorageConfig, CloudProvider,
};
use bpi_core::enhanced_cdn_storage::{
    EnhancedCdnStorage, ContentType as CdnContentType, GeographicLocation,
};
use bpi_core::immutable_audit_system::ImmutableAuditSystem;
use bpi_core::ipfs_plus_plus_engine::{
    IpfsPlusPlusEngine,
    StorageOptions as IpfsStorageOptions,
    StorageTier as IpfsStorageTier,
};
use bpi_core::logbook_6d_bridge::blockchain_writer::{
    SixDBlockchainWriter,
    SixDTransaction,
    DimensionalCoordinates,
    TransactionData,
    CryptographicProofs,
    TransactionType,
};
use bpi_core::os_security_supervisor::OsSecuritySupervisor;
use bpi_core::shadow_registry_bridge::{
    ShadowRegistryBridge,
    Web2ApiEndpoint,
    ApiType,
    AuthenticationType,
    RateLimit,
    SecurityLevel as ShadowSecurityLevel,
};
use bpi_core::four_d_database_bridge::{
    FourDDatabaseBridge,
    BpciEndpointConfig,
    AuthenticationConfig as FourDAuthConfig,
    TimeoutConfig as FourDTimeoutConfig,
    BridgeSecurityConfig as FourDBridgeSecurityConfig,
    SecurityLevel as FourDSecurityLevel,
};
use bpi_core::security::deception_technology::{
    DeceptionEngine,
    DeceptionInteraction,
    DeceptionType,
};
use bpi_core::security::soar_engine::{
    SOAREngine,
    SecurityEvent,
    IncidentType,
    IncidentSeverity,
    ClassificationCondition,
    ClassificationRule,
    Playbook,
    PlaybookStep,
    StepType,
    ActionDefinition,
    ExecutionMethod,
    RetryPolicy,
    BackoffStrategy,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    println!("=== BPI Core: Full Stack Runtime Demo ===");

    let mut report = String::new();
    let run_id = Uuid::new_v4();
    let started_at = Utc::now();

    let progress_log_path = "full_demo_progress.log";
    fs::write(
        progress_log_path,
        format!("0% - demo start run_id={} at {}\n", run_id, started_at.to_rfc3339()),
    )?;

    writeln!(
        report,
        "BPI CORE FULL STACK DEMO REPORT\nRun ID: {}\nStarted at: {}\n",
        run_id,
        started_at.to_rfc3339(),
    )?;

    // ---------------------------------------------------------------------
    // 1. Initialize shared OS security supervisor
    // ---------------------------------------------------------------------
    let audit_root = format!("/tmp/bpi_full_demo_{}", run_id);
    let supervisor = Arc::new(
        OsSecuritySupervisor::new(
            &audit_root,
            "full_stack_demo_profile",
            "demo-node-1",
        )
        .await
        .expect("failed to initialize OsSecuritySupervisor"),
    );

    supervisor.record_kernel_boot_event().await;

    writeln!(report, "[Security Supervisor]\n  audit_root: {}\n", audit_root)?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"10% - security supervisor initialized\n")?;

    // ---------------------------------------------------------------------
    // 2. Distributed storage + Enhanced CDN
    // ---------------------------------------------------------------------
    let storage_config = DistributedStorageConfig {
        min_cloud_providers: 2,
        max_cloud_providers: 4,
        block_size_kb: 1024,
        redundancy_factor: 2,
        instant_backup_threshold_ms: 50,
        vm_audit_required: true,
    };

    let distributed = BpiDistributedStorage::new_with_supervisor(
        storage_config,
        Some(Arc::clone(&supervisor)),
    );

    let cdn = EnhancedCdnStorage::new_with_supervisor(distributed.clone(), Some(Arc::clone(&supervisor)));

    let cdn_data = b"BPI Core full-stack demo: enhanced CDN + distributed storage payload";
    let cdn_metadata = "full_stack_demo_document";

    let cdn_content_id = cdn
        .store_big_data(cdn_data, CdnContentType::Document, cdn_metadata)
        .await
        .expect("failed to store data in EnhancedCdnStorage");

    let user_location = GeographicLocation {
        country: "US".to_string(),
        city: "DemoCity".to_string(),
        latitude: 37.7749,
        longitude: -122.4194,
        provider: CloudProvider::Local,
    };

    let _retrieved = cdn
        .retrieve_with_ultra_fast_cdn(&cdn_content_id, &user_location)
        .await
        .expect("failed to retrieve data from EnhancedCdnStorage");

    let cdn_metrics = cdn
        .get_performance_metrics()
        .await
        .expect("failed to get CDN performance metrics");

    writeln!(report, "[Storage + CDN]")?;
    writeln!(report, "  content_id: {}", cdn_content_id)?;
    writeln!(report, "  edge_nodes_count: {}", cdn_metrics.edge_nodes_count)?;
    writeln!(report, "  cache_hit_rate: {:.3}", cdn_metrics.cache_hit_rate)?;
    writeln!(report, "  average_latency_ms: {}", cdn_metrics.average_latency_ms)?;
    writeln!(report, "  cost_savings_percent: {:.1}", cdn_metrics.cost_savings_percent)?;
    writeln!(report, "  total_content_served: {}", cdn_metrics.total_content_served)?;
    writeln!(report, "  bandwidth_saved_gb: {:.3}\n", cdn_metrics.bandwidth_saved_gb)?;

    let container_block = distributed
        .get_container_block(&cdn_content_id)
        .await
        .expect("failed to inspect distributed storage layout");

    writeln!(report, "[Distributed Storage Layout]")?;
    if let Some(block) = container_block {
        writeln!(report, "  block_id: {}", block.block_id)?;
        writeln!(report, "  data_hash: {}", block.data_hash)?;
        writeln!(report, "  size_bytes: {}", block.size_bytes)?;
        writeln!(report, "  distribution_map_len: {}", block.distribution_map.len())?;
        for loc in &block.distribution_map {
            writeln!(
                report,
                "    - provider={:?} region={} encrypted_path={} backups={}",
                loc.cloud_provider,
                loc.region,
                loc.encrypted_path,
                loc.backup_locations.len(),
            )?;
        }
        writeln!(report)?;
    } else {
        writeln!(report, "  note: container block not found for content_id (in-memory layout only)\n")?;
    }

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"25% - storage + CDN + distributed layout complete\n")?;

    // ---------------------------------------------------------------------
    // 3. AgiDigitalNation storage: citizen record with integrity
    // ---------------------------------------------------------------------
    let agi_storage = AgiDigitalNationStorage::new()
        .await
        .expect("failed to initialize AgiDigitalNationStorage");

    let app_id = Uuid::new_v4();
    let citizen_id = Uuid::new_v4();

    let citizen_record = json!({
        "record_type": "citizen",
        "citizen_id": citizen_id.to_string(),
        "name": "Full Demo Citizen",
        "created_at": Utc::now().to_rfc3339(),
        "attributes": {
            "tier": "founding",
            "region": "demo-region",
            "roles": ["validator", "governance"]
        }
    });

    let store_result = agi_storage
        .store_app_data(app_id, citizen_record)
        .await
        .expect("failed to store citizen record");

    let mut filters = HashMap::new();
    filters.insert("record_type".to_string(), "citizen".to_string());

    let query = DataQuery {
        query_id: Uuid::new_v4(),
        query_type: QueryType::Simple,
        filters,
        sort_order: SortOrder::Temporal,
        limit: Some(1),
        offset: Some(0),
    };

    let retrieval = agi_storage
        .retrieve_app_data(app_id, query)
        .await
        .expect("failed to retrieve citizen record");

    writeln!(report, "[AgiDigitalNation Storage]")?;
    writeln!(report, "  app_id: {}", app_id)?;
    writeln!(report, "  citizen_id: {}", citizen_id)?;
    writeln!(report, "  storage_id: {}", store_result.storage_id)?;
    writeln!(report, "  quantum_storage_id: {}", store_result.quantum_storage_id)?;
    writeln!(report, "  persistence_id: {}", store_result.persistence_id)?;
    writeln!(report, "  success: {}", store_result.success)?;
    writeln!(report, "  verified: {}", retrieval.verification.verified)?;
    writeln!(
        report,
        "  coherence_level: {:.3}",
        retrieval.verification.coherence_level
    )?;
    writeln!(
        report,
        "  entanglement_strength: {:.3}",
        retrieval.verification.entanglement_strength
    )?;
    writeln!(report, "  integrity_hash: {}", retrieval.verification.integrity_hash)?;
    writeln!(report, "  retention_years: {}", retrieval.metadata.retention_policy.retention_years)?;
    writeln!(
        report,
        "  archival_strategy: {:?}\n",
        retrieval.metadata.retention_policy.archival_strategy
    )?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"30% - AgiDigitalNation storage complete\n")?;

    // ---------------------------------------------------------------------
    // 4. Security stack: SOAR engine + Deception (honeyfile)
    // ---------------------------------------------------------------------
    let soar = SOAREngine::new();
    soar.start_soar().await.expect("failed to start SOAR engine");

    // Register playbooks
    let playbook_unauth = Playbook {
        playbook_id: "pb_unauth_access".to_string(),
        playbook_name: "Investigate unauthorized access".to_string(),
        incident_types: vec![IncidentType::UnauthorizedAccess],
        severity_levels: vec![IncidentSeverity::Medium, IncidentSeverity::High],
        steps: vec![PlaybookStep {
            step_id: "step_collect_auth_logs".to_string(),
            step_name: "Collect authentication logs".to_string(),
            step_type: StepType::Investigation,
            action: ActionDefinition {
                action_type: "collect_logs".to_string(),
                target_systems: vec!["auth_service".to_string()],
                parameters: HashMap::new(),
                execution_method: ExecutionMethod::API,
            },
            timeout: chrono::Duration::minutes(5),
            retry_policy: RetryPolicy {
                max_retries: 1,
                backoff_strategy: BackoffStrategy::Fixed,
            },
        }],
        estimated_duration: chrono::Duration::minutes(15),
    };

    soar
        .register_playbook(playbook_unauth)
        .await
        .expect("failed to register SOAR playbook");

    let rule_unauth = ClassificationRule {
        rule_id: "rule_full_demo_unauth".to_string(),
        conditions: vec![
            ClassificationCondition {
                field: "event_type".to_string(),
                operator: "equals".to_string(),
                value: "auth_failure".to_string(),
                weight: 0.7,
            },
            ClassificationCondition {
                field: "resource".to_string(),
                operator: "contains".to_string(),
                value: "/admin".to_string(),
                weight: 0.3,
            },
        ],
        incident_type: IncidentType::UnauthorizedAccess,
        severity: IncidentSeverity::High,
        confidence_score: 0.95,
    };

    soar
        .add_classification_rule(rule_unauth)
        .await
        .expect("failed to register SOAR classification rule");

    // Create one synthetic security event
    let mut attrs = HashMap::new();
    attrs.insert("event_type".to_string(), "auth_failure".to_string());
    attrs.insert("resource".to_string(), "/admin/login".to_string());

    let sec_event = SecurityEvent {
        event_id: "ev_soar_1".to_string(),
        event_type: "login_failure".to_string(),
        timestamp: Utc::now(),
        source: "auth_service".to_string(),
        attributes: attrs,
    };

    let (incident_type, severity, confidence) = soar
        .classify_incident(&sec_event)
        .await
        .expect("SOAR classification failed");

    let playbook_id = soar
        .suggest_playbook(&incident_type, &severity)
        .await;

    if let Some(ref pb_id) = playbook_id {
        let _exec_id = soar
            .execute_playbook(pb_id, &sec_event.event_id)
            .await
            .expect("SOAR playbook execution failed");
    }

    writeln!(report, "[Security + SOAR]")?;
    writeln!(report, "  security_event_id: {}", sec_event.event_id)?;
    writeln!(report, "  incident_type: {:?}", incident_type)?;
    writeln!(report, "  severity: {:?}", severity)?;
    writeln!(report, "  confidence: {:.3}", confidence)?;
    writeln!(
        report,
        "  chosen_playbook: {}",
        playbook_id.unwrap_or_else(|| "<none>".to_string())
    )?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"40% - SOAR + deception flows executed\n")?;

    // Deception: honeyfile trigger
    let deception = DeceptionEngine::new();
    deception
        .start_deception()
        .await
        .expect("failed to start Deception engine");

    let honey_path = "/tmp/bpi_full_demo_honey/confidential_data.xlsx";
    let honey_id = deception
        .create_honeyfile("full_demo_template", honey_path)
        .await
        .expect("failed to create honeyfile");

    let interaction_id = Uuid::new_v4().to_string();
    let mut details = HashMap::new();
    details.insert("file_path".to_string(), honey_path.to_string());
    details.insert("access_type".to_string(), "read".to_string());

    let interaction = DeceptionInteraction {
        interaction_id: interaction_id.clone(),
        deception_type: DeceptionType::Honeyfile,
        target_id: honey_id.clone(),
        timestamp: Utc::now(),
        source_ip: "203.0.113.42".to_string(),
        user_agent: Some("curl/8.0 attacker-probe".to_string()),
        interaction_details: details,
        threat_indicators: vec![
            "suspicious_read".to_string(),
            "high_value_path".to_string(),
        ],
    };

    let alerts = deception
        .analyze_interaction(&interaction)
        .await
        .expect("deception analysis failed");

    writeln!(report, "[Deception]")?;
    writeln!(report, "  honeyfile_id: {}", honey_id)?;
    writeln!(report, "  honeyfile_path: {}", honey_path)?;
    writeln!(report, "  interaction_id: {}", interaction_id)?;
    if let Some(alert) = alerts.get(0) {
        writeln!(report, "  alert_id: {}", alert.alert_id)?;
        writeln!(report, "  severity: {:?}", alert.severity)?;
        writeln!(report, "  description: {}", alert.description)?;
        writeln!(report, "  recommended_actions:")?;
        for act in &alert.recommended_actions {
            writeln!(report, "    - {}", act)?;
        }
    }
    writeln!(report)?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"45% - deception flows executed\n")?;

    // ---------------------------------------------------------------------
    // 5. 6D Blockchain writer (transactions + block)
    // ---------------------------------------------------------------------
    let writer = SixDBlockchainWriter::new()
        .await
        .expect("failed to create SixDBlockchainWriter");
    writer
        .initialize()
        .await
        .expect("failed to initialize SixDBlockchainWriter");

    let tx = SixDTransaction {
        transaction_id: "full_demo_tx_1".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        transaction_type: TransactionType::VMOperation,
        logbook_entry_id: "full_demo_entry_1".to_string(),
        dimensional_coordinates: DimensionalCoordinates {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            t: 4.0,
            s: 0.5,
            q: 0.8,
        },
        transaction_data: TransactionData {
            operation_hash: "op_hash_demo".to_string(),
            input_data_hash: "input_hash_demo".to_string(),
            output_data_hash: "output_hash_demo".to_string(),
            execution_context: "full_stack_demo".to_string(),
            resource_usage: "cpu=10%,mem=128MB".to_string(),
            performance_metrics: "latency_ms=42".to_string(),
            audit_trail: "audit_demo".to_string(),
            compliance_data: "compliance_demo".to_string(),
        },
        cryptographic_proofs: CryptographicProofs {
            merkle_proof: "merkle_demo".to_string(),
            zero_knowledge_proof: "zk_demo".to_string(),
            quantum_proof: "quantum_demo".to_string(),
            consensus_proof: "consensus_demo".to_string(),
            integrity_proof: "integrity_demo".to_string(),
            non_repudiation_proof: "non_repudiation_demo".to_string(),
        },
        poe_tree_root: None,
        traversal_report: None,
        vm_audit_proof: None,
        quantum_signature: "quantum_sig_demo".to_string(),
        integrity_hash: "integrity_hash_demo".to_string(),
    };

    let _tx_id = writer
        .write_transaction(tx)
        .await
        .expect("failed to write 6D transaction");

    let block_hash = writer
        .create_block_from_pending()
        .await
        .expect("failed to create 6D block from pending");

    let chain_state = writer
        .get_blockchain_state()
        .await
        .expect("failed to get blockchain state");
    let writer_stats = writer
        .get_stats()
        .await
        .expect("failed to get writer stats");

    writer.stop().await.expect("failed to stop 6D writer");

    writeln!(report, "[6D Blockchain]")?;
    writeln!(report, "  last_block_hash: {}", chain_state.last_block_hash)?;
    writeln!(report, "  current_block_number: {}", chain_state.current_block_number)?;
    writeln!(report, "  total_transactions: {}", chain_state.total_transactions)?;
    writeln!(report, "  chain_length: {}", chain_state.chain_length)?;
    writeln!(
        report,
        "  writer_total_blocks_created: {}",
        writer_stats.total_blocks_created
    )?;
    writeln!(
        report,
        "  writer_total_transactions_written: {}\n",
        writer_stats.total_transactions_written
    )?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"55% - standalone 6D blockchain writer round complete\n")?;

    // ---------------------------------------------------------------------
    // 6. Immutable OS bridge: endpoint mapping
    // ---------------------------------------------------------------------
    let bridge = BpiImmutableOSIntegration::new()
        .expect("failed to create BpiImmutableOSIntegration");

    bridge
        .initialize()
        .await
        .expect("failed to initialize immutable OS bridge");

    let mappings = bridge
        .get_service_mappings()
        .await
        .expect("failed to get service mappings");
    let fs_state = bridge
        .get_filesystem_state()
        .expect("failed to get filesystem state");
    let net_state = bridge
        .get_network_state()
        .expect("failed to get network state");
    let integration_status = bridge
        .get_integration_status()
        .await
        .expect("failed to get integration status");
    let integration_stats = bridge
        .get_integration_stats()
        .expect("failed to get integration stats");

    writeln!(report, "[Immutable OS Bridge]")?;
    for (name, mapping) in &mappings {
        writeln!(
            report,
            "  - bpi_service={} os_service_type={:?} port={} integration_status={:?} health_status={:?}",
            name,
            mapping.service_type,
            mapping.service_port,
            mapping.integration_status,
            mapping.health_status,
        )?;
    }
    writeln!(report, "  filesystem_namespace_mounted: {}", fs_state.namespace_mounted)?;
    writeln!(report, "  vpod_network_active: {}", net_state.vpod_network_active)?;
    writeln!(report, "  integration_status: {:?}", integration_status)?;
    writeln!(
        report,
        "  total_services_integrated: {}\n",
        integration_stats.total_services_integrated
    )?;

    let audit_metrics = supervisor
        .get_security_audit_metrics()
        .await
        .expect("failed to get security audit metrics");

    writeln!(report, "[Security Audit Metrics]")?;
    writeln!(
        report,
        "  total_security_events: {}",
        audit_metrics.total_security_events
    )?;
    writeln!(
        report,
        "  audit_records_created: {}",
        audit_metrics.audit_records_created
    )?;
    writeln!(
        report,
        "  forensic_evidence_collected: {}",
        audit_metrics.forensic_evidence_collected
    )?;
    writeln!(
        report,
        "  compliance_violations: {}",
        audit_metrics.compliance_violations
    )?;
    writeln!(
        report,
        "  threat_detection_accuracy: {:.3}",
        audit_metrics.threat_detection_accuracy
    )?;
    writeln!(
        report,
        "  false_positive_rate: {:.3}",
        audit_metrics.false_positive_rate
    )?;
    writeln!(report, "  events_by_component:")?;
    for (comp, count) in audit_metrics.events_by_component.iter() {
        writeln!(report, "    - {}: {}", comp, count)?;
    }
    writeln!(report, "  events_by_severity:")?;
    for (sev, count) in audit_metrics.events_by_severity.iter() {
        writeln!(report, "    - {}: {}", sev, count)?;
    }
    writeln!(report)?;

    let ipfs_engine = IpfsPlusPlusEngine::default();

    let web3_payload = json!({
        "component": "ipfs_plus_plus",
        "purpose": "full_stack_demo_web3_payload",
        "citizen_id": citizen_id.to_string(),
        "cdn_content_id": cdn_content_id,
        "tags": ["web2-bridge", "web3.5", "demo"],
    });

    let web3_bytes = web3_payload.to_string().into_bytes();
    let ipfs_options = IpfsStorageOptions {
        tier: IpfsStorageTier::Hot,
        replication_factor: 3,
        encryption_enabled: true,
    };

    let ipfs_address = ipfs_engine
        .store_data(&web3_bytes, &ipfs_options)
        .await
        .expect("failed to store data in IPFS++ engine");

    let html_snippet = format!(
        "<html><body><h1>BPI Web2 → Web3.5 Bridge Demo</h1><p>Citizen: {}</p><p>IPFS++ address: {}</p></body></html>",
        citizen_id,
        ipfs_address,
    );

    let py_client_snippet = r#"import requests
resp = requests.post(
    'https://demo.pyapp.local/api/bpi-bridge',
    json={'citizen_id': '<CITIZEN_ID>', 'ipfs_ref': '<IPFS_ADDRESS>'},
)
print(resp.status_code, resp.text)
"#
    .replace("<CITIZEN_ID>", &citizen_id.to_string())
    .replace("<IPFS_ADDRESS>", &ipfs_address);

    let web2_request = format!(
        "PY_APP_REQUEST /bpi-bridge HTTP/1.1\nUser-Agent: py-demo-client/0.1\nHost: demo.pyapp.local\n\n{}\n\n{}",
        py_client_snippet,
        html_snippet,
    );

    let shadow_audit_root = format!("{}/shadow_registry", audit_root);
    let shadow_audit = ImmutableAuditSystem::new(&shadow_audit_root)
        .await
        .expect("failed to initialize ImmutableAuditSystem for Shadow Registry");

    let shadow_bridge = ShadowRegistryBridge::new(Arc::new(shadow_audit))
        .await
        .expect("failed to initialize ShadowRegistryBridge");

    let web2_endpoint = Web2ApiEndpoint {
        id: "demo_py_app_api".to_string(),
        url: "https://demo.pyapp.local/api/bpi-bridge".to_string(),
        api_type: ApiType::Rest,
        authentication: AuthenticationType::ApiKey,
        rate_limit: RateLimit {
            requests_per_minute: 60,
            burst_size: 10,
            window_size_seconds: 60,
        },
        security_level: ShadowSecurityLevel::High,
        created_at: Utc::now(),
    };

    let bridge_id = shadow_bridge
        .establish_web2_bridge(web2_endpoint)
        .await
        .expect("failed to establish Web2 bridge");

    let processed_web2 = shadow_bridge
        .process_web2_communication(&bridge_id, &web2_request)
        .await
        .expect("failed to process Web2 communication");

    let identity_mapping_id = shadow_bridge
        .manage_cross_platform_identity("py-user-123", &ipfs_address)
        .await
        .expect("failed to manage cross-platform identity");

    let bridge_status = shadow_bridge
        .get_bridge_status()
        .await
        .expect("failed to get Shadow Registry bridge status");

    writeln!(report, "[IPFS++ Storage]")?;
    writeln!(report, "  ipfs_address: {}", ipfs_address)?;
    writeln!(report, "  payload_bytes: {}", web3_bytes.len())?;
    writeln!(report, "  storage_tier: {:?}", ipfs_options.tier)?;
    writeln!(report, "  replication_factor: {}", ipfs_options.replication_factor)?;
    writeln!(report, "  encryption_enabled: {}\n", ipfs_options.encryption_enabled)?;

    writeln!(report, "[Shadow Registry Web2 Bridge]")?;
    writeln!(report, "  bridge_id: {}", bridge_id)?;
    writeln!(report, "  web2_py_client_user: py-user-123")?;
    writeln!(report, "  identity_mapping_id: {}", identity_mapping_id)?;
    writeln!(report, "  active_bridges: {}", bridge_status.active_bridges)?;
    writeln!(report, "  registry_entries: {}", bridge_status.registry_entries)?;
    writeln!(report, "  identity_mappings: {}", bridge_status.identity_mappings)?;
    writeln!(report, "  active_threats: {}", bridge_status.active_threats)?;
    writeln!(report, "  audit_logs: {}", bridge_status.audit_logs)?;
    writeln!(report, "  status: {}", bridge_status.status)?;
    writeln!(report, "  last_updated: {}", bridge_status.last_updated.to_rfc3339())?;
    writeln!(report, "  sample_web2_request_prefix: {}", &web2_request[..web2_request.len().min(120)])?;
    writeln!(report, "  processed_web2_response_prefix: {}\n", &processed_web2[..processed_web2.len().min(120)])?;

    let fourd_config = BpciEndpointConfig {
        base_url: "https://bpci-demo.internal/4d-api".to_string(),
        api_version: "v1".to_string(),
        auth_config: FourDAuthConfig {
            api_key: "demo-api-key".to_string(),
            client_cert_path: None,
            private_key_path: None,
            jwt_token: None,
            token_refresh_interval: 3600,
        },
        timeout_config: FourDTimeoutConfig {
            connection_timeout_ms: 1000,
            request_timeout_ms: 2000,
            keep_alive_timeout_ms: 5000,
        },
        security_config: FourDBridgeSecurityConfig {
            enable_tls: true,
            enable_mtls: false,
            enable_request_signing: false,
            enable_response_validation: true,
            security_level: FourDSecurityLevel::Internal,
        },
    };

    let fourd_bridge = FourDDatabaseBridge::new(fourd_config)
        .await
        .expect("failed to initialize FourDDatabaseBridge");

    let fourd_perf = fourd_bridge
        .get_performance_metrics()
        .await
        .expect("failed to get 4D performance metrics");
    let fourd_health = fourd_bridge
        .health_check()
        .await
        .expect("failed to get 4D health status");

    writeln!(report, "[4D Database Bridge]")?;
    writeln!(
        report,
        "  average_query_time_ms: {:.3}",
        fourd_perf.average_query_time_ms
    )?;
    writeln!(
        report,
        "  queries_per_second: {:.1}",
        fourd_perf.queries_per_second
    )?;
    writeln!(
        report,
        "  cache_hit_ratio: {:.3}",
        fourd_perf.cache_hit_ratio
    )?;
    writeln!(
        report,
        "  connection_pool_utilization: {:.3}",
        fourd_perf.connection_pool_utilization
    )?;
    writeln!(report, "  perf_last_updated: {}", fourd_perf.last_updated.to_rfc3339())?;
    writeln!(report, "  health_status: {}", fourd_health.status)?;
    writeln!(report, "  health_uptime_seconds: {}", fourd_health.uptime_seconds)?;
    writeln!(report, "  health_active_connections: {}", fourd_health.active_connections)?;
    writeln!(report, "  health_last_check: {}\n", fourd_health.last_check.to_rfc3339())?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"70% - IPFS++ + ShadowRegistry + 4D bridge complete\n")?;

    let resource_manager = BlockchainResourceManager::new()
        .expect("failed to create BlockchainResourceManager");
    resource_manager
        .initialize()
        .await
        .expect("failed to initialize BlockchainResourceManager");

    let utilization_before = resource_manager
        .get_detailed_utilization()
        .await
        .expect("failed to get initial resource utilization");

    let allocation = resource_manager
        .allocate_resources("full_demo_vm_1", &ProcessType::VMApplication)
        .await
        .expect("failed to allocate resources for demo VM");

    let utilization_after = resource_manager
        .get_detailed_utilization()
        .await
        .expect("failed to get post-allocation utilization");

    let healthy = resource_manager
        .health_check()
        .await
        .expect("resource manager health check failed");

    resource_manager
        .update_orchestration_mode(&OrchestrationMode::Emergency)
        .await
        .expect("failed to update orchestration mode");

    let utilization_emergency = resource_manager
        .get_detailed_utilization()
        .await
        .expect("failed to get emergency-mode utilization");

    resource_manager
        .release_resources("full_demo_vm_1")
        .await
        .expect("failed to release resources for demo VM");
    resource_manager
        .shutdown()
        .await
        .expect("failed to shutdown BlockchainResourceManager");

    writeln!(report, "[Resource Manager]")?;
    writeln!(report, "  allocation_id: {}", allocation.allocation_id)?;
    writeln!(report, "  process_id: {}", allocation.process_id)?;
    writeln!(report, "  cpu_cores: {}", allocation.cpu_cores)?;
    writeln!(report, "  memory_mb: {}", allocation.memory_mb)?;
    writeln!(report, "  storage_gb: {}", allocation.storage_gb)?;
    writeln!(
        report,
        "  network_bandwidth_mbps: {}",
        allocation.network_bandwidth_mbps
    )?;
    writeln!(
        report,
        "  quantum_access_level: {:?}",
        allocation.quantum_access_level
    )?;
    writeln!(
        report,
        "  utilization_before_overall: {:.3}",
        utilization_before.overall_utilization
    )?;
    writeln!(
        report,
        "  utilization_after_overall: {:.3}",
        utilization_after.overall_utilization
    )?;
    writeln!(
        report,
        "  utilization_emergency_overall: {:.3}",
        utilization_emergency.overall_utilization
    )?;
    writeln!(report, "  health_check_healthy: {}\n", healthy)?;

    let os_kernel = BlockchainOSKernel::new()
        .await
        .expect("failed to initialize BlockchainOSKernel");
    os_kernel
        .boot()
        .await
        .expect("failed to boot BlockchainOSKernel");

    let mesh_node_id = format!("mesh-node-{}", run_id);
    let _poincare = os_kernel
        .poincare_space
        .add_node(mesh_node_id.clone(), LokaType::Bhuloka)
        .await
        .expect("failed to add node to Poincare space");

    let poincare_nodes = os_kernel
        .poincare_space
        .nodes
        .read()
        .unwrap()
        .len();
    let klein_nodes = os_kernel
        .klein_space
        .coordinates
        .read()
        .unwrap()
        .len();

    let kernel_state = os_kernel.kernel_state.read().unwrap().clone();

    writeln!(report, "[Blockchain OS Kernel + Hyperbolic Mesh]")?;
    writeln!(report, "  kernel_id: {}", kernel_state.kernel_id)?;
    writeln!(report, "  orchestration_mode: {:?}", kernel_state.orchestration_mode)?;
    writeln!(report, "  security_level: {:?}", kernel_state.security_level)?;
    writeln!(report, "  total_processes: {}", kernel_state.total_processes)?;
    writeln!(report, "  active_processes: {}", kernel_state.active_processes)?;
    writeln!(report, "  hyperbolic_poincare_nodes: {}", poincare_nodes)?;
    writeln!(report, "  hyperbolic_klein_nodes: {}\n", klein_nodes)?;

    os_kernel
        .shutdown()
        .await
        .expect("failed to shutdown BlockchainOSKernel");

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"85% - Blockchain OS Kernel boot + hyperbolic mesh snapshot complete\n")?;

    // ---------------------------------------------------------------------
    // 7. Final summary
    // ---------------------------------------------------------------------
    let finished_at = Utc::now();
    let duration = finished_at.signed_duration_since(started_at);

    writeln!(report, "[Summary]")?;
    writeln!(report, "  run_id: {}", run_id)?;
    writeln!(report, "  started_at: {}", started_at.to_rfc3339())?;
    writeln!(report, "  finished_at: {}", finished_at.to_rfc3339())?;
    writeln!(report, "  duration_seconds: {}", duration.num_seconds())?;
    writeln!(report, "  components_exercised:")?;
    writeln!(report, "    - OsSecuritySupervisor (kernel boot + storage ops)")?;
    writeln!(report, "    - BpiDistributedStorage + EnhancedCdnStorage")?;
    writeln!(report, "    - AgiDigitalNationStorage (citizen record)")?;
    writeln!(report, "    - SOAREngine (incident classification + playbook)")?;
    writeln!(report, "    - DeceptionEngine (honeyfile trigger)")?;
    writeln!(report, "    - SixDBlockchainWriter (6D transaction + block)")?;
    writeln!(report, "    - BpiImmutableOSIntegration (endpoint mapping)")?;
    writeln!(report, "    - IPFS++ Engine (web3 payload storage)")?;
    writeln!(report, "    - ShadowRegistryBridge (Web2 ↔ Web3.5 identity + audit)")?;
    writeln!(report, "    - FourDDatabaseBridge (4D query performance + health)")?;
    writeln!(report, "  DEMO_STATUS: OK")?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(progress_log_path)?
        .write_all(b"100% - demo completed successfully\n")?;

    fs::write("full_demo_report.txt", &report)?;

    println!("Full demo report written to full_demo_report.txt");
    println!("Demo completed in {} seconds", duration.num_seconds());

    Ok(())
}
