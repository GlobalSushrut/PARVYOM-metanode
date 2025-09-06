package bpi_core

// Nginx Configuration for BPI Core - High-Performance Web Server
nignix: {
	// Nginx system configuration
	system: {
		type: "high_performance_nginx"
		version: "1.0"
		performance_grade: "enterprise"
		security_level: "maximum"
	}

	// Server configuration
	server: {
		worker_processes: "auto"
		worker_connections: 65536
		keepalive_timeout: "65s"
		client_max_body_size: "100M"
		server_tokens: false
	}

	// SSL/TLS configuration
	ssl: {
		protocols: ["TLSv1.2", "TLSv1.3"]
		ciphers: "ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256"
		prefer_server_ciphers: true
		session_cache: "shared:SSL:10m"
		session_timeout: "10m"
	}

	// Security headers
	security_headers: {
		hsts: "max-age=31536000; includeSubDomains"
		csp: "default-src 'self'; script-src 'self' 'unsafe-inline'"
		x_frame_options: "DENY"
		x_content_type_options: "nosniff"
		referrer_policy: "strict-origin-when-cross-origin"
	}

	// Load balancing
	load_balancing: {
		method: "least_conn"
		health_checks: true
		failover: "automatic"
		sticky_sessions: false
		backup_servers: true
	}

	// Caching configuration
	caching: {
		proxy_cache: true
		cache_size: "1g"
		cache_levels: "1:2"
		cache_valid: "1h"
		cache_bypass: ["$cookie_nocache", "$arg_nocache"]
	}

	// Rate limiting
	rate_limiting: {
		enabled: true
		requests_per_second: 100
		burst: 200
		delay: "nodelay"
		status_code: 429
	}

	// Logging and monitoring
	logging: {
		access_log: true
		error_log: true
		log_format: "json"
		log_rotation: "daily"
		metrics_endpoint: "/nginx_status"
	}

	// Integration with BPI
	bpi_integration: {
		upstream_servers: ["bpi_core:8080", "bpi_api:8081"]
		auth_integration: true
		payment_proxy: true
		oracle_proxy: true
		websocket_support: true
	}
}
