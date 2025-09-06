package bpi_core

// Gateway Configuration for BPI Core - API Gateway and Service Mesh
gateway: {
	// Gateway system configuration
	system: {
		type: "bpi_api_gateway"
		version: "1.0"
		architecture: "service_mesh"
		performance_grade: "enterprise"
	}

	// API gateway settings
	api_gateway: {
		enabled: true
		protocols: ["http", "https", "grpc", "websocket", "bpi_native"]
		rate_limiting: true
		load_balancing: "intelligent"
		circuit_breaker: true
	}

	// Service mesh configuration
	service_mesh: {
		enabled: true
		sidecar_proxy: "envoy_optimized"
		traffic_management: true
		service_discovery: "automatic"
		observability: "comprehensive"
	}

	// Authentication and authorization
	auth: {
		oauth2: true
		jwt_validation: true
		api_keys: true
		mutual_tls: true
		bpi_wallet_auth: true
	}

	// Traffic management
	traffic: {
		routing_rules: "dynamic"
		canary_deployments: true
		blue_green_deployments: true
		traffic_splitting: true
		fault_injection: true
	}

	// Security and compliance
	security: {
		waf_protection: true
		ddos_mitigation: true
		ssl_termination: true
		security_headers: true
		cors_management: true
	}

	// Monitoring and observability
	observability: {
		distributed_tracing: true
		metrics_collection: true
		log_aggregation: true
		health_checks: true
		performance_monitoring: true
	}

	// Integration capabilities
	integration: {
		payment_gateways: ["stripe", "paypal", "square", "bank_transfers"]
		blockchain_networks: true
		legacy_systems: true
		cloud_services: true
		third_party_apis: true
	}
}
