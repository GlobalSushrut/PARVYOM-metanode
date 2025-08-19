package metanode

import "github.com/metanode/config/schemas:metanode"

// Development environment configuration
config: metanode.development & {
	system: {
		version:        "0.1.0"
		metrics_port:   9090
		dashboard_port: 8080
	}
	
	// Relaxed security for development
	http_cage: {
		port:           8080
		tls_cert_path:  null
		tls_key_path:   null
		quantum_crypto: false
	}
	
	// Single node cluster for development
	enc_cluster: {
		node_count: 1
	}
	
	// Development storage settings
	relay_storage: {
		storage_path:       "/tmp/metanode-dev/storage"
		replication_factor: 1
	}
	
	// Development token economics
	bank_mesh: token_economics: {
		base_token:           "DEV_META"
		governance_threshold: 10
	}
}
