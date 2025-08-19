# Web 3.5: A Conscious Architecture
A practical, security-forward pathway to run unmodified Web2 apps with Web3 guarantees. Written deliberately to expose intent, tradeoffs, and checkpoints.

---

## Why this exists (Thesis)
- Most apps don’t need a global VM. They need accountable execution, fast finality, durable evidence, and credible neutrality.
- We separate execution (containers) from consensus (headers). We make facts—not app logic—global. This keeps performance high and costs sane.
- Developers get Docker-like DX with cryptographic receipts, DA, and slashing-backed guarantees.

Success looks like this: p50 finality <1s, receipts verifiable by a 2ms light client, onboarding in <30 minutes, and a deployment flow that feels like Docker Compose.

---

## What we’re building (Essentials)
- DockLock: deterministic container fence + Canonical Event Stream → receipts with Merkle roots and optional ZK claims.
- Header-only IBFT with BLS aggregation: sub-second finality without forcing global app execution.
- PoH + DA: cheap ordering signals and probabilistic availability, enforced via sampling and slashing.
- Agreements: human-first policy DSL compiled to WASM (pre/post). Determinism is enforced.
- Light client: millisecond verification of headers, DA samples, and anchors.
- Anchors to public L1s: periodic economic finality.
- CLI + one-file config: bpicompose.yml and bpi commands for dev→prod.

---

## How it works (Flow of a request)
1) App request hits service wrapped by DockLock. Pre-policy decides allow/deny.
2) Execution runs in a determinism cage (seccomp, time/rng virtualization). IO is witnessed.
3) Outputs + events are sliced, committed into a Merkle forest, optionally proven (ZK). A DockLock Record is sealed and signed.
4) Receipt headers (not full logs) enter the mempool. Inclusion/integrity is protected by VRF leader + inclusion list rules.
5) Header-only IBFT finalizes the block. Validators sample DA shards. Failures can be challenged and slashed.
6) Light clients verify: BLS aggregate, PoH ticks, DA samples, and periodic L1 anchors.

---

## Guardrails (Security + Integrity)
- Determinism by default: deny nondeterministic syscalls; fixed RNG seeds; virtualized clocks.
- Cryptographic binding: SHA-256/Merkle are primary. Any knot digest is secondary.
- mTLS via SPIFFE/SVID; regular key rotation (daily SVID, weekly service keys, monthly BLS epoch).
- Censorship resistance: inclusion lists, VRF proposer, multi-relay gossip, and a force-inclusion inbox on L1.

---

## Minimal viable stack (M1→M3)
- M1 Devnet + CLI: bpi init/up/deploy/verify. 3-validator IBFT (headers only). Light client verifies in <2ms/header.
- M2 DockLock + Agreements: YAML→WASM policies, receipts, replay-determinism in CI.
- M3 DA + Anchors: RS sharding at edges, validator sampling + challenge market, anchors to two L1s.

Definition of Done (per milestone) lives in the checklist below.

---

## Developer Experience (DX)
- You write bpicompose.yml once. You run bpi. You deploy like Docker. Policies live next to services.
- Core commands: init, up, deploy <svc>, verify, receipts get <id>, agreement build|pin|simulate, mainnet scale, deploy k8, connect k8 testnet.
- Tight error surfaces: explicit exit codes (0 success; 10 policy deny; 20 consensus fail; 30 DA fail; 40 anchor fail).

---

## Starter repository (shape)
```
bpi-starter/
├─ bpicompose.yml
├─ agreements/
│  ├─ basic.v1.yaml
│  └─ plugins/
│     └─ rate_limiter.rs
├─ services/
│  └─ hello-world/
│     ├─ Dockerfile
│     ├─ main.py
│     └─ requirements.txt
└─ docs/
   ├─ getting-started.md
   └─ architecture.md
```
- Compose expresses project, mainnet, agreements, and services.
- Agreements compile to WASM bundles; simulate locally before pinning.
- Hello-world gives a golden path for demo, CI, and tutorials.

---

## Conscious tradeoffs
- Not a global VM: we gain performance and cost predictability, but complex app-side state machines remain app-local. We compensate with receipts, DA, and anchors.
- Header-only consensus: finality is fast; full execution is not replicated. Verification relies on receipts + DA sampling + replay determinism.
- Determinism strictness: some legacy apps won’t run without minor tweaks (time/rng). We provide shims and guidance.

---

## Milestone checklist (acceptance-driven)

### M1 — CLI & Devnet
- [ ] `bpi init` scaffolds bpicompose.yml + samples
- [ ] `bpi up` boots 3-validator devnet locally
- [ ] `bpi deploy <svc>` rolls a new image
- [ ] `bpi verify` confirms BLS commit + headers chain
- [ ] p50 finality <1s on dev machine; light client <2ms/header
- [ ] Golden-path e2e: init → up → request → verify

### M2 — DockLock & Agreements
- [ ] Seccomp profile blocks nondeterministic syscalls
- [ ] YAML→WASM `agreementc build` + `bpi agreement simulate`
- [ ] 1000 deterministic replays yield identical roots
- [ ] Receipts sealed; Merkle proofs verifiable in CI

### M3 — Data Availability & Anchors
- [ ] RS sharding at edges; manifest finalized
- [ ] Validator sampling RPCs + proofs; challenge market
- [ ] Anchors posted to two L1s with reorg handling
- [ ] Light client verifies DA samples + anchors
- [ ] DA success ≥99.9%/epoch (testbed)

### M4 — K8s Two-Cluster UX
- [ ] `deploy k8` + `connect k8 testnet` operational
- [ ] Auto validator triplet per tenant; sane resource limits
- [ ] One-command onboarding to public testnet

### M5 — Inclusion & Force-Inbox
- [ ] VRF proposer + rotating inclusion committee
- [ ] Inclusion list enforcement; slashing on misses
- [ ] Force-inclusion inbox anchored to L1
- [ ] Chaos runs: censorship-resistance holds

### M6 — Marketplace & Docs
- [ ] Community dashboard + validator marketplace
- [ ] Example apps (AI, files, payments)
- [ ] 5–10 external teams onboarded; case studies

### M7+ — Open Onboarding & DAO
- [ ] ≥20 validators (≥10 orgs), ≥2 client impls
- [ ] Permissionless validator entry; on-chain params

---

## What “done” feels like (north star)
- A new team can: init, up, deploy, verify in under 30 minutes.
- They get sub-second finality and receipts their lawyers and auditors can reason about.
- Their ops team can scale validators and DA confidence via configuration—not bespoke infra.
- And their users never learn a new mental model; it’s just a faster, safer internet underneath.

---

## Next steps (suggested order)
1) Lock the CLI skeleton + local devnet (M1).
2) Ship DockLock minimal receipt path + YAML→WASM compiler (M2).
3) Integrate DA sampling + two anchors + light client checks (M3).
4) Wrap in docs, dashboards, and K8s flows; enable public testnet onboarding (M4).

If you want, I can now: (a) scaffold the starter repo, (b) generate the initial bpicompose.yml + basic agreement, and (c) wire a tiny Flask service for the golden path.
