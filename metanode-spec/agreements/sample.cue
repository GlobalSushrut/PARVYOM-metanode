package metanode

import "github.com/metanode/metanode-spec/schema"

// Sample Metanode Agreement - Medical Data Processing Pipeline
agreement: schema.#Agreement & {
	id: "metanode-2025-08-13-medical-ai-001"
	version: "1.1"
	
	parties: [
		{
			id: "did:key:z6MkHospitalABC123..."
			role: "hospital"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1000.0
			reputation: 95
		},
		{
			id: "did:key:z6MkDoctorXYZ789..."
			role: "doctor"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 500.0
			reputation: 88
		},
		{
			id: "did:key:z6MkVendorDEF456..."
			role: "vendor"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 750.0
			reputation: 92
		},
		{
			id: "did:key:z6MkNotaryGHI012..."
			role: "notary"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 2000.0
			reputation: 98
		}
	]
	
	terms: {
		sla_ms: 1500  // 1.5 second SLA
		max_fee: 25.5
		slash_ratio: 0.15
		stake_required: 100.0
		reward_multiplier: 1.2
		payment_token: "GOLD"  // High-value medical data
		mining_reward: true
	}
	
	pipeline: {
		steps: [
			{
				id: "data-ingest"
				image: "ghcr.io/metanode/medical-ingest:v1.2"
				cpu: "1000m"
				mem: "1Gi"
				ports: [8080, 8443]
				attest: true
				proof_required: true
				timeout_ms: 5000
				retry_count: 2
			},
			{
				id: "ai-analysis"
				image: "ghcr.io/metanode/medical-ai:v2.1"
				cpu: "2000m"
				mem: "4Gi"
				ports: [9090]
				attest: true
				proof_required: true
				timeout_ms: 15000
				retry_count: 1
			},
			{
				id: "compliance-check"
				image: "ghcr.io/metanode/hipaa-validator:v1.0"
				cpu: "500m"
				mem: "512Mi"
				attest: true
				proof_required: true
				timeout_ms: 3000
				retry_count: 3
			},
			{
				id: "digital-signature"
				image: "ghcr.io/metanode/medical-signer:v1.5"
				cpu: "500m"
				mem: "256Mi"
				ports: [7777]
				attest: true
				proof_required: true
				timeout_ms: 2000
				retry_count: 2
			}
		]
		
		edges: [
			{from: "data-ingest", to: "ai-analysis"},
			{from: "ai-analysis", to: "compliance-check"},
			{from: "compliance-check", to: "digital-signature"}
		]
		
		parallel_limit: 2
		failure_policy: "retry"
	}
	
	onchain: {
		chain: "bpi"
		contract_name: "MedicalAgreementSettle"
		abi_name: "MetanodeAgreement"
		consensus_required: true
		finality_blocks: 6  // Faster finality for medical data
	}
	
	security: {
		allowed_egress_cidrs: ["10.0.0.0/8", "172.16.0.0/12"]  // Private networks only
		allowed_registry: ["ghcr.io"]  // Restricted to our registry
		require_attestations: true
		require_signatures: true
		encryption_level: "military"  // Medical data requires highest security
		audit_level: "military"
		court_validation: true
		slashing_enabled: true
	}
	
	storage: {
		backend: "relay"
		replication_factor: 5  // High availability for medical data
		encryption: true
		compression: true
		dedupe: true
		performance_tier: "extreme"  // Medical data needs fastest access
	}
	
	// Metanode-specific configuration
	court_node_config: {
		yaml_contracts: true
		auto_dispute: false  // Manual review for medical disputes
		mediation_timeout: 43200  // 12 hours for medical disputes
	}
	
	bank_mesh_config: {
		autonomous_economy: true
		notary_nodes: 5  // Extra notaries for medical validation
		economic_validation: true
	}
	
	// Metadata
	created_at: "2025-08-13T05:39:40Z"
	expires_at: "2026-08-13T05:39:40Z"  // 1 year validity
	metadata: {
		use_case: "Medical AI Analysis Pipeline"
		compliance_frameworks: ["HIPAA", "GDPR", "SOC2"]
		data_classification: "PHI"  // Protected Health Information
		geographic_restrictions: ["US", "EU"]
		emergency_contact: "emergency@metanode.io"
	}
}
