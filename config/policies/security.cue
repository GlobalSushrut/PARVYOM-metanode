package policies

// Security policies for Metanode platform
// Enforced across all components via CUE validation

#SecurityPolicy: {
	// Minimum security requirements
	min_tls_version:     "1.3"
	require_mTLS:        bool | *true
	quantum_resistant:   bool | *true
	audit_all_requests:  bool | *true
	
	// Cryptographic requirements
	signature_algorithm: "Ed25519" | "ECDSA-P256" | *"Ed25519"
	hash_algorithm:      "SHA256" | "SHA3-256" | "BLAKE3" | *"BLAKE3"
	encryption_standard: "AES-256-GCM" | "ChaCha20-Poly1305" | *"ChaCha20-Poly1305"
	
	// Access control
	require_authentication: bool | *true
	require_authorization:  bool | *true
	session_timeout:        int & >=300 & <=86400 | *3600 // 1 hour default
	
	// Network security
	allow_insecure_connections: bool | *false
	require_certificate_pinning: bool | *true
	enable_hsts:                bool | *true
	
	// Audit and compliance
	log_all_access:          bool | *true
	retain_audit_logs_days:  int & >=30 & <=2555 | *365 // 1 year default
	compliance_frameworks:   [...string] | *["SOC2", "ISO27001", "GDPR"]
}

// Component-specific security policies
http_cage_policy: #SecurityPolicy & {
	// HTTP Cage specific requirements
	intercept_all_traffic:   true
	validate_all_requests:   true
	block_malicious_payloads: true
	rate_limiting:           true
	ddos_protection:         true
}

docklock_policy: #SecurityPolicy & {
	// Container security requirements
	verify_image_signatures:  true
	scan_for_vulnerabilities: true
	enforce_resource_limits:  true
	isolate_containers:       true
	deterministic_execution:  true
}

bpci_policy: #SecurityPolicy & {
	// Blockchain security requirements
	validate_all_transactions: true
	verify_consensus_proofs:   true
	maintain_chain_integrity:  true
	prevent_double_spending:   true
	enforce_gas_limits:        true
}

// Global security enforcement
security_enforcement: {
	// Fail-safe defaults
	default_deny:        true
	explicit_allow_only: true
	
	// Monitoring and alerting
	security_monitoring: true
	threat_detection:    true
	incident_response:   true
	
	// Compliance validation
	continuous_compliance_check: true
	automated_remediation:       true
}
