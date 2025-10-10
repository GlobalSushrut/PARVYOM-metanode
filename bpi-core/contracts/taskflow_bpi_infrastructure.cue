package metanode

import "github.com/metanode/metanode-spec/schema"

// Web3 Task Manager Infrastructure Agreement - BPI Token Based
agreement: schema.#Agreement & {
	id: "taskflow-bpi-infra-2025-10-06-002"
	version: "1.1"
	
	parties: [
		{
			id: "did:bpci:taskflow:global:001"
			role: "application_provider"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1500.0
			reputation: 95
		},
		{
			id: "did:bpi:system:firewall"
			role: "firewall_provider"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1500.0
			reputation: 98
		},
		{
			id: "did:bpi:system:storage"
			role: "storage_provider"
			pubkeyPem: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----"
			stake: 1500.0
			reputation: 97
		}
	]
	
	terms: {
		sla_ms: 1000  // 1 second SLA for task operations
		max_fee: 5.0
		slash_ratio: 0.05
		stake_required: 1500.0  // Testnet allocation: 1500 BPI tokens
		reward_multiplier: 1.5
		payment_token: "BPI"  // Use available BPI tokens
		mining_reward: true
	}

	// Infrastructure Configuration - Simplified for Testing
	infrastructure: {
		// Firewall Configuration
		firewall: {
			enabled: true
			type: "ai_powered_forensic_firewall"
			
			// TaskFlow protection rules
			rules: [
				{
					name: "taskflow_app_protection"
					source: "httpcg://app/taskflow.global"
					action: "allow"
					audit: true
				},
				{
					name: "api_protection"
					source: "httpcg://app/taskflow.global/api/*"
					action: "allow"
					rate_limit: "100/minute"
				}
			]
		}

		// Storage Configuration
		storage: {
			enabled: true
			type: "distributed_container_block"
			replication_factor: 2
			encryption: "aes_256"
			
			// TaskFlow storage policies
			policies: [
				{
					name: "task_data"
					path: "/taskflow/tasks/*"
					replication: 2
					audit: true
				}
			]
		}

		// Pipeline Configuration
		pipeline: {
			enabled: true
			type: "pravyom_standard_pipeline"
			
			// Simplified VM configuration
			vms: {
				"vmapp01": {
					type: "VM-APP"
					description: "TaskFlow app execution"
					resources: {
						cpu: "1 core"
						memory: "2GB"
					}
				}
			}
		}
	}

	// Service Level Agreements
	sla: {
		availability: "99%"
		response_time_ms: 1000
		throughput_tps: 100
		
		// TaskFlow operations
		task_operations: {
			create_task_ms: 500
			update_task_ms: 300
			verify_blockchain_ms: 2000
		}
	}

	// Economic Model - BPI Token Based
	economics: {
		// Pricing in BPI tokens
		pricing: {
			task_creation: 0.01   // 0.01 BPI tokens
			task_update: 0.005   // 0.005 BPI tokens
			blockchain_verification: 0.02  // 0.02 BPI tokens
		}
		
		// Revenue sharing
		revenue_sharing: {
			application_provider: 0.5  // 50%
			infrastructure_providers: 0.4  // 40%
			bpi_network: 0.1  // 10%
		}
	}

	// Deployment Configuration
	deployment: {
		auto_deploy: true
		
		// Infrastructure provisioning order
		provisioning_order: [
			"firewall",
			"storage", 
			"pipeline",
			"application"
		]
		
		// Health checks
		health_checks: {
			interval: "60s"
			timeout: "30s"
		}
	}
}

// Smart Contract Logic
smartcontract: {
	// Contract initialization
	init: {
		provision_infrastructure: true
		register_domain: "httpcg://app/taskflow.global"
		enable_audit: true
	}
	
	// Contract methods
	methods: {
		// Deploy infrastructure component
		deploy_component: {
			input: {
				component_type: string
				configuration: object
			}
			output: {
				deployment_id: string
				status: string
			}
		}
		
		// Get infrastructure status
		get_status: {
			output: {
				firewall_status: string
				storage_status: string
				pipeline_status: string
				overall_health: string
			}
		}
		
		// Process task operation
		process_task: {
			input: {
				operation: string
				task_data: object
			}
			output: {
				result: string
				blockchain_hash: string
				cost_bpi: number
			}
		}
	}
	
	// Events
	events: {
		InfrastructureDeployed: {
			component: string
			timestamp: string
		}
		
		TaskProcessed: {
			task_id: string
			operation: string
			cost_bpi: number
			timestamp: string
		}
	}
}
