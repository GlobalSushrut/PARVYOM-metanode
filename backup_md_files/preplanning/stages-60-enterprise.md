# Metanode / BPI Mesh — 60-Stage Enterprise Build Plan
Enterprise-grade breakdown from bootstrap to mainnet, with tests and exit criteria per stage. Use with concept.md, logic.md, infra-architecture.md, planning.md, and how-to-build.md.

Legend: For every stage, run Functional, Determinism, Security, Performance, and Docs checks where applicable.

---

## Phase A — Foundations (Stages 1–8)

1) Repo Bootstrap & CI
- Objective: Standard repo layout, CI lint/test, pre-commit
- Scope: mono-repo, language toolchains, Makefile, pre-commit, CI pipelines
- Deliverables: repo skeleton, CI green on PR
- Tests: CI runs lint/unit; branch protection
- Exit: 100% green on main; CONTRIBUTING.md present

2) Canonical Encoding Library
- Objective: CBOR/Protobuf fixed-order encoders
- Scope: enc()/dec(), domain-separated hashing integration
- Deliverables: enc lib, test vectors
- Tests: cross-lang equivalence; golden vectors
- Exit: encodings match vectors across Rust/Go

3) Hash & Merkle Library
- Objective: BLAKE3-256/SHA-256 + canonical binary Merkle
- Scope: leaf/node functions; odd-duplicate rule
- Deliverables: lib + CLI merkle tool
- Tests: RFC vectors; large-tree fuzz
- Exit: 10k-tree fuzz pass; perf < 1ms/1k leaves

4) Keys & Signatures (Ed25519)
- Objective: Ed25519 identities; mTLS bootstrap
- Scope: keygen, sign/verify, SPIFFE IDs wiring
- Deliverables: key tool, CSR flow (dev)
- Tests: negative signature tests; rotation
- Exit: rotate w/o downtime in devnet

5) BLS Aggregate Verify (G1 sig, G2 pk)
- Objective: Minimal-sig BLS aggregate verification
- Scope: blst binding; multi-scalar add; batch verify
- Deliverables: bls-agg lib
- Tests: IETF vectors; 10k random aggregates
- Exit: verify ≤ 2ms on laptop per block

6) VRF Library
- Objective: EC-VRF outputs (π, β) for leader/ticks
- Scope: Ed25519 or BLS-VRF; API and vectors
- Deliverables: vrf lib + vectors
- Tests: determinism; invalid proof reject
- Exit: cross-lang compatibility

7) AEAD & KDF Primitives
- Objective: XChaCha20-Poly1305; HKDF-SHA256
- Scope: encrypt/decrypt; context strings
- Deliverables: aead lib; kdf helpers
- Tests: Wycheproof; nonce misuse negative
- Exit: pass all crypto test suites

8) Noise over QUIC Scaffold
- Objective: QUIC transport with Noise(XX) profile
- Scope: handshake; channel abstraction
- Deliverables: net lib; examples
- Tests: interop; packet loss 10% sim
- Exit: stable under 5% loss, 1000 msgs

---

## Phase B — Consensus & Headers (Stages 9–16)

9) Header Struct & Hashing
- Objective: Define/serialize header and header_hash
- Scope: fields per logic.md; versioning; hashing
- Deliverables: header lib + vectors
- Tests: merkle roots consistent; hash vectors
- Exit: round-trip stable; vectors fixed

10) IBFT Message Types
- Objective: PRE-PREPARE, PREPARE, COMMIT models
- Scope: wire types; signature bindings
- Deliverables: ibft types lib
- Tests: serialization fuzz; replay guards
- Exit: pass AFL fuzz 1h no crash

11) Validator Set Map & Hash
- Objective: Merkle-map of index→pubkey+meta
- Scope: updates, epoch rotation, hashing
- Deliverables: set lib
- Tests: update proofs; tamper detect
- Exit: O(log N) proofs verified

12) Leader Selection via VRF
- Objective: deterministic proposer per (h, r)
- Scope: seed, modulo selection, tie rules
- Deliverables: proposer module
- Tests: distribution uniformity (chi-square)
- Exit: p>0.05 for uniformity across 1e6 draws

13) BLS Commit Object
- Objective: aggregate signatures + bitmap
- Scope: aggregation, bitmap compress
- Deliverables: commit lib
- Tests: invalid signer detection
- Exit: wrong bitmap rejected reliably

14) Safety Slashing Proofs
- Objective: equivocation detection proofs
- Scope: two commits, overlap analysis
- Deliverables: slashing proof type
- Tests: unit proofs; adversarial cases
- Exit: light client verifies proof

15) Header Pipeline (3-node devnet)
- Objective: produce/finalize headers
- Scope: PRE-PREPARE→COMMIT; pipelining
- Deliverables: validator microservice
- Tests: finality under 250ms block time
- Exit: ≥2f+1 commits consistently

16) Light Client Verify (Headers)
- Objective: verify prev_hash + BLS + set hash
- Scope: lc-verify CLI
- Deliverables: lc-verify binary
- Tests: 1k headers < 2s; negative cases
- Exit: laptop verify <2ms/header p50

---

## Phase C — BPCI Data Plane (Stages 17–22)

17) BPCI Frame & Header Auth
- Objective: define frame; src sig over header
- Scope: nonces, svc_id_hash, PoH tick ref
- Deliverables: bpci lib + vectors
- Tests: replay rejection; AEAD AD bind
- Exit: monotonic counters enforced

18) E2E Key Agreement
- Objective: X25519→HKDF AEAD keys
- Scope: service key publish; ephemeral derivation
- Deliverables: key mgmt + bpci integration
- Tests: KCI resistance; rekey
- Exit: per-session keys; pfs verified

19) Relay Service (QUIC/MASQUE)
- Objective: multi-relay anti-eclipse broadcast
- Scope: fan-out; dedup; rate-limit
- Deliverables: relay microservice
- Tests: 30% loss; partition heal
- Exit: recover within 2 blocks post-heal

20) Gateway Agent
- Objective: client SDK + sidecar for apps
- Scope: send/recv BPCI; retries; metrics
- Deliverables: agent + docs
- Tests: backpressure; idempotency
- Exit: 10k req/min stable on dev box

21) Encrypted Mempool
- Objective: leader-ephemeral encryption
- Scope: tx encrypt; post-proposal reveal
- Deliverables: mempool module
- Tests: decryption ordering; DoS limits
- Exit: decrypt rate ≥ block throughput

22) Inclusion Lists (Data Plane)
- Objective: inc_root maintenance
- Scope: pending obligations; reject proofs
- Deliverables: inclusion module
- Tests: missed-inclusion detection
- Exit: slashable evidence produced

---

## Phase D — PoH & Ticks (Stages 23–24)

23) Nonce Chain & Tick Derivation
- Objective: NC chain; tick H(0x12||seed||NC)
- Scope: VRF seed; per-sender NC
- Deliverables: poh lib
- Tests: recomputation determinism
- Exit: light client replays ticks

24) PoH Root in Headers
- Objective: Merkle root of ticks per block
- Scope: integration with proposer
- Deliverables: header+PoH integration
- Tests: malformed tick detection
- Exit: lc-verify checks PoH root

---

## Phase E — DockLock & Receipts (Stages 25–32)

25) Determinism Cage
- Objective: seccomp/AppArmor denylist
- Scope: syscall filters; RNG seed injection
- Deliverables: docklock runtime
- Tests: nondet syscalls blocked
- Exit: determinism CI passes 1k replays

26) Canonical Event Stream
- Objective: event ordering + Merkle root
- Scope: CES structs; hash roots
- Deliverables: ces lib
- Tests: reorder detection
- Exit: root changes on reorder

27) Witness Log & I/O Recording
- Objective: capture inputs/nondet outputs
- Scope: witness Merkle; compression
- Deliverables: witness lib
- Tests: missing witness fail
- Exit: receipts reject without witness

28) Receipt Structure & Signing
- Objective: record schema + Ed25519
- Scope: run_header, trace_roots, policy
- Deliverables: receipts lib
- Tests: signature verify; tamper detect
- Exit: receipts_root stable in header

29) Policy Engine (WASM)
- Objective: pre/post hooks, deterministic
- Scope: ABI; read-only host APIs
- Deliverables: agreements SDK
- Tests: sandbox escape attempts
- Exit: policies pure/deterministic

30) ZK Claim Hooks (optional)
- Objective: integrate SNARK verification
- Scope: bounds/set-membership proofs
- Deliverables: zk module
- Tests: invalid proofs rejected
- Exit: optional path flagged in ctx.zk

31) Receipt Registry Facade
- Objective: query receipts by id
- Scope: indexing; pagination; auth
- Deliverables: registry svc
- Tests: consistency with headers
- Exit: hash matches receipts_root

32) Shadow Receipts (Gateway)
- Objective: acting-as signatures for legacy
- Scope: proxy identity; audit fields
- Deliverables: gateway receipts
- Tests: chain-of-custody preserved
- Exit: verifiable delegation metadata

---

## Phase F — Data Availability (Stages 33–40)

33) RS Encoding at Edge
- Objective: RS(k,n) per chunk
- Scope: 256–512KB chunks; params
- Deliverables: rs-encode lib
- Tests: recover from any k
- Exit: bit-exact reconstruction

34) Shard Headers & da_root
- Objective: shard_header hashing
- Scope: index, size, height, content hash
- Deliverables: da structs
- Tests: merkle inclusion proofs
- Exit: da_root verified by lc

35) DA Pinner Service
- Objective: pin/store shards with policy
- Scope: backends S3/GCS/Azure/IPFS/Filecoin
- Deliverables: pinner svc
- Tests: SLA probes; repair
- Exit: 99.9% probe success in dev

36) DA Sampler
- Objective: s-sample per validator
- Scope: seed schedule; challenge proto
- Deliverables: sampler tool
- Tests: withheld fraction detection
- Exit: P_miss < 2^-32 with params

37) DA Challenge & Slashing
- Objective: enforce failures to signers
- Scope: attribution; windows; proofs
- Deliverables: DA slashing logic
- Tests: simulated withholders
- Exit: slashable proofs assembled

38) Multi-Cloud Storage Policy
- Objective: shard placement + encryption
- Scope: keyslots; provider diversity
- Deliverables: placement engine
- Tests: provider outage drills
- Exit: tolerate ≥2 provider loss

39) CAR/DAG Packaging
- Objective: object packaging for IPFS/Filecoin
- Scope: CAR files; CIDs mapping
- Deliverables: car toolchain
- Tests: re-fetch & verify roots
- Exit: da_root ties to CAR set

40) DA Observability
- Objective: dashboards + alerts
- Scope: probe %, slashes, storage health
- Deliverables: DA dashboards
- Tests: synthetic failures visible
- Exit: alert within 5m of issue

---

## Phase G — Censorship Resistance & Anchors (Stages 41–46)

41) Inclusion Lists (Consensus Rule)
- Objective: proposer must include or reject
- Scope: K-window rule; evidence format
- Deliverables: consensus rule impl
- Tests: byzantine proposer sim
- Exit: violation yields proof

42) Force-Inclusion Inbox
- Objective: L1-anchored delayed inbox
- Scope: commit hash; timers
- Deliverables: inbox client + watcher
- Tests: two L1 stubs in devnet
- Exit: inclusion forced post-Δ

43) Encrypted Mempool Finalization
- Objective: key reveal post-proposal
- Scope: epoch keys; reveal protocol
- Deliverables: mempool finalize
- Tests: lost key recovery
- Exit: no stuck tx due to reveal

44) External Anchor Client
- Objective: post header hash to L1s
- Scope: 2 chains; retries; receipts
- Deliverables: anchor client
- Tests: gas spikes; reorg coping
- Exit: anchor every T_anchor on-time

45) Anchor Verification in LC
- Objective: prefer anchored headers
- Scope: last confirmed anchor rule
- Deliverables: lc anchor check
- Tests: deep reorg sim
- Exit: lc refuses pre-anchor reorgs

46) Slashing Evidence Export
- Objective: standardized proofs
- Scope: equivocation, DA, inclusion, anchor
- Deliverables: evidence API
- Tests: verifiable by third parties
- Exit: proofs portable and minimal

---

## Phase H — Networking & Core (Stages 47–52)

47) Relay Diversity Controls
- Objective: multi-ASN/region relays
- Scope: policy; health score; routing
- Deliverables: relay policy engine
- Tests: relay failure rotation
- Exit: maintain throughput under 1 relay loss

48) Directory Service & Diversity Policy
- Objective: validator directory + policy
- Scope: ASN/region/client diversity
- Deliverables: directory svc
- Tests: policy violations blocked
- Exit: validator_set_hash reflects policy

49) Headers Proxy
- Objective: stateless header relay for Core
- Scope: GET/stream headers
- Deliverables: headers-proxy svc
- Tests: back-pressure; cache
- Exit: 10k headers/min stable

50) Billing Meter → Settlement Hash
- Objective: usage metering to commitment
- Scope: Merkle commit; export API
- Deliverables: billing svc
- Tests: tamper-evidence
- Exit: commitments reproducible

51) Faucet Service
- Objective: testnet token dispenser
- Scope: rate-limit; CAPTCHA; API
- Deliverables: faucet svc + UI
- Tests: abuse sim; quotas
- Exit: 99% requests < 2s

52) Governance Scaffolding
- Objective: parameters via DAO config
- Scope: fee splits; emissions
- Deliverables: governance module
- Tests: param updates reflected
- Exit: on-chain/off-chain sync

---

## Phase I — CLI, DX, and Docs (Stages 53–56)

53) `bpi` CLI Core
- Objective: init, up, verify, receipts
- Scope: consistent UX; exit codes
- Deliverables: bpi binary
- Tests: golden-path demo < 60s
- Exit: demo script reliable on fresh machine

54) Agreement Tooling `agreementc`
- Objective: build, pin, simulate
- Scope: WASM bundles; registry ops
- Deliverables: agreementc binary
- Tests: deterministic simulation
- Exit: identical outputs across runs

55) Examples & Tutorials
- Objective: hello-world API, video ingest, compliance example
- Scope: end-to-end guides
- Deliverables: examples/*, docs
- Tests: CI runs examples
- Exit: green examples CI pipeline

56) Architecture Pack (PDF)
- Objective: printable pack with diagrams
- Scope: concept/logic/infra/addons condensed
- Deliverables: pdfs/*
- Tests: link checks; diagram renders
- Exit: investor/operator-ready pack

---

## Phase J — Observability, Security, Launch (Stages 57–60)

57) Observability Suite
- Objective: Prometheus, Loki, OTEL, dashboards
- Scope: SLOs; alerts
- Deliverables: helm charts; dashboards
- Tests: chaos drills visible
- Exit: alert within 5m; runbooks linked

58) Security Hardening & Threat Model
- Objective: finalize threat model; hardening
- Scope: CIS benchmarks; SBOM; image signing
- Deliverables: SECURITY.md; hardening guides
- Tests: kube-bench; trivy; signing verify
- Exit: clean scans; exceptions documented

59) External Audits
- Objective: cryptography + infra audits
- Scope: scope docs; fix findings
- Deliverables: audit reports; fixes
- Tests: regression tests added
- Exit: high/critical issues resolved

60) Public Testnet → Mainnet Launch
- Objective: staged launch
- Scope: validators on-board; governance live
- Deliverables: launch runbook; announcements
- Tests: soak 2 weeks; SLO adherence
- Exit: Go/No-Go checklist complete; mainnet genesis

---

## Global Testing Matrix (applies throughout)
- Functional: unit/integration/e2e
- Determinism: 1k-run replay harness (DockLock)
- Security: fuzzing, property tests, negative cases
- Performance: p50/p95 latency budgets; QPS targets per component
- Interop: at least two client builds where feasible
- Docs: updated READMEs, runbooks, API references per change

## Governance & Compliance Gates
- Diversity policy checks each epoch
- Evidence export APIs for audits
- Anchors posted within SLA; discrepancies escalated

## Change Management
- Semantic versioning; release notes
- Canary deployments; feature flags
- Rollback procedures and postmortems
