package metanode

// Metanode-Specific Core Types (Optimized for 150MB Installer)
#DID: string & !=""  // Simplified validation for size
#Image: string & !=""
#Role: "validator" | "miner" | "court" | "bank" | "user"  // Metanode-specific roles only

// Metanode Core Configuration (Single Source of Truth)
#MetanodeConfig: {
	id: string & !=""
	
	// DockLock Container Configuration
	docklock: {
		containers: [...{
			name: string & !=""
			image: #Image
			cpu: string | *"500m"
			mem: string | *"512Mi"
			ports?: [...int]
		}]
	}
	
	// ENC Cluster Orchestration
	enc_cluster: {
		nodes: int & >0 | *3
		auto_scale: bool | *true
		max_nodes: int & >0 | *10
	}
	
	// BPI Consensus Layer
	bpi: {
		consensus_type: "ibft" | "pos" | *"ibft"
		validators: int & >0 | *5
		block_time: int & >0 | *2  // seconds
	}
	
	// BPCI Enterprise Server
	bpci: {
		port: int & >0 | *8080
		economy_enabled: bool | *true
		banking_enabled: bool | *true
	}
	
	// Court Node (YAML SmartContracts++)
	court: {
		enabled: bool | *true
		yaml_contracts: bool | *true
		dispute_timeout: int & >0 | *86400
	}
	
	// Bank Mesh (Autonomous Economy)
	bank: {
		enabled: bool | *true
		tokens: ["GOLD", "SILVER", "COPPER", "IRON"]
		notary_nodes: int & >0 | *3
	}
	
	// Relay Storage (10x IPFS)
	relay: {
		performance_tier: "high" | "extreme" | *"extreme"
		replication: int & >0 & <=10 | *3
		encryption: bool | *true
	}
	
	// Security (Military-grade)
	security: {
		level: "enterprise" | "military" | *"military"
		attestations: bool | *true
		audit_all: bool | *true
	}
}
