package metanode

import "github.com/metanode/metanode-spec/schema"

// BPI Escrow Agreement - Following Metanode Core Patterns
agreement: schema.#Agreement & {
	id: "bpi-escrow-2025-08-20-001"
	version: "1.0"
	
	parties: [
		{
			id: "did:bpi:buyer123456789012345678901234567890"
			role: "buyer"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1000.0
			reputation: 85
		},
		{
			id: "did:bpi:seller456789012345678901234567890"
			role: "seller"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1000.0
			reputation: 92
		},
		{
			id: "did:bpi:escrow789012345678901234567890"
			role: "escrow_agent"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 5000.0
			reputation: 98
		},
		{
			id: "did:bpi:notary012345678901234567890"
			role: "notary"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 2000.0
			reputation: 96
		}
	]
	
	terms: {
		sla_ms: 5000  // 5 second SLA for escrow operations
		max_fee: 50.0
		slash_ratio: 0.10
		stake_required: 1000.0
		reward_multiplier: 1.5
		payment_token: "GOLD"  // High-value escrow transactions
		mining_reward: true
	}
	
	pipeline: {
		steps: [
			{
				id: "escrow-initialize"
				image: "ghcr.io/bpi/escrow-init:v1.0"
				cpu: "500m"
				mem: "512Mi"
				ports: [8080]
				attest: true
				proof_required: true
				timeout_ms: 3000
				retry_count: 2
			},
			{
				id: "funds-validation"
				image: "ghcr.io/bpi/funds-validator:v1.0"
				cpu: "1000m"
				mem: "1Gi"
				ports: [8081]
				attest: true
				proof_required: true
				timeout_ms: 5000
				retry_count: 1
			},
			{
				id: "escrow-execution"
				image: "ghcr.io/bpi/escrow-executor:v1.0"
				cpu: "1000m"
				mem: "1Gi"
				ports: [8082]
				attest: true
				proof_required: true
				timeout_ms: 10000
				retry_count: 1
			},
			{
				id: "release-validation"
				image: "ghcr.io/bpi/release-validator:v1.0"
				cpu: "500m"
				mem: "512Mi"
				ports: [8083]
				attest: true
				proof_required: true
				timeout_ms: 3000
				retry_count: 2
			},
			{
				id: "settlement-finalization"
				image: "ghcr.io/bpi/settlement-finalizer:v1.0"
				cpu: "500m"
				mem: "512Mi"
				ports: [8084]
				attest: true
				proof_required: true
				timeout_ms: 5000
				retry_count: 1
			}
		]
		
		edges: [
			{from: "escrow-initialize", to: "funds-validation"},
			{from: "funds-validation", to: "escrow-execution"},
			{from: "escrow-execution", to: "release-validation"},
			{from: "release-validation", to: "settlement-finalization"}
		]
		
		// BPI-specific pipeline configuration
		bpi_config: {
			consensus_required: true
			zk_proof_validation: true
			economic_coordination: true
			court_integration: true
		}
	}
	
	// Onchain settlement configuration
	onchain: {
		network: "bpi-mainnet"
		contract_name: "BpiEscrowSettlement"
		finality_blocks: 12
		gas_limit: 500000
		deployment_cost: 0.1
	}
	
	// Court Node configuration for disputes
	court_node_config: {
		enabled: true
		yaml_contracts: true
		dispute_timeout: 86400  // 24 hours
		arbitration_fee: 25.0
		appeal_window: 172800   // 48 hours
	}
	
	// BPI-specific escrow configuration
	escrow_config: {
		minimum_amount: 0.01
		maximum_amount: 1000000.0
		escrow_fee_percentage: 2.5
		release_timeout_hours: 168  // 7 days
		dispute_timeout_hours: 72   // 3 days
		supported_currencies: ["BPI", "GOLD", "SILVER", "COPPER"]
		
		// Release conditions
		release_conditions: {
			buyer_confirmation: true
			seller_delivery: true
			timeout_automatic: true
			arbiter_decision: true
		}
		
		// Dispute resolution
		dispute_resolution: {
			evidence_submission_hours: 24
			voting_period_hours: 48
			appeal_period_hours: 72
			final_settlement_hours: 24
		}
	}
	
	// Security and compliance
	security: {
		encryption_required: true
		audit_trail: true
		compliance_checks: true
		kyc_required: false  // Optional for BPI
		aml_monitoring: true
		
		// BPI-specific security
		bpi_security: {
			zk_privacy: true
			quantum_resistant: true
			military_grade: true
			cross_chain_validation: true
		}
	}
	
	// Economic incentives
	economics: {
		staking_rewards: true
		performance_bonuses: true
		reputation_system: true
		slashing_penalties: true
		
		// Token distribution
		token_distribution: {
			escrow_agent: 40
			buyer: 25
			seller: 25
			notary: 10
		}
		
		// Fee structure
		fee_structure: {
			base_fee: 1.0
			success_fee: 1.5
			dispute_fee: 5.0
			cancellation_fee: 2.0
		}
	}
}
