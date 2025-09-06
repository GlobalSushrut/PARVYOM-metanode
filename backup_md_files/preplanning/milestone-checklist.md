# Milestone Checklist (Acceptance-Driven)

Mark each item as you validate it in CI or on testbeds.

---

## M1 — CLI & Devnet
- [ ] bpi init scaffolds project (bpicompose.yml, samples)
- [ ] bpi up launches 3-validator devnet
- [ ] bpi deploy <svc> updates running image
- [ ] bpi verify confirms header chain + BLS commit
- [ ] Finality p50 < 1s (dev machine)
- [ ] Light client verify < 2ms/header
- [ ] Golden path: init → up → request → verify

## M2 — DockLock & Agreements
- [ ] Seccomp denies nondeterministic syscalls (e.g., gettimeofday, rdtsc)
- [ ] agreementc build (YAML → WASM) success
- [ ] bpi agreement simulate dry-run
- [ ] 1000 replays → identical Merkle roots
- [ ] Receipts sealed; proofs verify in CI

## M3 — Data Availability & Anchors
- [ ] RS sharding at edges (k/n, sizeKB) implemented
- [ ] Validator sampling RPC + proofs
- [ ] Challenge market operational; failed response ⇒ slash
- [ ] Anchors to two L1s with reorg handling
- [ ] Light client verifies DA samples + anchors
- [ ] DA success ≥ 99.9%/epoch (testbed)

## M4 — K8s Two-Cluster UX
- [ ] deploy k8 + connect k8 testnet commands work
- [ ] Auto validator triplet per tenant
- [ ] Resource targets respected (validator ~180Mi, pinner ~150Mi, relay ~100Mi)
- [ ] One-command onboarding to public testnet

## M5 — Inclusion & Force-Inbox
- [ ] VRF proposer selection; rotating inclusion committee
- [ ] Inclusion list enforcement; slashing on misses
- [ ] Force-inclusion inbox anchored on L1
- [ ] Chaos tests demonstrate censorship resistance

## M6 — Marketplace & Docs
- [ ] Validator marketplace + dashboards
- [ ] Example apps (AI, files, payments)
- [ ] 5–10 external teams onboarded; case studies

## M7+ — Open Onboarding & DAO
- [ ] ≥ 20 validators (≥ 10 orgs)
- [ ] ≥ 2 independent client implementations
- [ ] Permissionless validator entry; on-chain parameterization

---

## Testing Matrix (Quick Pointers)
- Performance: finality latency, throughput, resource profiles
- Security: Byzantine behaviors, crypto checks, sandbox escapes
- Integration: cross-chain anchors, disaster recovery, upgrades
- UX: onboarding time, error clarity, docs completeness, CLI latency
