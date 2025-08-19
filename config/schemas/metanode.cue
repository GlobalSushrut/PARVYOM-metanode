package metanode

// Metanode Platform Configuration Schema
// Single source of truth for all system configurations

#MetanodeConfig: {
	// Core system configuration
	system: #SystemConfig
	
	// Component configurations
	http_cage:     #HttpCageConfig
	docklock:      #DocklockConfig
	enc_cluster:   #EncClusterConfig
	bpci:          #BpciConfig
	court_node:    #CourtNodeConfig
	relay_storage: #RelayStorageConfig
	bank_mesh:     #BankMeshConfig
	bpi_consensus: #BpiConsensusConfig
	security_core: #SecurityCoreConfig
}

#SystemConfig: {
	version:        string & =~"^[0-9]+\\.[0-9]+\\.[0-9]+$"
	environment:    "development" | "staging" | "production"
	log_level:      "trace" | "debug" | "info" | "warn" | "error"
	metrics_port:   int & >=1024 & <=65535
	dashboard_port: int & >=1024 & <=65535 & !=metrics_port
	data_dir:       string
}

#HttpCageConfig: {
	enabled:            bool | *true
	port:               int & >=1024 & <=65535 | *8443
	tls_cert_path:      string | *null
	tls_key_path:       string | *null
	audit_enabled:      bool | *true
	split_origin_audit: bool | *true
	quantum_crypto:     bool | *true
}

#DocklockConfig: {
	enabled:                bool | *true
	socket_path:            string | *"/var/run/docklock.sock"
	deterministic_execution: bool | *true
	witness_recording:      bool | *true
	cue_validation:         bool | *true
	receipt_generation:     bool | *true
}

#EncClusterConfig: {
	enabled:             bool | *true
	node_count:          int & >=1 & <=100 | *3
	consensus_scheduler: bool | *true
	p2p_port:           int & >=1024 & <=65535 | *30303
	control_plane_port: int & >=1024 & <=65535 | *6443
	service_mesh:       bool | *true
}

#BpciConfig: {
	enabled:               bool | *true
	rpc_port:             int & >=1024 & <=65535 | *8545
	p2p_port:             int & >=1024 & <=65535 | *30304
	consensus_algorithm:   "IBFT" | "PoA" | "PoS" | *"IBFT"
	cross_chain_bridge:   bool | *true
	enterprise_api:       bool | *true
	compliance_monitoring: bool | *true
}

#CourtNodeConfig: {
	enabled:           bool | *true
	governance_port:   int & >=1024 & <=65535 | *9000
	yaml_contracts:    bool | *true
	dispute_resolution: bool | *true
	voting_mechanism:  "simple" | "quadratic" | "delegated" | *"quadratic"
}

#RelayStorageConfig: {
	enabled:             bool | *true
	storage_path:        string | *"/var/lib/metanode/storage"
	ipfs_compatible:     bool | *true
	multi_tier_caching:  bool | *true
	replication_factor:  int & >=1 & <=10 | *3
}

#BankMeshConfig: {
	enabled:                bool | *true
	economic_engine:        bool | *true
	autonomous_scaling:     bool | *true
	cross_chain_settlement: bool | *true
	token_economics:        #TokenEconomics
}

#TokenEconomics: {
	base_token:           string | *"META"
	staking_rewards:      number & >=0 & <=1 | *0.05
	transaction_fees:     number & >=0 & <=1 | *0.001
	governance_threshold: int & >=1 | *1000
}

#BpiConsensusConfig: {
	enabled:              bool | *true
	consensus_mechanism:  string | *"PoH+VRF+BLS"
	proof_of_history:     bool | *true
	vrf_leader_selection: bool | *true
	bls_aggregation:      bool | *true
	finality_proofs:      bool | *true
}

#SecurityCoreConfig: {
	enabled:                      bool | *true
	quantum_resistant:            bool | *true
	ai_threat_detection:          bool | *true
	multi_jurisdiction_compliance: bool | *true
	audit_trails:                 bool | *true
	security_score_target:        number & >=0 & <=10 | *9.5
}

// Environment-specific configurations
development: #MetanodeConfig & {
	system: {
		environment: "development"
		log_level:   "debug"
		data_dir:    "/tmp/metanode-dev"
	}
	security_core: security_score_target: 8.0
}

staging: #MetanodeConfig & {
	system: {
		environment: "staging"
		log_level:   "info"
		data_dir:    "/var/lib/metanode-staging"
	}
	security_core: security_score_target: 9.0
}

production: #MetanodeConfig & {
	system: {
		environment: "production"
		log_level:   "warn"
		data_dir:    "/var/lib/metanode"
	}
	security_core: security_score_target: 9.5
}
