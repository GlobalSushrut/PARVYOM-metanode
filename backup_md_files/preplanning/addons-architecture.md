# Metanode Web 3.5 – Add-On Architecture Document
Version 1.0 — August 2025

A single reference for all add-ons and expansions complementing the core Web 3.5 architecture. Pairs with concept.md and logic.md.

---

## 0. Core Vision — Web 3.5
Run Web2 apps unchanged (Docker/K8s) with Web3 security (finality, receipts, data availability).

- Header-only consensus: IBFT + BLS aggregates → sub-second finality
- Mesh data-plane: BPCI QUIC/HTTP3 transport with anti-eclipse multi-relay
- Immutable policy over mutable services: Agreements (YAML→WASM) enforce rules
- Zero-config DX: one YAML (bpicompose.yml), one CLI (bpi)

---

## 1. Gateway Proxy with BlockchainSocket
Purpose: Bridge Web3 ↔ Web2 securely when one side has no Metanode.

Components:
- DockLock Gateway next to legacy app
- BlockchainSocket overlay for control-plane: nonce + PoH tick + AEAD + BLS signature
- Shadow receipts: proxy signs for legacy app with “acting-as” identity
- DA pinner for payload chunks
- Video: WebRTC SFU inside gateway with rolling receipts

Benefits:
- Web2 apps get verifiable proofs without code changes
- Enables regulated industries to interoperate

---

## 2. Cross-Communication Protocol (Side-Net Mesh)
Purpose: Allow organizations to run their own BPCI side-net fully connected to mainnet.

Components:
- Internal validators + DA pinners + relays
- Header peering to mainnet every N blocks
- Anchors to public L1s
- Cross-mesh relays verify proofs before forwarding

Benefits:
- Org traffic stays private; proofs verifiable on mainnet
- Failover and redundancy across networks

---

## 3. New Mining Model
Purpose: Reward all useful roles, not just validators.

Roles:
- Validators: header finalization, inclusion compliance
- Relays: move encrypted frames quickly
- DA pinners: store shards & pass probes
- Replayers: cross-attest peer blocks
- Anchors: post to external L1s

Reward Split (example): validators 45%, relays 25%, DA 20%, replayers 7%, anchors 3%
Slashing: equivocation, DA failure, censorship

---

## 4. Storage L1 over Filecoin + Multi-Cloud
Purpose: Make db layer the L1 of storage — Filecoin for cold permanence, multi-cloud for hot speed.

Features:
- RS(k/n) encoding; shards across AWS, GCP, Azure, OVH, Backblaze, Filecoin, Storj, IPFS
- DA proofs tie storage to block headers
- Encryption with per-tenant keyslots
- Auto-repair from surviving shards

Benefits:
- Cloud speed + blockchain proof
- Survives multiple provider failures

---

## 5. True Multi-Cloud Decentralization
Purpose: Use centralized clouds without centralizing trust.

Mechanism:
- Each shard stored on independent providers
- CIDs + Merkle roots committed on-chain
- DA probes detect failure or tampering
- Providers (or assigned pinners) have stake slashed for faults

Outcome:
- Even if large providers fail, data can be rebuilt from remaining shards

---

## 6. Cosmos CDN
Purpose: Decentralized content discovery, storage, and delivery (e.g., YouTube-class).

Components:
- Ingest gateways + transcoders
- Edge caches (Cosmos CDN nodes) serving HLS/DASH/WebRTC
- Discovery via DHT-like index
- Receipts for each segment served

Benefits:
- Fast edge delivery with proofs
- Monetization for creators, edges, and transcoders
- Anti-censorship & compliance via Agreements

---

## 7. Faucet Service
Purpose: Free token dispenser for testnet.

Features:
- Web/CLI/API to request testnet BPI
- Rate-limiting, CAPTCHA to prevent abuse
- Mints or transfers tokens instantly

Benefits:
- Easy onboarding for devs
- Enables 60-second “init → deploy → verify” demo

---

## 8. Developer Experience Enhancements
Purpose: Make Metanode adoption frictionless.

Features:
- Single CLI for all roles (bpi)
- One-file config (bpicompose.yml)
- Zero-config devnet identical to testnet/mainnet
- Agreement lifecycle: build, pin, simulate

Benefits:
- Fast learning curve for Web2 devs
- Reduced cost of migration

---

## 9. Compliance & Governance
Purpose: Keep decentralization + enterprise readiness.

Components:
- Directory DAO enforces ASN/region diversity
- On-chain governance for fee splits & emissions
- Agreements for compliance rules (GDPR, HIPAA)
- Anchors to external L1s for legal auditability

Benefits:
- Meets regulatory standards
- Ensures network cannot be captured by one operator

---

## 10. Strategic Moat
Why Metanode is Unreplaceable:
- Zero-rewrite Web2 adoption + Web3 guarantees
- Cross-domain: works for Web2 apps, Web3 dApps, hybrid workloads
- Integrated DA L1 + multi-cloud hot cache
- Cross-replay mining for historical immutability
- Cosmos CDN as a native module
- Developer-first UX

Position: The default runtime for Web 3.5 — where Web2 UX meets Web3 trust.

---

## Launch Order (suggested)
1. Core Metanode + devnet CLI
2. Faucet + example workflows
3. Gateway Proxy with BlockchainSocket
4. Side-net cross-mesh + peering
5. Storage L1 + multi-cloud integration
6. Public testnet with mining roles live
7. Cosmos CDN beta
8. Governance DAO + external anchors

---

Notes:
- See concept.md and logic.md for core architecture and verification rules.
- This document is additive: it layers on top of header-only consensus, DA sampling, and deterministic receipts.
