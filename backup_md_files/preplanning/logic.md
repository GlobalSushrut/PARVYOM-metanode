# Metanode / BPI Mesh — Logic (Math, Encodings, Verification)
Authoritative specification for cryptographic domains, canonical encodings, consensus rules, DA sampling, receipts, and light-client verification. Pairs with concept.md.

---

## 1) System model & threat assumptions
- Processes
  - Tenant Project cluster: DockLock-wrapped app containers.
  - Tenant Mainnet-mini: validators, relays, DA pinners.
  - Core (2 GB k3s): directory, PKI, gateway, headers-proxy, billing. No consensus/DA.
- Network: partial synchrony (eventual delivery < Δ).
- Adversary: up to f of N validators Byzantine; can DoS, censor, equivocate, withhold DA. No secure clocks.
- Goal: deterministic safety/finality (IBFT) + DA sampling + anti-censorship + light-client verifiability.

---

## 2) Cryptographic primitives & domain separation
- Hash H: BLAKE3-256 (preferred) or SHA-256 (FIPS). All domain tags are single-byte prefixes.
  - 0x00 Merkle leaf, 0x01 Merkle internal
  - 0x10 Header hash, 0x11 BPCI header hash, 0x12 PoH tick hash
  - 0x13 DockLock record hash, 0x14 DA shard header hash
- Merkle (binary, canonical)
  - Leaf(d) = H(0x00 || d)
  - Node(L,R) = H(0x01 || L || R)
  - Odd leaves: duplicate last once
- Signatures
  - Network/identity: Ed25519
  - Consensus: BLS12-381 minimal-sig: signatures in G1, pubkeys in G2
    - Aggregate verify: e(Σσ_i, G2) ?= e(H2C(header_hash), ΣPK_i)
- VRF: EC-VRF (RFC 9381) on Ed25519 or BLS-VRF; output (π, β) with β ∈ {0,1}^256
- AEAD: XChaCha20-Poly1305 for BPCI payloads
- KDF: HKDF-SHA256 with explicit context strings

---

## 3) Canonical encodings
- Use CBOR or Protobuf with fixed field ordering.
- Define enc(x) → bytes as the exact serialization for hashing/signing.
- Never use JSON for binding commitments.

---

## 4) BPCI frame (Blocked, Packeted, Clustered Information)
### 4.1 Layout
```
struct BPCI {
  u8    ver = 1;
  bytes src_cluster_id[16];
  bytes dst_cluster_id[16];
  bytes svc_id_hash[32];          // H(service FQDN)
  u64   nonce;                    // strictly increasing per (src,svc)
  bytes poh_tick[32];             // see §7
  bytes payload_ct[..];           // AEAD ciphertext
  bytes aead_tag[16];             // AEAD tag
  bytes sig_src[64];              // Ed25519 over header
}
```
- Header to sign: bpci_hdr = enc(ver, src, dst, svc_id_hash, nonce, poh_tick, len(payload_ct))
- sig_src = Ed25519.Sign(sk_src, H(0x11 || bpci_hdr))

### 4.2 E2E key agreement
- Service publishes X25519 static pubkey K_srv.
- Sender derives K = HKDF(X25519(sk_src_eph, K_srv), "BPCI-AEAD" || svc_id_hash).
- AEAD AD = bpci_hdr.

### 4.3 Replay protection
- Gateways enforce per-(src,svc) monotonic nonce. Reject nonce ≤ last_nonce with small out-of-order tolerance window.

---

## 5) Headers & IBFT (header-only)
### 5.1 Header
```
struct Header {
  u8    version;
  u64   height;
  bytes prev_hash[32];           // H(0x10 || enc(prev_header))
  bytes poh_root[32];            // Merkle of ticks
  bytes receipts_root[32];       // Merkle of DockLock record hashes or ZERO
  bytes da_root[32];             // Merkle of DA shard headers
  bytes xcmp_root[32];           // outbound queue root
  bytes validator_set_hash[32];  // Merkle-map root (index → PK)
  u8    mode;                    // 2 = IBFT
  u64   round;                   // IBFT round
}
```
- header_hash = H(0x10 || enc(header))

### 5.2 IBFT (N = 3f+1, Q = 2f+1)
- Leader per (height,round): VRF leader with β mod N
- Messages: PRE-PREPARE, PREPARE, COMMIT over header_hash
- Commit object
```
struct Commit {
  bytes header_hash[32];
  bytes bls_agg_sig[48];
  bytes bitmap[ceil(N/8)];
}
```
- Finality: valid Commit with ≥ Q signers (bitmap) and valid BLS aggregate
- Pipelining: overlap (h+1) while finalizing h

### 5.3 Slashing (safety)
- Equivocation proof: two Commits for same height but different header_hash with overlapping signers

---

## 6) Light client
Given (header, commit, prev_header) and directory’s validator_set_hash:
1. Link: header.prev_hash == H(0x10 || enc(prev_header))
2. Set: header.validator_set_hash == dir.current_set_hash
3. BLS: let S = bitmap signers, PK = Σ dir.pk[i], verify e(commit.sig, G2) == e(H2C(header_hash), PK)
4. PoH: recompute poh_root (§7)
5. DA: verify samples against da_root (§8)
6. Inclusion: verify inclusion rules (§11)
7. Anchors: ensure last external anchor ≥ height and hash matches when available

All true ⇒ header valid; ≥ last confirmed anchor ⇒ economically irreversible.

---

## 7) Proof-of-History (PoH) ticks
- Per sender maintain nonce chain NC: NC := H(NC || nonce), start NC = 0^32
- Seed for height h: seed_h = VRFβ(sk_vrf, enc(h || prev_tick)) (light clients need only β)
- Tick: tick = H(0x12 || seed_h || NC)
- Block poh_root = MerkleRoot([tick_i] for included BPCI)

---

## 8) Data Availability (DA)
### 8.1 Edge encoding
- Concatenate content into 256–512 KB chunks
- Reed–Solomon RS(k,n) → n shards, any k recover (e.g., 8/12)
- For shard S_j: shard_header = enc(index=j, size, block_height, content_hash); shard_hash = H(0x14 || shard_header)
- da_root = MerkleRoot([shard_hash_j])
- Shards pinned to DA pinners (redundancy policy)

### 8.2 Sampling & challenge
- For block h, each validator samples s indices with idx_i = H(seed || i) mod n
- Challenge requires (shard_header, shard_bytes) with Merkle opening to da_root
- Timeout τ; failed or invalid responses attribute to signers of header h ⇒ slash if t of s fail in window W

### 8.3 Detection probability
- With withheld fraction p and s samples: P_miss = (1 − p)^s. Choose s to make P_miss ≪ 2^-32.

---

## 9) DockLock deterministic execution & receipts
### 9.1 Canonical Event Stream (CES)
```
Event = (eid: u128, parent: u128?, t_seq: u64, kind: ENUM, payload_commit: bytes32)
```
- t_seq: monotonic logical sequence (not wall-clock)
- Merkle root over enc(Event) in t_seq order

### 9.2 Determinism cage
- seccomp deny: gettimeofday, clock_gettime(realtime), rdtsc, getrandom (unless seeded), fork/exec (unless allowed)
- Fixed RNG seed injection (committed in record)
- I/O witness: record input streams and nondet syscall results; Merkle-ize witness log

### 9.3 Receipts
```
DockLockRecord {
  run_header: { node_id, service_id, agreement_id, input_root, rng_seed, env_root, ts_logical }
  trace_roots: { event_root, output_root, state_delta_root }
  policy: { pre_decision, post_decision, obligations_root, witness_root, zk_claims_root? }
  sig_node: Ed25519 over H(0x13 || enc(record))
}
```
- receipts_root = MerkleRoot(H(0x13 || enc(record))) across included runs
- Knot digest (braid invariant) may be included as auxiliary, never binding

---

## 10) Agreements (policy engine)
- Authoring: YAML DSL → WASM bundle; plugins (Rust/Go → WASM)
- ABI: deterministic pre(ctx), post(ctx) → Decision { allow, obligations }
- Host APIs (read-only): ctx.input/output/env/trace/zk/obligations
- No wall-clock/random/network inside policies
- Optional ZK claims: bounds via Pedersen commitments + SNARK π; set-membership via Merkle proofs; verified status exposed in ctx.zk

---

## 11) Censorship resistance
### 11.1 Inclusion lists
- Proposer of height h+K must include any BPCI observed by height h unless valid reject proof
- inc_root = MerkleRoot([H(bpci_hdr), decision]) for pending obligations
- Missing eligible items without reject ⇒ slashable for signers

### 11.2 Force-inclusion inbox
- Users can commit C = H(bpci_hdr) to Delay Inbox on two L1s
- After Δ seconds, validators must include the item; failure within W ⇒ slashable

### 11.3 Encrypted mempool
- Transactions encrypted to VRF leader’s ephemeral key for the block; leader reveals decrypt key post-proposal

---

## 12) Validator set, diversity, slashing
- Directory enforces per-epoch diversity: ≥R regions, ≥A ASNs, ≥2 client impls; optional HSM/TEE attestation
- validator_set_hash is a Merkle-map root of index → (pubkey, metadata)
- Slashable events: equivocation, DA failure, inclusion failure, anchor mismatch. All proofs verifiable by light clients.

---

## 13) External anchors
- Every T_anchor (≈300 s), any validator posts (height, header_hash) to two L1s
- Light clients prefer headers ≥ last confirmed anchor height; treat reorgs beyond anchors as economically infeasible

---

## 14) Performance targets
- IBFT O(N^2) messaging; with N≈9 acceptable
- BLS aggregation: 1 pairing verify per block
- Block time 200–300 ms; finality in 0.4–0.8 s typical
- Headers carry only roots, scaling dataplane independently

---

## 15) Light client pseudocode
```pseudo
function verify_bundle(bundle):
  (header, commit, prev) = bundle
  assert header.prev_hash == H(0x10 || enc(prev))
  assert header.validator_set_hash == dir.current_set_hash

  S = parse_bitmap(commit.bitmap)
  PK = sum_pubkeys([dir.pk[i] for i in S])
  msg = H2C(H(0x10 || enc(header)))
  assert pairing(commit.bls_agg_sig, G2) == pairing(msg, PK)

  ticks = reconstruct_ticks_from_index(header.height)
  assert MerkleRoot(ticks) == header.poh_root

  if header.da_root != ZERO:
    assert verify_da_samples(header.da_root, sampler_state)

  assert check_inclusion_rules(header)
  assert check_latest_anchors(header)
  return VALID
```
- Target: < 2 ms per header on laptop hardware

---

## 16) Economics (v1)
- Stake: validators bond BPI; relays/pinners smaller bonds
- Rewards per epoch: validators (signed blocks), relays (delivered GB), pinners (GB·day + successful challenges)
- Fees: protocol 1–3% on paid BPCI calls (0% dev)
- Settlement: off-chain meter → Merkle commitment → optional on-chain settlement

---

## 17) CLI UX (reference)
- bpi init | up | deploy <svc> | verify | receipts get <id>
- agreementc build | pin | simulate
- bpi mainnet scale --validators N --threshold T
- deploy k8 <name> | connect k8 testnet <addr>
- Exit codes: 0 ok; 10 policy deny; 20 consensus fail; 30 DA fail; 40 anchor fail

---

## 18) Defensive implementation guidance
- Hashing: constant-time libs, domain-separated prefixes, structured encodings
- Encodings: fixed-order CBOR/Protobuf; avoid ambiguous concatenations
- Randomness: approved CSPRNG; VRF for leader/indices; no clocks in proofs
- Key storage: HSM/TEE preferred; otherwise OS keystore; zeroize
- Networking: Noise(XX) over QUIC; disable TLS 1.2; prefer ChaCha20-Poly1305
- Config: fail-closed defaults, explicit feature flags, deterministic mode
- Testing: Byzantine sims (loss/duplication), determinism CI (1k replays), DA failure injection, censorship drills

---

## 19) Parameters (initial vs mature)
| Param | Early | Mature |
|---|---|---|
| Validators N | 9 | 25–49 |
| Faults f | 2 | 8–16 |
| Threshold Q | 6/9 | 2f+1 |
| Block time | 250 ms | 250–400 ms |
| DA shards (k,n,size) | (8,12,256KB) | (16,24,512KB) |
| DA samples/validator | 20 | 40–60 |
| Inclusion window K | 4 blocks | 6–8 |
| Force-inclusion delay | 60 s | 30–60 s |
| Anchor period | 300 s | 120–300 s |

---

## 20) Minimal viable core (build order)
1. Header-only IBFT (3 validators) + BLS agg + light client
2. BPCI overlay (QUIC, AEAD, nonce rules)
3. PoH ticks (VRF + recomputation)
4. DockLock receipts (determinism cage + receipts_root)
5. DA encoding at edges + sampling
6. External anchors (two L1s)
7. Inclusion lists + force-inclusion inbox
8. CLI (`bpi`) & agreements toolchain (`agreementc`)

---

## 21) End-to-end example
```bash
bpi init
bpi up
agreementc build agreements/acme.v1.yaml -o build/acme.v1.wasm
bpi agreement pin build/acme.v1.wasm
docklock run service api
curl -s "mainnet://registry.submit" -d '{"amount":420,"pii":"***"}'
bpi verify --last 10
bpi receipts get <run-id>
```
- Expected verify: ✓ prev_hash ✓ BLS ✓ PoH ✓ DA ✓ Inclusion ✓ Anchors

---

## 22) Formal appendices
### 22.1 BLS aggregate (minimal-sig)
- σ = Σσ_i in G1, PK = ΣPK_i in G2; check e(σ, G2) ?= e(H2C(header_hash), PK)

### 22.2 EC-VRF (digest)
- Given sk, input α, output (π, β); β seeds leader selection and PoH seed

### 22.3 Reed–Solomon (systematic)
- Split into k blocks; compute parity to n blocks; any k recover
- da_root binds shard headers via Merkle root

### 22.4 Inclusion lists (formal)
- G(h): set of BPCI headers gossiped by height h
- Rule: proposer of h+K includes x ∈ G(h) or provides Reject(x); absence + evidence ⇒ slash

---

## 23) Final notes
- Knot digest is auxiliary only; binding integrity is via canonical encodings + hash roots.
- Keep the 2 GB core stateless; never host consensus/DA there.
- Lead with DX: `bpi init → bpi up → bpi verify` must be boringly smooth.
