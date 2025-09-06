# Metanode / BPI Mesh — Infrastructure Architecture
A practical blueprint for environments, topology, networking, security, and ops.

---

## 1) Environments
- Devnet: single machine/k3d or docker-compose; 3 validators, 1 relay, 1 DA pinner; local registry.
- Testnet: managed k8s (GKE/EKS/AKS) small nodes; 9 validators, 3 relays, 3 pinners; public endpoints; faucet.
- Mainnet: multi-region, multi-ISP; ≥9 validators (Phase 1), ≥25 (Phase 2+); 3–5 relays; ≥3 DA clusters; anchors to 2 L1s.

---

## 2) Core Topology (No validators on Core)
- Core (2 GB k3s, stateless):
  - bpi-control (admin API + feature flags)
  - bpi-directory (validator directory, diversity policy)
  - bpi-gateway (tenant onboarding, receipt registry facade)
  - bpi-headers-proxy (light relay for headers only)
  - bpi-billing (meter→settlement hash)
- Per-Tenant Project Cluster:
  - App services (Docker/K8s)
  - bpi-agent (BPCI client + DockLock driver)
  - bpi-gateway (ingress)
  - DockLock (optional sidecar/daemon)
- Per-Tenant Mainnet-mini Cluster:
  - Validators (IBFT header-only)
  - DA pinner (RS shards)
  - Relay (QUIC/MASQUE)

---

## 3) Network Plan
- Transport: QUIC/HTTP3 default; MASQUE tunneling; TCP/TLS1.3 fallback.
- Encryption:
  - Control plane: mTLS (SPIFFE/SVID), Ed25519 identities.
  - Data plane (BPCI): X25519→XChaCha20-Poly1305 AEAD; AD = bpci_hdr.
- Anti-eclipse: multi-relay broadcast, ≥3 relays across distinct ASNs/regions.
- Firewalling:
  - Only relays expose public UDP/443 (QUIC) + TCP/443.
  - Validators accept from relays only.
  - Core exposes narrow HTTPS APIs; no inbound from internet to validator ports.

---

## 4) Storage & DA
- Object storage: CAR/DAG chunks encrypted per-tenant; local PV + S3/GCS/Azure + IPFS/Filecoin.
- Databases:
  - db.class (SQLite→Postgres): metadata, cid_map, keyslots, SBOM signers.
  - db.mock (append-only log): RocksDB/Parquet.
- DA encoding: RS(k/n) at edges; da_root committed in headers.
- Backups: snapshot db.class + cid_map; CARs re-fetchable from IPFS/Filecoin; verify with da_root.

---

## 5) Identity & Keys
- SPIFFE/SVID for service identity; rotate daily.
- Service keys: weekly rotation; stored in HSM/TEE where possible.
- Consensus keys: BLS epoch rotation monthly; threshold BLS (t-of-n) recommended.
- Secrets: KMS integration (GCP KMS/AWS KMS), sealed-secrets for k8s.

---

## 6) Security Controls
- DockLock determinism cage: seccomp, AppArmor/SELinux, cgroups; deny nondeterministic syscalls.
- Network Policies (k8s): namespace isolation; only relays cross-namespace.
- Audit: Prometheus + Loki + OpenTelemetry; ship logs with hash-chains for tamper-evidence.
- Censorship resistance: VRF proposer, inclusion lists, force-inclusion inbox anchored on L1.

---

## 7) Sizing & Capacity (initial targets)
- Validator: 1 vCPU, 180 MiB RAM, 2–5 GB disk.
- DA pinner: 1 vCPU, 150 MiB RAM, disk per retention (50–500 GB).
- Relay: 1 vCPU, 100–250 MiB RAM, high NIC throughput.
- Core pod: 0.5 vCPU, 256–512 MiB RAM each.
- Block time: 250 ms; <1 s finality (p50) with N=9.

---

## 8) Deployment Models
- Single-node dev: k3d/docker-compose; scripts/bpi up.
- Managed k8s: Helm charts for validators/relays/pinners; separate release for core.
- Bare-metal: systemd + containerd; same images; wireguard mesh optional.

---

## 9) Observability & SLOs
- Metrics: consensus latency, BLS verify ms, PoH tick rate, DA probe %, relay delivery %, mempool depth, validator diversity counters.
- Dashboards: Chain health, DA health, Validator set, Tenant throughput.
- Alerts: finality >3 s (5 m), DA failures >0.1%, anchor lag >10 m, participation <2f+1.

---

## 10) Disaster Recovery
- Core stateless: blue/green; restore from config + directory snapshot.
- DA: reconstruct from any k shards; periodic restore drills.
- Anchors: re-sync from public L1s; light client verifies continuity.

---

## 11) Compliance & Governance Hooks
- Directory policies: min stake, ASN/region diversity, client plurality.
- Agreements encode compliance constraints (GDPR/HIPAA redactions).
- Evidence: receipts + anchors; exportable for audits.

---

## 12) Deliverables
- Helm charts: validators, relays, DA pinner, mainnet-mini, core services.
- Terraform modules: VPC, subnets, NAT, k8s clusters, load balancers, KMS.
- Runbooks: onboarding, incident, upgrade, key rotation.
