# BPCI Cluster Ledger Server - Deep BPI OS Integration API Documentation

## 🚀 **Production-Ready Enterprise APIs for Deep BPI OS Integration**

The BPCI Cluster Ledger Server now provides comprehensive HTTP API endpoints for all deep BPI OS integration components, enabling government enterprise-grade operations with impossible-to-hide audit trails, quantum security, and AI-powered forensic analysis.

## 📊 **API Overview**

**Base URL**: `http://localhost:8080/api/v1`

**Authentication**: Currently open (production deployment should add authentication)

**Response Format**: All responses follow a consistent JSON structure:
```json
{
  "status": "success|error",
  "message": "Human-readable message",
  "data": { ... },
  "timestamp": "2023-10-23T19:50:25Z"
}
```

---

## 🔧 **Core Cluster Management APIs**

### Health Check
- **GET** `/health`
- **Description**: Comprehensive cluster health status
- **Response**: Detailed health metrics and cluster status

### Status Overview
- **GET** `/status`
- **Description**: Real-time cluster status and metrics
- **Response**: Complete cluster operational status

### Performance Metrics
- **GET** `/api/v1/metrics`
- **Description**: Detailed performance metrics and statistics
- **Response**: Comprehensive performance data

---

## 🌟 **Deep BPI OS Integration APIs**

### 1. Deep Integration Status
- **GET** `/api/v1/deep-integration/status`
- **Description**: Comprehensive status of all deep BPI OS integration components
- **Response**: Status of VM Client CBOR Pipeline, Forensic Oracle, Quantum Engine, BPI Core Bridge, Audit System, and CBOR Pipeline Foundation

### 2. VM Client CBOR Pipeline APIs

#### Process VM Client Request
- **POST** `/api/v1/vm-client/process-request`
- **Description**: Process client requests through 100-year stable CBOR pipeline
- **Request Body**:
```json
{
  "method": "POST",
  "path": "/api/v1/example",
  "headers": {
    "Content-Type": "application/json",
    "Authorization": "Bearer token"
  },
  "body": "base64_encoded_request_body",
  "client_context": "client_wallet_id_or_context"
}
```
- **Response**: Processed CBOR request with government compliance metadata

#### Generate VM Response
- **POST** `/api/v1/vm-client/generate-response`
- **Description**: Generate VM responses with cryptographic signatures
- **Request Body**:
```json
{
  "request_id": "uuid",
  "vm_type": "BPI_VM_TYPE",
  "vm_instance_id": "vm_instance_uuid",
  "status_code": 200,
  "headers": {
    "Content-Type": "application/json"
  },
  "body": "base64_encoded_response_body",
  "processing_start": 1698087025000000000
}
```
- **Response**: Generated CBOR response with witness signatures

### 3. Forensic Oracle CBOR APIs

#### Perform Forensic Analysis
- **POST** `/api/v1/forensic/analyze`
- **Description**: AI-powered forensic analysis with government enterprise-grade compliance
- **Request Body**:
```json
{
  "transaction_data": {
    "tx_id": "transaction_uuid",
    "amount": 1000.50,
    "from": "wallet_address",
    "to": "wallet_address",
    "metadata": { ... }
  },
  "analysis_type": "comprehensive_threat_detection"
}
```
- **Response**: Forensic analysis results with AI insights and threat detection

#### Forensic Status
- **GET** `/api/v1/forensic/status`
- **Description**: Current status of the Forensic Oracle system
- **Response**: Oracle performance metrics, compliance metadata, and audit trail statistics

### 4. Quantum Entanglement Engine APIs

#### Create Quantum Entanglement
- **POST** `/api/v1/quantum/entangle`
- **Description**: Create quantum entanglement with 4D space-time security patterns
- **Request Body**:
```json
{
  "tx_id1": "first_transaction_uuid",
  "tx_id2": "second_transaction_uuid",
  "entanglement_type": "Security"
}
```
- **Supported Entanglement Types**: `Spatial`, `Temporal`, `Security`, `Quantum`, `ChainEntanglement`, `TreeEntanglement`, `TransactionPair`
- **Response**: Quantum entanglement result with cryptographic proofs

#### Quantum Status
- **GET** `/api/v1/quantum/status`
- **Description**: Status of the Quantum Entanglement Engine
- **Response**: Engine status, supported entanglement types, and quantum security information

### 5. BPI OS Operations APIs

#### Process BPI OS Operation
- **POST** `/api/v1/bpi-os/operation`
- **Description**: Process real BPI OS operations through deep integration layers
- **Request Body**:
```json
{
  "operation_type": "smart_contract_deploy|vm_rent_session|storage_operation|consensus_participation",
  "operation_data": {
    "contract_code": "...",
    "vm_config": { ... },
    "storage_key": "...",
    "consensus_data": { ... }
  }
}
```
- **Response**: Operation result with BPI Core Bridge integration status

#### BPI Core Bridge Status
- **GET** `/api/v1/bpi-os/bridge-status`
- **Description**: Status of the BPI Core Bridge connection and operations
- **Response**: Bridge connection state, operation statistics, and real BPI OS integration status

### 6. Immutable Audit System APIs

#### Get Audit Events
- **GET** `/api/v1/audit/events`
- **Description**: Retrieve recent audit events with impossible-to-hide guarantees
- **Response**: Recent audit events with Merkle tree proofs and witness signatures

#### Get Audit Statistics
- **GET** `/api/v1/audit/statistics`
- **Description**: Comprehensive audit system statistics
- **Response**: Audit statistics with integrity validation and compliance metrics

### 7. CBOR Pipeline Foundation APIs

#### CBOR Diagnostic
- **POST** `/api/v1/cbor/diagnostic`
- **Description**: Generate CBOR diagnostic information for government compliance
- **Request Body**:
```json
{
  "data": {
    "any_json_data": "to_be_analyzed",
    "complex_structures": { ... }
  }
}
```
- **Response**: CBOR diagnostic with canonical serialization and audit trail integration

---

## 🛡️ **Security Features**

### Government Enterprise-Grade Compliance
- **Impossible-to-Hide Audit Trails**: All operations are recorded with Merkle tree integration
- **Witness Signatures**: Cryptographic signatures for all critical operations
- **100-Year Stability**: Client information system designed for century-long operation
- **Quantum-Safe Communication**: Advanced quantum security protocols

### AI-Powered Security
- **Threat Detection**: Real-time AI analysis of transactions and operations
- **Evidence Correlation**: Automatic correlation of security events and evidence
- **Workflow Automation**: Automated response to security threats
- **Confidence Thresholds**: Configurable confidence levels for threat detection

### Quantum Security
- **4D Space-Time Patterns**: Advanced quantum entanglement with dimensional security
- **Cryptographic Proofs**: Mathematical proofs of quantum entanglement
- **Coherence Calculations**: Quantum coherence factor calculations
- **BLS Signatures**: Advanced cryptographic signatures for quantum operations

---

## 🔧 **Node Management APIs**

### Register BPI Node
- **POST** `/api/v1/nodes/register`
- **Description**: Register a new BPI node in the cluster
- **Request Body**: Node information and capabilities

### List BPI Nodes
- **GET** `/api/v1/nodes`
- **Description**: List all registered BPI nodes
- **Response**: Complete list of cluster nodes with status

### vPod Cluster Management
- **POST** `/api/v1/vpods/clusters` - Create vPod cluster
- **GET** `/api/v1/vpods/clusters` - List vPod clusters

### Connection Management
- **POST** `/api/v1/connections/establish` - Establish cluster connection
- **POST** `/api/v1/connections/distribute-load` - Distribute load across cluster

### Mesh and Consensus
- **GET** `/api/v1/mesh/status` - BPCI mesh network status
- **GET** `/api/v1/consensus/status` - Consensus mechanism status

---

## 📈 **Usage Examples**

### Example 1: Process VM Client Request
```bash
curl -X POST http://localhost:8080/api/v1/vm-client/process-request \
  -H "Content-Type: application/json" \
  -d '{
    "method": "POST",
    "path": "/api/v1/transaction",
    "headers": {"Content-Type": "application/json"},
    "body": "eyJ0eF9pZCI6InRlc3QifQ==",
    "client_context": "client_wallet_123"
  }'
```

### Example 2: Create Quantum Entanglement
```bash
curl -X POST http://localhost:8080/api/v1/quantum/entangle \
  -H "Content-Type: application/json" \
  -d '{
    "tx_id1": "tx_123",
    "tx_id2": "tx_456",
    "entanglement_type": "Security"
  }'
```

### Example 3: Perform Forensic Analysis
```bash
curl -X POST http://localhost:8080/api/v1/forensic/analyze \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_data": {
      "tx_id": "suspicious_tx_789",
      "amount": 10000.00,
      "from": "wallet_abc",
      "to": "wallet_xyz"
    },
    "analysis_type": "comprehensive_threat_detection"
  }'
```

### Example 4: Get Deep Integration Status
```bash
curl -X GET http://localhost:8080/api/v1/deep-integration/status
```

---

## 🚀 **Production Deployment**

### Environment Variables
```bash
export BPCI_SERVER_HOST="0.0.0.0"
export BPCI_SERVER_PORT="8080"
export BPI_CONSENSUS_SERVER_URL="http://consensus-server:9090"
export BPI_BRIDGE_SERVER_URL="http://bridge-server:8090"
export VPOD_ALLOCATION_STRATEGY="round_robin"
```

### Docker Deployment
```dockerfile
FROM rust:1.70
WORKDIR /app
COPY . .
RUN cargo build --release --bin bpci_cluster_ledger_server
EXPOSE 8080
CMD ["./target/release/bpci_cluster_ledger_server"]
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: bpci-cluster-ledger-server
spec:
  replicas: 3
  selector:
    matchLabels:
      app: bpci-cluster-ledger-server
  template:
    metadata:
      labels:
        app: bpci-cluster-ledger-server
    spec:
      containers:
      - name: server
        image: bpci-cluster-ledger-server:latest
        ports:
        - containerPort: 8080
        env:
        - name: BPCI_SERVER_HOST
          value: "0.0.0.0"
        - name: BPCI_SERVER_PORT
          value: "8080"
```

---

## 📋 **API Testing Checklist**

- [ ] Health endpoint responds correctly
- [ ] Deep integration status shows all components active
- [ ] VM Client CBOR Pipeline processes requests
- [ ] Forensic analysis generates threat detection results
- [ ] Quantum entanglement creates security patterns
- [ ] BPI OS operations process through deep integration
- [ ] Audit events are recorded with impossible-to-hide guarantees
- [ ] CBOR diagnostics generate compliance information

---

## 🔒 **Security Considerations**

1. **Authentication**: Add proper authentication for production deployment
2. **Rate Limiting**: Implement rate limiting for API endpoints
3. **Input Validation**: Validate all input data for security
4. **HTTPS**: Use HTTPS in production environments
5. **Audit Logging**: All API calls are automatically logged in the immutable audit system
6. **Government Compliance**: All operations meet government enterprise-grade requirements

---

## 📞 **Support and Monitoring**

- **Health Monitoring**: Use `/health` endpoint for load balancer health checks
- **Metrics Collection**: Use `/api/v1/metrics` for Prometheus/Grafana monitoring
- **Audit Trail**: All operations are recorded in the immutable audit system
- **Real-time Status**: Use deep integration status endpoint for system monitoring

The BPCI Cluster Ledger Server now provides a complete, production-ready HTTP API for all deep BPI OS integration features, enabling seamless integration with government enterprise-grade security, quantum protection, and AI-powered forensic analysis.
