# Metanode / BPI Mesh — Concept
A deliberate, high-level narrative for why and what to build. Pairs with logic.md for exact math and wire rules.

---

## Core idea
Run unmodified Web2 apps (Docker/K8s) while giving Web3-grade guarantees: fast finality, immutable receipts, data availability, staking/slashing, and credible neutrality. Separate concerns: apps execute locally in containers; only succinct facts (headers) go through consensus.

---

## What users experience
- A Docker-like DX with a one-file `bpicompose.yml` and a `bpi` CLI.
- Sub-second finality, verifiable receipts, and easy verification via a light client.
- Policies (Agreements) in YAML compiled to deterministic WASM; optional plugins.

---

## What the system actually does
- DockLock wraps containers and emits canonical, deterministic receipts (Merkle roots + optional ZK claims).
- Header-only IBFT consensus finalizes only block headers (roots and commitments) with BLS aggregate signatures.
- Proof-of-History (PoH) binds request ordering without trusting clocks.
- Data Availability (DA) is provided at the edges by tenants via Reed–Solomon; validators/challengers sample and slash failures.
- Anti-censorship via VRF-selected proposer, inclusion lists, encrypted mempool, and a force-inclusion inbox anchored to L1s.
- Light clients verify headers in milliseconds using BLS aggregates, PoH ticks, DA samples, inclusion evidence, and external anchors.

---

## Trust model and threat stance
- Partial synchrony; up to f of N validators are Byzantine. No trusted clocks.
- Safety from IBFT; availability from sampling + slashing; neutrality from inclusion rules and L1-anchored inbox.
- Keys rotate routinely; the 2 GB "core" is stateless (no validators/DA), so it is easily replaceable.

---

## Why this shape
- Performance and cost: don’t replicate app execution globally. Replicate only the facts (roots and proofs) with cryptographic accountability.
- DX-first: if `bpi init → bpi up → bpi verify` isn’t smooth, adoption stalls.
- Evolvability: lanes (e.g., EVM/WASM) are optional sidecars, not the base.

---

## What to ship first
1) Header-only IBFT + light client (devnet).
2) BPCI overlay (QUIC + AEAD + replay rules).
3) PoH ticks + receipts root from DockLock.
4) DA encoding at edges + sampling; anchors to 2 L1s.
5) Inclusion lists + force-inclusion inbox.
6) CLI (`bpi`) and Agreement toolchain (`agreementc`).

---

## Success criteria (north star)
- p50 finality < 1s; light client verifies < 2 ms/header.
- Onboarding in < 30 minutes; deploy cost ≈ Docker.
- Deterministic receipts, verifiable in CI and by anyone via the light client.

---

## Pointers
- See `logic.md` for exact encodings, domain separations, cryptographic checks, slashing rules, and pseudocode.
- See `starter-repo.md` for a minimal scaffold; `milestone-checklist.md` for acceptance-driven execution.
