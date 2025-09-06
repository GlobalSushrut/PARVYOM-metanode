# Metanode / BPI Mesh — How to Build
Hands-on assembly guide to stand up devnet, testnet, and core components. Use alongside starter-repo.md.

---

## 0) Prereqs
- OS: Linux/macOS; Docker/Podman + Compose; or k3d/kind (Kubernetes)
- Tooling: rustup (stable), go>=1.22, node>=20, python>=3.11
- Crypto libs: blst, libsodium, blake3 (via package manager)
- CLI: make, jq, yq, curl

---

## 1) Clone starter and bootstrap
```bash
# TODO: replace with actual repo URL
git clone https://github.com/metanode-labs/starter.git
cd starter
make bootstrap  # installs toolchains, pre-commit hooks
```

---

## 2) Build core libs
```bash
# Rust libs
cargo build -p bpci -p merkle -p poh -p receipts -p bls-agg

# Go tools (light client, sampler)
go build ./cmd/lc-verify
go build ./cmd/da-sampler
```
- Outputs:
  - libbpci.a, libmerkle.a, libpoh.a, libreceipts.a, libblsagg.a
  - lc-verify, da-sampler

---

## 3) Run devnet (3 validators)
```bash
# Docker Compose
make up-devnet   # or: docker compose -f compose.devnet.yml up -d

# Verify headers
./bin/lc-verify --rpc http://localhost:8547 --last 10
```
Acceptance:
- prev_hash chain OK, BLS agg verify OK, PoH root OK, DA off if not enabled

---

## 4) Send a BPCI and get a receipt
```bash
# Build agreement
agreementc build agreements/acme.v1.yaml -o build/acme.v1.wasm

# Pin agreement
bpi agreement pin build/acme.v1.wasm

# Run service in DockLock
bpi run service api --deterministic

# Send payload
curl -s "http://localhost:8080/registry.submit" -d '{"amount":420,"pii":"***"}' | jq

# Fetch receipt
bpi receipts get <run-id>
```
Acceptance:
- receipt contains witness roots, policy decision, and node signature

---

## 5) Enable DA and sampling
```bash
export DA_RS_K=8 DA_RS_N=12 DA_CHUNK=262144
make enable-da

# Sampler runs against latest header
go run ./cmd/da-sampler --n 12 --k 8 --samples 20 --rpc http://localhost:8547
```
Acceptance:
- sampler reports all challenges satisfied; da_root present in headers

---

## 6) Anchors and inclusion lists (optional in devnet)
```bash
# Simulate anchors (devnet stub)
make anchors-dev

# Enable inclusion rule checks
bpi config set inclusion.window 4
```
Acceptance:
- lc-verify reports anchors OK when present; inclusion violations trigger alerts

---

## 7) Kubernetes deployment (testnet)
```bash
# Create cluster (example with k3d)
k3d cluster create metanode --agents 3

# Install charts
helm repo add metanode https://charts.metanode.dev
helm install validators metanode/validators -f charts/values.validators.yaml
helm install relays metanode/relays -f charts/values.relays.yaml
helm install pinners metanode/pinners -f charts/values.pinners.yaml
helm install core metanode/core -f charts/values.core.yaml
```
Acceptance:
- 9 validators live, relays healthy, DA pinners responding, faucet reachable

---

## 8) Observability
```bash
helm install monitoring kube-prom-stack -f charts/values.monitoring.yaml
kubectl -n monitoring port-forward svc/grafana 3000:80
```
Dashboards: Chain health, DA health, Validator set, Tenant throughput

---

## 9) Security & keys
- Generate Ed25519 identities via SPIRE/SPIFFE; inject via SDS
- BLS keys via threshold ceremony (t-of-n); rotate monthly
- Store secrets in KMS; use sealed-secrets for k8s

---

## 10) Runbooks
- Onboarding: create tenant, pin agreement, deploy service, verify receipts
- Incident: finality regression, DA failure, censorship alert
- Upgrade: rolling restart, chart version bump, post-verify with lc-verify
- Key rotation: drain, rotate, rejoin; verify validator_set_hash change

---

## 11) Validation checklist (quick)
- lc-verify passes BLS, PoH, DA, inclusion (when enabled)
- Receipts contain trace roots and signatures
- DA sampler success ≥ 99.9%
- Anchors posted within SLA

---

## 12) Next steps
- Wire side-net peering for staging
- Bring multi-cloud storage providers online
- Launch public testnet with faucet and examples
