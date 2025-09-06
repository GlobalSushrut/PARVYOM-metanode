# HttpCG Client Troubleshooting and Monitoring Guide

## Overview

This comprehensive guide provides detailed troubleshooting procedures, monitoring strategies, and operational best practices for the HttpCG protocol client. It covers common issues, diagnostic tools, performance optimization, and proactive monitoring to ensure reliable operation in production environments.

## Diagnostic Tools and Commands

### Built-in Diagnostic Commands
```bash
# Service health check
httpcg-client health-check
httpcg-client health-check --detailed --json

# Configuration validation
httpcg-client config validate
httpcg-client config test --dry-run

# Connection testing
httpcg-client test connection --endpoint https://gateway.pravyom.com
httpcg-client test shadow-registry --url httpcg://app/api.example.com/test

# Certificate management
httpcg-client cert list
httpcg-client cert verify --certificate-id <cert_id>
httpcg-client cert renew --force

# Performance diagnostics
httpcg-client perf benchmark --duration 60s
httpcg-client perf profile --output /tmp/profile.json

# Cache management
httpcg-client cache stats
httpcg-client cache clear --type shadow-registry
httpcg-client cache clear --type qlock-sessions

# Metrics export
httpcg-client metrics export --format prometheus
httpcg-client metrics export --format json --output /tmp/metrics.json
```

### System-level Diagnostics
```bash
#!/bin/bash
# System diagnostic script for HttpCG client

echo "=== HttpCG Client System Diagnostics ==="

# Service status
echo "## Service Status"
systemctl status httpcg-client --no-pager

# Process information
echo "## Process Information"
ps aux | grep httpcg-client
pgrep -f httpcg-client | xargs -I {} cat /proc/{}/status | grep -E "(Name|Pid|VmRSS|VmSize|Threads)"

# Network connections
echo "## Network Connections"
netstat -tlnp | grep httpcg-client
ss -tlnp | grep httpcg-client

# File descriptors
echo "## File Descriptors"
lsof -p $(pgrep httpcg-client) | wc -l
cat /proc/$(pgrep httpcg-client)/limits | grep "Max open files"

# Memory usage
echo "## Memory Usage"
cat /proc/$(pgrep httpcg-client)/status | grep -E "(VmPeak|VmSize|VmRSS|VmSwap)"

# Disk usage
echo "## Disk Usage"
du -sh /var/lib/httpcg-client/*
du -sh /var/log/httpcg-client/*

# System resources
echo "## System Resources"
free -h
df -h /var/lib/httpcg-client /var/log/httpcg-client

# Network connectivity
echo "## Network Connectivity"
curl -s -o /dev/null -w "%{http_code} %{time_total}s" https://gateway.pravyom.com/health
```

## Common Issues and Solutions

### Issue 1: Service Won't Start

#### Symptoms
```
● httpcg-client.service - HttpCG Protocol Client
   Loaded: loaded (/etc/systemd/system/httpcg-client.service; enabled; vendor preset: enabled)
   Active: failed (Result: exit-code) since Mon 2024-01-15 10:30:45 UTC; 5s ago
```

#### Diagnostic Steps
```bash
# Check service logs
journalctl -u httpcg-client --since "1 hour ago" --no-pager

# Check configuration syntax
httpcg-client config validate

# Check file permissions
ls -la /etc/httpcg-client/
ls -la /var/lib/httpcg-client/
ls -la /var/log/httpcg-client/

# Check port availability
netstat -tlnp | grep :8080
ss -tlnp | grep :8080

# Test configuration manually
sudo -u httpcg httpcg-client start --config /etc/httpcg-client/config.toml --dry-run
```

#### Common Solutions
```bash
# Fix file permissions
sudo chown -R httpcg:httpcg /var/lib/httpcg-client
sudo chown -R httpcg:httpcg /var/log/httpcg-client
sudo chmod 750 /var/lib/httpcg-client
sudo chmod 755 /var/log/httpcg-client

# Fix configuration file permissions
sudo chown root:httpcg /etc/httpcg-client/config.toml
sudo chmod 640 /etc/httpcg-client/config.toml

# Create missing directories
sudo mkdir -p /var/lib/httpcg-client/{cache,certificates,keys}
sudo mkdir -p /var/log/httpcg-client
sudo chown -R httpcg:httpcg /var/lib/httpcg-client /var/log/httpcg-client

# Generate missing keys
sudo -u httpcg httpcg-client keys generate --output /var/lib/httpcg-client/keys/
```

### Issue 2: Connection Timeouts

#### Symptoms
```
ERROR httpcg_client: Connection timeout to gateway.pravyom.com:443
ERROR shadow_registry: Failed to resolve httpcg URL: connection timeout
```

#### Diagnostic Steps
```bash
# Test network connectivity
ping gateway.pravyom.com
curl -v --connect-timeout 10 https://gateway.pravyom.com/health

# Check DNS resolution
nslookup gateway.pravyom.com
dig gateway.pravyom.com

# Check firewall rules
sudo ufw status verbose
sudo iptables -L -n

# Check routing
traceroute gateway.pravyom.com
mtr --report gateway.pravyom.com

# Test with different timeout values
httpcg-client test connection --endpoint https://gateway.pravyom.com --timeout 60s
```

#### Solutions
```toml
# Increase timeout values in config.toml
[client]
connection_timeout = "60s"
read_timeout = "120s"
write_timeout = "120s"

[shadow_registry]
max_retries = 5
retry_backoff = ["200ms", "1s", "5s", "10s", "30s"]

# Add backup endpoints
gateway_endpoints = [
    "https://gateway1.pravyom.com",
    "https://gateway2.pravyom.com", 
    "https://gateway3.pravyom.com"
]
```

### Issue 3: Certificate Validation Failures

#### Symptoms
```
ERROR tlsls_manager: Certificate validation failed: invalid signature
ERROR httpcg_client: TLSLS handshake failed: certificate expired
```

#### Diagnostic Steps
```bash
# Check certificate status
httpcg-client cert list --verbose
httpcg-client cert verify --all

# Check certificate files
openssl x509 -in /var/lib/httpcg-client/certificates/client.crt -text -noout
openssl x509 -in /var/lib/httpcg-client/certificates/client.crt -checkend 86400

# Check system time
timedatectl status
ntpq -p

# Check CA bundle
openssl verify -CAfile /etc/ssl/certs/ca-certificates.crt /var/lib/httpcg-client/certificates/client.crt
```

#### Solutions
```bash
# Regenerate certificates
httpcg-client cert generate --force
httpcg-client cert renew --all

# Update CA bundle
sudo apt-get update && sudo apt-get install ca-certificates

# Sync system time
sudo ntpdate -s time.nist.gov
sudo systemctl restart ntp

# Fix certificate permissions
sudo chown httpcg:httpcg /var/lib/httpcg-client/certificates/*
sudo chmod 600 /var/lib/httpcg-client/certificates/*.key
sudo chmod 644 /var/lib/httpcg-client/certificates/*.crt
```

### Issue 4: High Memory Usage

#### Symptoms
```
WARNING: Memory usage high: 2.5GB (limit: 2GB)
ERROR: Out of memory, killing process
```

#### Diagnostic Steps
```bash
# Check memory usage details
cat /proc/$(pgrep httpcg-client)/status | grep -E "(VmPeak|VmSize|VmRSS|VmSwap)"
cat /proc/$(pgrep httpcg-client)/smaps | grep -E "(Size|Rss|Pss)" | awk '{sum+=$2} END {print sum " KB"}'

# Check cache sizes
httpcg-client cache stats --detailed

# Memory profiling
httpcg-client perf profile --type memory --duration 300s --output /tmp/memory-profile.json

# Check for memory leaks
valgrind --tool=memcheck --leak-check=full httpcg-client start --config /etc/httpcg-client/config.toml
```

#### Solutions
```toml
# Reduce cache sizes in config.toml
[shadow_registry]
cache_size = 5000  # Reduce from 10000
cache_ttl = "2m"   # Reduce from 5m

[qlock]
cache_size_limit = 5000  # Reduce from 10000
cleanup_interval = "15s" # Increase cleanup frequency

[client]
max_connections = 5000  # Reduce if too high

# Enable memory limits
[performance]
max_memory_usage = "1.5GB"
memory_cleanup_threshold = 0.8
```

### Issue 5: Poor Performance

#### Symptoms
```
High response times (>5s)
Low throughput (<100 req/s)
High CPU usage (>90%)
```

#### Diagnostic Steps
```bash
# Performance benchmarking
httpcg-client perf benchmark --concurrent 100 --duration 60s
httpcg-client perf benchmark --endpoint httpcg://app/api.example.com/test

# CPU profiling
perf record -g httpcg-client start --config /etc/httpcg-client/config.toml
perf report

# I/O analysis
iotop -p $(pgrep httpcg-client)
iostat -x 1

# Network analysis
iftop -i eth0
nethogs -p $(pgrep httpcg-client)
```

#### Solutions
```toml
# Performance optimization in config.toml
[performance]
worker_threads = 16  # Increase based on CPU cores
connection_pool_size = 2000
enable_response_caching = true
response_cache_size = "200MB"

# Compression settings
enable_compression = true
compression_algorithm = "lz4"  # Faster than brotli
compression_level = 3  # Lower level for speed

# Buffer optimization
read_buffer_size = "128KB"
write_buffer_size = "128KB"
```

## Monitoring Setup

### Prometheus Metrics
```yaml
# Custom metrics configuration
metrics:
  - name: httpcg_requests_total
    type: counter
    help: "Total number of HTTP requests"
    labels: ["method", "endpoint", "status"]
    
  - name: httpcg_request_duration_seconds
    type: histogram
    help: "Request duration in seconds"
    buckets: [0.1, 0.5, 1.0, 2.5, 5.0, 10.0]
    
  - name: httpcg_active_connections
    type: gauge
    help: "Number of active connections"
    
  - name: httpcg_cache_hits_total
    type: counter
    help: "Cache hits by type"
    labels: ["cache_type"]
    
  - name: httpcg_certificate_expiry_seconds
    type: gauge
    help: "Seconds until certificate expiry"
    labels: ["certificate_id"]
    
  - name: httpcg_qlock_sessions_active
    type: gauge
    help: "Active QLOCK sessions"
    
  - name: httpcg_memory_usage_bytes
    type: gauge
    help: "Memory usage in bytes"
    labels: ["type"]
```

### Grafana Dashboards

#### Main Dashboard
```json
{
  "dashboard": {
    "title": "HttpCG Client - Main Dashboard",
    "panels": [
      {
        "title": "Request Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(rate(httpcg_requests_total[5m]))",
            "legendFormat": "Requests/sec"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "unit": "reqps",
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "yellow", "value": 1000},
                {"color": "red", "value": 5000}
              ]
            }
          }
        }
      },
      {
        "title": "Response Time Distribution",
        "type": "heatmap",
        "targets": [
          {
            "expr": "sum(rate(httpcg_request_duration_seconds_bucket[5m])) by (le)",
            "format": "heatmap",
            "legendFormat": "{{le}}"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(rate(httpcg_requests_total{status=~\"4..|5..\"}[5m])) / sum(rate(httpcg_requests_total[5m])) * 100",
            "legendFormat": "Error %"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "unit": "percent",
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "yellow", "value": 1},
                {"color": "red", "value": 5}
              ]
            }
          }
        }
      },
      {
        "title": "Active Connections",
        "type": "timeseries",
        "targets": [
          {
            "expr": "httpcg_active_connections",
            "legendFormat": "Connections"
          }
        ]
      }
    ]
  }
}
```

#### Security Dashboard
```json
{
  "dashboard": {
    "title": "HttpCG Client - Security Dashboard",
    "panels": [
      {
        "title": "Certificate Expiry",
        "type": "table",
        "targets": [
          {
            "expr": "httpcg_certificate_expiry_seconds / 86400",
            "format": "table",
            "legendFormat": "{{certificate_id}}"
          }
        ],
        "transformations": [
          {
            "id": "organize",
            "options": {
              "excludeByName": {"Time": true},
              "renameByName": {"Value": "Days to Expiry"}
            }
          }
        ]
      },
      {
        "title": "QLOCK Session Activity",
        "type": "timeseries",
        "targets": [
          {
            "expr": "httpcg_qlock_sessions_active",
            "legendFormat": "Active Sessions"
          },
          {
            "expr": "rate(httpcg_qlock_sessions_created_total[5m])",
            "legendFormat": "Sessions Created/sec"
          }
        ]
      },
      {
        "title": "Security Events",
        "type": "logs",
        "targets": [
          {
            "expr": "{job=\"httpcg-client\"} |= \"security\" | json",
            "refId": "A"
          }
        ]
      }
    ]
  }
}
```

### Alerting Rules

#### Prometheus Alerting Rules
```yaml
# /etc/prometheus/rules/httpcg-client.yml
groups:
  - name: httpcg-client
    rules:
      - alert: HttpcgClientDown
        expr: up{job="httpcg-client"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "HttpCG Client is down"
          description: "HttpCG Client has been down for more than 1 minute"
          
      - alert: HttpcgHighErrorRate
        expr: sum(rate(httpcg_requests_total{status=~"4..|5.."}[5m])) / sum(rate(httpcg_requests_total[5m])) > 0.05
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value | humanizePercentage }} for 5 minutes"
          
      - alert: HttpcgHighResponseTime
        expr: histogram_quantile(0.95, sum(rate(httpcg_request_duration_seconds_bucket[5m])) by (le)) > 5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High response time detected"
          description: "95th percentile response time is {{ $value }}s"
          
      - alert: HttpcgCertificateExpiring
        expr: httpcg_certificate_expiry_seconds < 7 * 24 * 3600
        for: 1h
        labels:
          severity: warning
        annotations:
          summary: "Certificate expiring soon"
          description: "Certificate {{ $labels.certificate_id }} expires in {{ $value | humanizeDuration }}"
          
      - alert: HttpcgHighMemoryUsage
        expr: httpcg_memory_usage_bytes{type="rss"} > 2 * 1024 * 1024 * 1024
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage"
          description: "Memory usage is {{ $value | humanizeBytes }}"
```

### Log Analysis

#### Log Parsing with ELK Stack
```yaml
# logstash.conf
input {
  beats {
    port => 5044
  }
}

filter {
  if [fields][service] == "httpcg-client" {
    json {
      source => "message"
    }
    
    date {
      match => [ "timestamp", "ISO8601" ]
    }
    
    if [level] == "ERROR" {
      mutate {
        add_tag => ["error"]
      }
    }
    
    if [message] =~ /security/ {
      mutate {
        add_tag => ["security"]
      }
    }
    
    grok {
      match => { 
        "message" => "%{WORD:request_method} %{URIPATH:request_path} %{NUMBER:response_code:int} %{NUMBER:response_time:float}ms"
      }
      tag_on_failure => ["_grokparsefailure"]
    }
  }
}

output {
  elasticsearch {
    hosts => ["elasticsearch:9200"]
    index => "httpcg-client-%{+YYYY.MM.dd}"
  }
}
```

#### Kibana Visualizations
```json
{
  "visualization": {
    "title": "HttpCG Request Analysis",
    "type": "line",
    "params": {
      "grid": {"categoryLines": false, "style": {"color": "#eee"}},
      "categoryAxes": [{"id": "CategoryAxis-1", "type": "category", "position": "bottom"}],
      "valueAxes": [{"id": "ValueAxis-1", "name": "LeftAxis-1", "type": "value", "position": "left"}],
      "seriesParams": [{"show": true, "type": "line", "mode": "normal", "data": {"label": "Count", "id": "1"}}]
    },
    "aggs": [
      {
        "id": "1",
        "enabled": true,
        "type": "count",
        "schema": "metric",
        "params": {}
      },
      {
        "id": "2", 
        "enabled": true,
        "type": "date_histogram",
        "schema": "segment",
        "params": {
          "field": "@timestamp",
          "interval": "auto",
          "min_doc_count": 1
        }
      }
    ]
  }
}
```

## Performance Monitoring

### Key Performance Indicators (KPIs)
```yaml
# Performance thresholds and targets
performance_targets:
  response_time:
    p50: "< 100ms"
    p95: "< 500ms"
    p99: "< 1000ms"
    
  throughput:
    minimum: "1000 req/s"
    target: "5000 req/s"
    maximum: "10000 req/s"
    
  error_rate:
    target: "< 0.1%"
    warning: "< 1%"
    critical: "> 5%"
    
  availability:
    target: "99.9%"
    minimum: "99.5%"
    
  resource_usage:
    cpu: "< 80%"
    memory: "< 2GB"
    disk: "< 80%"
    network: "< 80%"
```

### Automated Performance Testing
```bash
#!/bin/bash
# Automated performance test script

ENDPOINT="http://localhost:8080"
DURATION="300"  # 5 minutes
CONCURRENT="100"

echo "Starting performance test..."

# Warm-up
echo "Warming up..."
httpcg-client perf benchmark \
    --endpoint "$ENDPOINT" \
    --concurrent 10 \
    --duration 60 \
    --quiet

# Main test
echo "Running main performance test..."
httpcg-client perf benchmark \
    --endpoint "$ENDPOINT" \
    --concurrent "$CONCURRENT" \
    --duration "$DURATION" \
    --output /tmp/perf-results.json \
    --detailed

# Analyze results
RESULTS_FILE="/tmp/perf-results.json"
if [[ -f "$RESULTS_FILE" ]]; then
    echo "Performance test results:"
    
    # Extract key metrics
    THROUGHPUT=$(jq -r '.summary.requests_per_second' "$RESULTS_FILE")
    P95_LATENCY=$(jq -r '.summary.latency_percentiles.p95' "$RESULTS_FILE")
    ERROR_RATE=$(jq -r '.summary.error_rate' "$RESULTS_FILE")
    
    echo "Throughput: $THROUGHPUT req/s"
    echo "P95 Latency: $P95_LATENCY ms"
    echo "Error Rate: $ERROR_RATE%"
    
    # Check against thresholds
    if (( $(echo "$THROUGHPUT < 1000" | bc -l) )); then
        echo "WARNING: Throughput below minimum threshold (1000 req/s)"
    fi
    
    if (( $(echo "$P95_LATENCY > 500" | bc -l) )); then
        echo "WARNING: P95 latency above threshold (500ms)"
    fi
    
    if (( $(echo "$ERROR_RATE > 1" | bc -l) )); then
        echo "WARNING: Error rate above threshold (1%)"
    fi
fi
```

## Security Monitoring

### Security Event Detection
```bash
#!/bin/bash
# Security monitoring script

LOG_FILE="/var/log/httpcg-client/audit.log"
ALERT_FILE="/tmp/security-alerts.log"

# Monitor for security events
tail -f "$LOG_FILE" | while read line; do
    # Check for authentication failures
    if echo "$line" | grep -q "authentication_failed"; then
        echo "$(date): Authentication failure detected: $line" >> "$ALERT_FILE"
    fi
    
    # Check for certificate errors
    if echo "$line" | grep -q "certificate_validation_failed"; then
        echo "$(date): Certificate validation failure: $line" >> "$ALERT_FILE"
    fi
    
    # Check for rate limiting
    if echo "$line" | grep -q "rate_limit_exceeded"; then
        echo "$(date): Rate limit exceeded: $line" >> "$ALERT_FILE"
    fi
    
    # Check for suspicious activity
    if echo "$line" | grep -q "suspicious_activity"; then
        echo "$(date): Suspicious activity detected: $line" >> "$ALERT_FILE"
    fi
done
```

### Certificate Monitoring
```bash
#!/bin/bash
# Certificate expiry monitoring

CERT_DIR="/var/lib/httpcg-client/certificates"
WARN_DAYS=30

echo "Checking certificate expiry..."

for cert_file in "$CERT_DIR"/*.crt; do
    if [[ -f "$cert_file" ]]; then
        cert_name=$(basename "$cert_file" .crt)
        
        # Check expiry date
        expiry_date=$(openssl x509 -in "$cert_file" -noout -enddate | cut -d= -f2)
        expiry_epoch=$(date -d "$expiry_date" +%s)
        current_epoch=$(date +%s)
        days_until_expiry=$(( (expiry_epoch - current_epoch) / 86400 ))
        
        echo "Certificate $cert_name expires in $days_until_expiry days"
        
        if [[ $days_until_expiry -lt $WARN_DAYS ]]; then
            echo "WARNING: Certificate $cert_name expires soon ($days_until_expiry days)"
            
            # Attempt automatic renewal
            if httpcg-client cert renew --certificate-id "$cert_name"; then
                echo "Successfully renewed certificate $cert_name"
            else
                echo "ERROR: Failed to renew certificate $cert_name"
            fi
        fi
    fi
done
```

## Operational Procedures

### Health Check Automation
```bash
#!/bin/bash
# Comprehensive health check script

HEALTH_ENDPOINT="http://localhost:8081/health"
METRICS_ENDPOINT="http://localhost:9090/metrics"

echo "=== HttpCG Client Health Check ==="

# Basic service check
if systemctl is-active --quiet httpcg-client; then
    echo "✓ Service is running"
else
    echo "✗ Service is not running"
    exit 1
fi

# HTTP health check
if curl -sf "$HEALTH_ENDPOINT" > /dev/null; then
    echo "✓ HTTP health check passed"
else
    echo "✗ HTTP health check failed"
    exit 1
fi

# Metrics endpoint check
if curl -sf "$METRICS_ENDPOINT" > /dev/null; then
    echo "✓ Metrics endpoint accessible"
else
    echo "✗ Metrics endpoint not accessible"
fi

# Configuration validation
if httpcg-client config validate --quiet; then
    echo "✓ Configuration is valid"
else
    echo "✗ Configuration validation failed"
    exit 1
fi

# Certificate check
cert_status=$(httpcg-client cert check --json)
expired_certs=$(echo "$cert_status" | jq -r '.expired | length')
expiring_certs=$(echo "$cert_status" | jq -r '.expiring_soon | length')

if [[ $expired_certs -eq 0 ]]; then
    echo "✓ No expired certificates"
else
    echo "✗ $expired_certs expired certificates found"
fi

if [[ $expiring_certs -eq 0 ]]; then
    echo "✓ No certificates expiring soon"
else
    echo "⚠ $expiring_certs certificates expiring soon"
fi

# Performance check
response_time=$(curl -o /dev/null -s -w "%{time_total}" "$HEALTH_ENDPOINT")
if (( $(echo "$response_time < 1.0" | bc -l) )); then
    echo "✓ Response time acceptable (${response_time}s)"
else
    echo "⚠ Response time high (${response_time}s)"
fi

echo "Health check completed"
```

### Maintenance Automation
```bash
#!/bin/bash
# Automated maintenance script

echo "Starting automated maintenance..."

# Log rotation
echo "Rotating logs..."
sudo logrotate -f /etc/logrotate.d/httpcg-client

# Cache cleanup
echo "Cleaning caches..."
httpcg-client cache cleanup --max-age 24h

# Certificate renewal check
echo "Checking certificates..."
httpcg-client cert check --renew-if-needed

# Performance optimization
echo "Optimizing performance..."
httpcg-client perf optimize --auto

# Security scan
echo "Running security scan..."
httpcg-client security scan --report /tmp/security-report.json

# Backup configuration
echo "Backing up configuration..."
tar -czf "/var/backups/httpcg-client-config-$(date +%Y%m%d).tar.gz" \
    /etc/httpcg-client \
    /var/lib/httpcg-client/keys \
    /var/lib/httpcg-client/certificates

# Cleanup old backups (keep 30 days)
find /var/backups -name "httpcg-client-config-*.tar.gz" -mtime +30 -delete

echo "Maintenance completed"
```

## Emergency Procedures

### Service Recovery
```bash
#!/bin/bash
# Emergency service recovery procedure

echo "=== HttpCG Client Emergency Recovery ==="

# Stop service
echo "Stopping service..."
sudo systemctl stop httpcg-client

# Check for core dumps
if ls /var/lib/httpcg-client/core.* 1> /dev/null 2>&1; then
    echo "Core dumps found, backing up..."
    mkdir -p /tmp/httpcg-recovery
    cp /var/lib/httpcg-client/core.* /tmp/httpcg-recovery/
fi

# Reset to known good configuration
echo "Restoring configuration..."
if [[ -f /etc/httpcg-client/config.toml.backup ]]; then
    cp /etc/httpcg-client/config.toml.backup /etc/httpcg-client/config.toml
fi

# Clear problematic caches
echo "Clearing caches..."
rm -rf /var/lib/httpcg-client/cache/*

# Regenerate certificates if needed
echo "Checking certificates..."
if ! httpcg-client cert verify --all --quiet; then
    echo "Regenerating certificates..."
    httpcg-client cert generate --force
fi

# Start service
echo "Starting service..."
sudo systemctl start httpcg-client

# Wait and verify
sleep 10
if systemctl is-active --quiet httpcg-client; then
    echo "✓ Service recovery successful"
else
    echo "✗ Service recovery failed"
    journalctl -u httpcg-client --since "5 minutes ago" --no-pager
    exit 1
fi
```

## Next Steps

1. **[API Reference](./07-api-reference.md)** - Complete API documentation and examples

## References

- **Monitoring Implementation**: `/home/umesh/metanode/bpi-core/src/bpi_vm_server.rs` (Health checks and metrics)
- **Diagnostic Tools**: `/home/umesh/metanode/bpi-core/src/client/httpcg_client.rs` (Built-in diagnostics)
- **Security Monitoring**: `/home/umesh/metanode/wallet-identity/src/client/transport/httpcg_client.rs` (Security events)
- **Performance Testing**: Integration test examples and benchmarking tools
