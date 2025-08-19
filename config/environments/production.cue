package metanode

import "github.com/metanode/config/schemas:metanode"

// Production environment configuration
config: metanode.production & {
	system: {
		version:        "1.0.0"
		metrics_port:   9090
		dashboard_port: 8080
	}
	
	// Maximum security for production
	http_cage: {
		port:               8443
		tls_cert_path:      "/etc/metanode/tls/server.crt"
		tls_key_path:       "/etc/metanode/tls/server.key"
		audit_enabled:      true
		split_origin_audit: true
		quantum_crypto:     true
	}
	
	// Full cluster for production
	enc_cluster: {
		node_count:          5
		consensus_scheduler: true
		service_mesh:        true
	}
	
	// Production storage with high availability
	relay_storage: {
		storage_path:       "/var/lib/metanode/storage"
		ipfs_compatible:    true
		multi_tier_caching: true
		replication_factor: 5
	}
	
	// Production token economics
	bank_mesh: token_economics: {
		base_token:           "META"
		staking_rewards:      0.05
		transaction_fees:     0.001
		governance_threshold: 10000
	}
	
	// Maximum security settings
	security_core: {
		quantum_resistant:            true
		ai_threat_detection:          true
		multi_jurisdiction_compliance: true
		audit_trails:                 true
		security_score_target:        9.5
	}
}
