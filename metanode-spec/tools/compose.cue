package tools
import "github.com/metanode/metanode-spec/agreements"

// Generate Docker Compose for Metanode pipeline
out: {
	version: "3.9"
	
	services: {
		// Generate service for each pipeline step
		for s in agreements.agreement.pipeline.steps {
			"\(s.id)": {
				image: s.image
				container_name: "metanode-\(s.id)-\(agreements.agreement.id)"
				
				deploy: {
					resources: {
						limits: {
							memory: s.mem
							cpus: s.cpu
						}
						reservations: {
							memory: s.mem
							cpus: s.cpu
						}
					}
					restart_policy: {
						condition: "on-failure"
						max_attempts: s.retry_count
					}
				}
				
				// Port mapping if ports are specified
				if s.ports != _|_ {
					ports: [ for p in s.ports { "\(p):\(p)" } ]
				}
				
				environment: {
					// Core Metanode environment
					AGREEMENT_ID: agreements.agreement.id
					STEP_ID: s.id
					REQUIRE_ATTEST: "\(agreements.agreement.security.require_attestations)"
					REQUIRE_SIGNATURES: "\(agreements.agreement.security.require_signatures)"
					ENCRYPTION_LEVEL: agreements.agreement.security.encryption_level
					AUDIT_LEVEL: agreements.agreement.security.audit_level
					
					// Performance settings
					TIMEOUT_MS: "\(s.timeout_ms)"
					RETRY_COUNT: "\(s.retry_count)"
					PROOF_REQUIRED: "\(s.proof_required)"
					
					// Storage configuration
					STORAGE_BACKEND: agreements.agreement.storage.backend
					REPLICATION_FACTOR: "\(agreements.agreement.storage.replication_factor)"
					PERFORMANCE_TIER: agreements.agreement.storage.performance_tier
					
					// Economic settings
					PAYMENT_TOKEN: agreements.agreement.terms.payment_token
					MINING_REWARD: "\(agreements.agreement.terms.mining_reward)"
					
					// Court Node integration
					if agreements.agreement.court_node_config != _|_ {
						COURT_NODE_ENABLED: "\(agreements.agreement.court_node_config.yaml_contracts)"
						AUTO_DISPUTE: "\(agreements.agreement.court_node_config.auto_dispute)"
					}
					
					// Bank Mesh integration
					if agreements.agreement.bank_mesh_config != _|_ {
						BANK_MESH_ENABLED: "\(agreements.agreement.bank_mesh_config.autonomous_economy)"
						NOTARY_NODES: "\(agreements.agreement.bank_mesh_config.notary_nodes)"
					}
				}
				
				// Security hardening
				security_opt: [
					"no-new-privileges:true"
				]
				
				read_only: true
				
				tmpfs: [
					"/tmp:rw,noexec,nosuid,size=100m"
				]
				
				// Network restrictions
				networks: ["metanode-secure"]
				
				// Health check
				healthcheck: {
					test: ["CMD", "curl", "-f", "http://localhost:\(s.ports[0] if s.ports != _|_ && len(s.ports) > 0 else 8080)/health"]
					interval: "30s"
					timeout: "10s"
					retries: 3
					start_period: "40s"
				}
				
				// Resource limits and capabilities
				cap_drop: ["ALL"]
				cap_add: ["NET_BIND_SERVICE"] // Only if ports are needed
				
				// Logging configuration
				logging: {
					driver: "json-file"
					options: {
						"max-size": "10m"
						"max-file": "3"
					}
				}
				
				// Labels for management
				labels: {
					"metanode.agreement.id": agreements.agreement.id
					"metanode.step.id": s.id
					"metanode.version": agreements.agreement.version
					"metanode.security.level": agreements.agreement.security.encryption_level
					"metanode.compliance": strings.Join(agreements.agreement.metadata.compliance_frameworks if agreements.agreement.metadata.compliance_frameworks != _|_ else [], ",")
				}
			}
		}
		
		// Add Metanode infrastructure services
		"metanode-relay": {
			image: "ghcr.io/metanode/relay:latest"
			container_name: "metanode-relay-\(agreements.agreement.id)"
			
			environment: {
				AGREEMENT_ID: agreements.agreement.id
				STORAGE_BACKEND: agreements.agreement.storage.backend
				REPLICATION_FACTOR: "\(agreements.agreement.storage.replication_factor)"
				PERFORMANCE_TIER: agreements.agreement.storage.performance_tier
				ENCRYPTION_ENABLED: "\(agreements.agreement.storage.encryption)"
				COMPRESSION_ENABLED: "\(agreements.agreement.storage.compression)"
				DEDUPE_ENABLED: "\(agreements.agreement.storage.dedupe)"
			}
			
			ports: ["8080:8080", "8443:8443"]
			networks: ["metanode-secure"]
			
			volumes: [
				"metanode-relay-data:/data:rw"
			]
			
			healthcheck: {
				test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
				interval: "15s"
				timeout: "5s"
				retries: 3
			}
		}
		
		"metanode-bpci": {
			image: "ghcr.io/metanode/bpci:latest"
			container_name: "metanode-bpci-\(agreements.agreement.id)"
			
			environment: {
				AGREEMENT_ID: agreements.agreement.id
				CHAIN_TYPE: agreements.agreement.onchain.chain
				CONTRACT_NAME: agreements.agreement.onchain.contract_name
				CONSENSUS_REQUIRED: "\(agreements.agreement.onchain.consensus_required)"
				FINALITY_BLOCKS: "\(agreements.agreement.onchain.finality_blocks)"
				PAYMENT_TOKEN: agreements.agreement.terms.payment_token
				AUTONOMOUS_ECONOMY: "\(agreements.agreement.bank_mesh_config.autonomous_economy if agreements.agreement.bank_mesh_config != _|_ else false)"
			}
			
			ports: ["9090:9090", "9443:9443"]
			networks: ["metanode-secure"]
			
			volumes: [
				"metanode-bpci-data:/data:rw"
			]
			
			healthcheck: {
				test: ["CMD", "curl", "-f", "http://localhost:9090/health"]
				interval: "15s"
				timeout: "5s"
				retries: 3
			}
		}
		
		if agreements.agreement.court_node_config != _|_ && agreements.agreement.court_node_config.yaml_contracts {
			"metanode-court": {
				image: "ghcr.io/metanode/court-node:latest"
				container_name: "metanode-court-\(agreements.agreement.id)"
				
				environment: {
					AGREEMENT_ID: agreements.agreement.id
					YAML_CONTRACTS_ENABLED: "true"
					AUTO_DISPUTE: "\(agreements.agreement.court_node_config.auto_dispute)"
					MEDIATION_TIMEOUT: "\(agreements.agreement.court_node_config.mediation_timeout)"
				}
				
				ports: ["7070:7070"]
				networks: ["metanode-secure"]
				
				volumes: [
					"metanode-court-data:/data:rw"
				]
			}
		}
	}
	
	// Secure network configuration
	networks: {
		"metanode-secure": {
			driver: "bridge"
			ipam: {
				config: [{
					subnet: "172.20.0.0/16"
				}]
			}
			options: {
				"com.docker.network.bridge.enable_icc": "false"
				"com.docker.network.bridge.enable_ip_masquerade": "true"
			}
		}
	}
	
	// Persistent volumes
	volumes: {
		"metanode-relay-data": {
			driver: "local"
		}
		"metanode-bpci-data": {
			driver: "local"
		}
		if agreements.agreement.court_node_config != _|_ && agreements.agreement.court_node_config.yaml_contracts {
			"metanode-court-data": {
				driver: "local"
			}
		}
	}
}
