# Metanode / BPI Mesh — Project Planning
Milestone-driven plan with workstreams, acceptance criteria, owners, and timelines.

---

## 0) Principles
- Acceptance-first: every milestone has objective, demo, and exit criteria.
- DX-first: CLI and zero-config paths must work before scale features.
- Determinism and verifiability over features.

---

## 1) Workstreams
- Consensus & Light Client (C&LC)
- Data Availability (DA)
- DockLock & Agreements (DL&A)
- Networking & Relays (NET)
- Core & Directory (CORE)
- Tooling & CLI (CLI)
- Infra & Ops (INF)
- Docs & Examples (DOC)

---

## 2) Milestones (M1–M8)
- M1 Devnet Bootstrap (CLI, 3 validators, BLS agg verify)
  - Owners: C&LC, CLI
  - Exit: `bpi init → bpi up → bpi verify` shows BLS-verified headers
- M2 BPCI Overlay + PoH ticks
  - Owners: NET, C&LC
  - Exit: bpci send/recv with AEAD; PoH root validated by light client
- M3 DockLock Receipts + Agreements v1
  - Owners: DL&A
  - Exit: receipt with witness roots; agreement denies/permits deterministically
- M4 DA Edge Encoding + Sampling
  - Owners: DA
  - Exit: RS shards, sampling probes, light-client DA pass
- M5 Anchors + Inclusion Lists
  - Owners: C&LC, NET
  - Exit: periodic anchors to two L1s; inclusion rule enforcement & slashing proof
- M6 Testnet (N=9) + Faucet + Examples
  - Owners: INF, CORE, DOC
  - Exit: public testnet; faucet live; example apps deployed
- M7 Side-net Peering + Multi-cloud Storage
  - Owners: NET, DA, INF
  - Exit: side-net peered headers; storage across ≥3 providers
- M8 Governance & Emissions
  - Owners: CORE
  - Exit: DAO votes update fee splits; validator diversity policy enforced

---

## 3) Acceptance Matrix (per milestone)
For each milestone, must pass:
- Functional: feature works on happy-path
- Determinism: 1k run replay matches
- Security: fuzz + negative tests
- Performance: p50, p95 targets met
- Interop: across 2 client builds
- Docs: README, CLI help, runbook updated

---

## 4) Timeline (suggested)
- Month 1: M1–M2
- Month 2: M3–M4
- Month 3: M5 + public demos
- Month 4: M6 (testnet)
- Month 5: M7
- Month 6: M8

---

## 5) Risks & Mitigations
- BLS aggregate perf: use fast pairing libs; batch/parallel verify
- DA probe economics: tune s, τ, W; simulate adversaries
- Determinism regressions: CI replay harness; syscall denylist enforcement
- Censorship by proposer: inclusion lists + force-inclusion inbox; multi-relay
- Key management: HSM/TEE; strict rotation; threshold BLS

---

## 6) Deliverables per workstream
- C&LC: ibft-header lib, bls-agg verify, lc-verify tool
- DA: rs-encode lib, sampler, probe responder, DA verifier
- DL&A: docklock runtime, receipts lib, agreements SDK (WASM)
- NET: bpci client (QUIC), relay service, anti-eclipse
- CORE: directory svc, headers-proxy, billing hash
- CLI: `bpi` commands for init/up/verify/deploy/receipts
- INF: helm charts, terraform modules, observability dashboards
- DOC: tutorials, examples, architecture pack

---

## 7) Reporting & Cadence
- Standups: 3x/week
- Milestone review: end of each milestone with demos
- Metrics: weekly SLO report; bug triage board

---

## 8) Budget Signals (high-level)
- Engineering: 6–10 FTE
- Infra: $2–6k/mo testnet; $10–25k/mo mainnet pre-scale
- External audits: cryptography + infra hardening in Month 3–4
