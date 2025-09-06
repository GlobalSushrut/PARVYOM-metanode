# Pravyom V1 Blueprint

## 0) Topology (per app)

**5-node BPCI attachment (per application)**

* **BPI-comm-1** (gateway / cross-chain / PoE submitter)
* **BPI-val-1, BPI-val-2** (IBFT validators)
* **ENC-notary-1** (witness + receipts → logbook)
* **ENC-val-1** (enclave consensus for sensitive rules)

**Shared backbone**

* **BPCI-HQ**: final IBFT ring, Notary Committee, Validator Committee, Bank Mesh.
* **Court Node** (neutral infra): BISO policy engine + dispute/arbitration + audit subpoenas.
* **HTTP Cage**: unified ingress, signatures, rate/econ limits, audit headers.

---

## 1) Identity, Keys, and Proofs

* **Node IDs**: `BPI::<app>::{comm|valN}` and `ENC::<app>::{notary|val}`; `BPCI::{hq|validatorX}`; `COURT::node`.
* **Keys**:
  * Transport: Ed25519 (TLS/MTLS)
  * Block/consensus: BLS (aggregate signatures)
  * Proof receipts: Ed25519 on StepReceipts + Blake3 Merkle roots
* **Attestation**: ENC nodes expose SGX/SEV attestation or software-attested mode with nonce challenges (upgradeable).

---

## 2) Court-authored SmartContracts++ (YAML)

Court policies and agreements are **declared in YAML**, compiled to deterministic state machines. You can parse → validate → freeze → publish a hash to BPCI.

```yaml
# smartcontractspp/v1/agreements/appA.yaml
meta:
  id: AGR-APP-A-0001
  version: 1
  court_policy_hash: "blake3:c852...aa1e"
actors:
  - id: APP_A
    roles: [CALLER, PAYER]
  - id: BPI
    roles: [EXECUTOR]
  - id: BANK_MESH
    roles: [SPRAVYOMMENT]
resources:
  cpu_ms: {price_nex_per_unit: 0.00002, quota_per_min: 9_000}
  memory_mb_s: {price_nex_per_unit: 0.00001}
  storage_gb_day: {price_nex_per_unit: 0.001}
  egress_mb: {price_nex_per_unit: 0.00003}
  receipts_count: {price_nex_per_unit: 0.00005}  # incentivize full audit
rules:
  - id: R1-allowed-endpoints
    match:
      http.method: [GET, POST]
      http.path_regex: "^/api/v[0-9]+/(jobs|quotes|settle)"
    effect: ALLOW
  - id: R2-enc-only-for-sensitive
    match: { tag: "pii" }
    require: ["enc=true", "witness=enc-notary"]
  - id: R3-bank-quotas
    when: "billing.window(1h).spend_nex > 50_000"
    effect: THROTTLE
    params: {percent: 50}
poe:
  weights:
    cpu_ms: 0.35
    memory_mb_s: 0.15
    storage_gb_day: 0.15
    egress_mb: 0.15
    receipts_count: 0.20
  scale:
    cpu_ms: 1000
    memory_mb_s: 1000
    storage_gb_day: 1
    egress_mb: 10
    receipts_count: 100
settlement:
  aur_backing: "gold.oracle:LBMA-XAU"
  fee_split:
    locked: 0.002
    spendable: 0.003
    owner: 0.002
    treasury: 0.003
governance:
  upgrade_window_s: 86400
  quorum_pct: 66.7
  ibft_min_validators: 4
```

---

## 3) Receipts, Logbook, and PoE Bundle

### 3.1 StepReceipt (DockLock → ENC)

```json
{
  "v": 1,
  "app": "APP_A",
  "container": "job-9812",
  "op": "exec.start",
  "ts": "2025-08-13T06:15:22Z",
  "usage": { "cpu_ms": 412, "memory_mb_s": 265, "egress_mb": 1.2 },
  "labels": { "pii": "false", "region": "ca-east" },
  "prev_hash": "blake3:...",
  "hash": "blake3:...",
  "sig": "ed25519:..."
}
```

### 3.2 LogBlock (ENC-notary)

* Groups N StepReceipts → **Merkle root** → `log_block`.
* Notarized by ENC-notary (BLS or Ed25519), pushed to **BPI-comm** and **Blockbook**.

```json
{
  "v": 1,
  "app": "APP_A",
  "height": 421,
  "merkle_root": "blake3:...",
  "count": 512,
  "sig_notary": "bls:...",
  "range": { "from_ts": "...", "to_ts": "..." }
}
```

### 3.3 PoE Bundle (BPI-comm)

Compute Φ and Γ, include economic deltas, submit to **BPCI mempool**.

```json
{
  "v": 1,
  "app": "APP_A",
  "log_blocks": ["blake3:...", "blake3:..."],
  "usage_sum": { "cpu_ms": 201233, "memory_mb_s": 90012, "storage_gb_day": 3.2,
                 "egress_mb": 882.3, "receipts_count": 1024 },
  "phi": 0.8473,
  "gamma": 0.4589,
  "billing_window": "2025-08-13T06:00:00Z/2025-08-13T07:00:00Z",
  "sig_bpi_comm": "ed25519:..."
}
```

---

## 4) PoE Math (deterministic & stable)

Given per-window resource usage $u_i$, **weights** $w_i$ and **scales** $s_i$:

$$
\Phi(t) = \sum_i w_i \cdot \frac{u_i}{s_i}, \quad 0 \le \sum_i w_i = 1
$$

$$
\Gamma(\Phi) = \frac{\Phi}{1+\Phi} \in [0,1)
$$

* **NEX mint for window**:
  $\text{NEX}_\text{mint} = K_\text{window} \cdot \Gamma(\Phi) \cdot A$
  where $K_\text{window}$ is protocol emission scalar (governance),
  and $A$ is an adoption factor (bounded; from network growth oracle).

* **Fee split** (Court-published):
  * **0.2% Locked**, **0.3% Spendable**, **0.2% Owner**, **0.3% Treasury** (of gross fees in NEX).
    Owner's 0.2% acts as **fixed salary rail** within treasury budget rules.

---

## 5) IBFT Settings & BPI/BPCI Roles

* **BPI ring (per app)**: `BPI-val-1`, `BPI-val-2`, `ENC-val-1`, plus **one shared HQ validator** for 4-of-N (>⅔) quorum → <3s finality target.
* **BPCI HQ ring**: 4–7 validators (odd), block time target 1s, commit at 2+ rounds worst case <3s.
* **Validator set updates** gated by **Court quorum** (≥66.7%) and **upgrade window**.

**Mempool tuning**

* Max tx 256k, max bundle size 1–2 MB, priority: PoE bundles > settlements > info txs.
* Anti-spam: stake-weighted rate limits + HTTP Cage economic rate limiter.

---

## 6) HTTP Cage — Single Ingress

**Headers**: `X-Audit-Id`, `X-Actor`, `X-Sig-Ed25519`, `X-Nonce`, `X-Rate-Pass`
**Primary endpoints** (REST/gRPC mirrored):

* `POST /v1/receipts` → StepReceipt (DockLock → ENC)
* `POST /v1/logblocks` → ENC-notary → Blockbook + BPI-comm
* `POST /v1/poe/submit` → BPI-comm → BPCI mempool
* `GET /v1/blocks/{h}` → BPCI HQ
* `GET /v1/audit/{app}/range?from=&to=` → Blockbook
* `POST /v1/settlement/fiat` → Bank Mesh (ACH/SWIFT/wire)
* `POST /v1/governance/vote` → Court/BPCI proposal IDs

**Auth**: BJWT or short-lived Ed25519 tokens; signatures verified before queueing.

---

## 7) DockLock Runtime Targets

* **Start** < 500ms → warmed snapshots + copy-on-write layers
* **Overhead** < 10MB → static-linked microinit + sealed namespace
* **Auditing**: every syscall class → aggregate to StepReceipts (1000+/s)
* **Determinism**: pinned kernel features, cpu quotas, net classes, stable clocks

---

## 8) ENC Scheduler (K8s++)

* **Queue**: priority = (policy score × fee rate × age)
* **Placement**: bin-pack by cpu/mem; locality for storage; **must** run sensitive jobs on **ENC-val-1**.
* **ZK optionality** (flag): generate succinct proofs for selected policies (future toggle).

---

## 9) Bank Mesh & Settlement

* **Notary Banks** sign **SettleNote** structs that chain to PoE bundles (cross-proof).
* **AUR** minted/burned against gold oracle with >100% collateral config.
* **GEN/NEX/FLX/AUR** roles:
  * **GEN** (×1000 unit scale): governance/anchors
  * **NEX** (×100): work/meter token (PoE-minted)
  * **FLX** (×10): elasticity buffer (market maker incentives)
  * **AUR** (×1): gold-backed settlement rail

**Owner earnings**:
Owner receives **0.2% of fees** (as per split), **disbursed each window**, capped by Court rule if treasury < target.

---

## 10) SLOs (convert your targets to enforceable budgets)

* API p95 < 100ms (Cage), p99 < 250ms
* Consensus finality: p95 < 3s
* Container cold start: p95 < 500ms
* Receipt throughput: sustain 1k/s; burst 5k/s for 60s
* HA: 99.99%; DR RTO < 5m; RPO ≤ 30s

---

## 11) Threat Model (quick cut)

* **Byzantine validators** → IBFT + Court revocation pathway
* **Receipt forgery** → Ed25519 per-receipt + Merkle in LogBlocks + ENC attestation
* **Economic spam** → Cage rate-pricing + stake buckets + fee floors
* **Oracle risk (gold)** → median of ≥3 providers with dispute window & halting rule
* **Key loss** → threshold wallets for banks/miners; rotation via governance

---

## 12) Test Plan (automation-first)

1. **Determinism suite**: replay 10k DockLock ops → identical LogBlock roots.
2. **IBFT chaos**: drop/partition 20–40% validators → finality < 5s maintained.
3. **Billing integrity**: fuzz usage → Φ/Γ reproducible; NEX mint stable.
4. **Court upgrades**: simulate bad policy → voter rollback with upgrade\_window guard.
5. **Bank settlement**: fiat sandbox loopbacks (ACH/Wire simulators) → AUR Δ equals fiat Δ ±ε.
6. **Throughput**: 10k TPS synthetic PoE bundles, mempool GC within steady memory.

---

## 13) Minimal Deploy (dev)

**Ports**: Cage:8443, BPI ring:7070/7071, BPCI:9090, Court:9443, Blockbook:9200

```yaml
# docker-compose-ish sketch (dev)
services:
  cage:
    image: pravyom/cage:dev
    env: [CAGE_KEYS=..., COURT_URL=https://court:9443]
    ports: ["8443:8443"]

  bpi-comm-1:
    image: pravyom/bpi:dev
    command: ["--role=comm","--hq=bpci-hq:9090"]
    depends_on: [cage, enc-notary-1]

  bpi-val-1:
    image: pravyom/bpi:dev
    command: ["--role=validator","--ibft","--index=1"]
    depends_on: [bpi-comm-1]

  bpi-val-2:
    image: pravyom/bpi:dev
    command: ["--role=validator","--ibft","--index=2"]
    depends_on: [bpi-comm-1]

  enc-notary-1:
    image: pravyom/enc:dev
    command: ["--role=notary","--attest=soft"]
    depends_on: [cage]

  enc-val-1:
    image: pravyom/enc:dev
    command: ["--role=validator","--attest=soft"]
    depends_on: [enc-notary-1]

  bpci-hq:
    image: pravyom/bpci:dev
    command: ["--validators=4","--ibft"]
    depends_on: [bpi-val-1, bpi-val-2, enc-val-1]

  court:
    image: pravyom/court:dev
    command: ["--policy-dir=/policies","--quorum=0.667"]

  blockbook:
    image: pravyom/blockbook:dev
    command: ["--store=/data","--index"]
```

---

## 14) Developer Experience (DX)

* **SDK** (`rust`, `ts`): `submit_receipt()`, `submit_poe_bundle()`, `query_audit()`, `settle_fiat()`.
* **CLI**:
  * `pravyom receipts tail --app APP_A`
  * `pravyom poe bundle --from T1 --to T2 --dry-run`
  * `pravyom court propose --file appA.yaml`
  * `pravyom bank settle --amount 1000 AUR --to SWIFT:...`

Onboarding target: **<30 minutes** = generate keys → run compose → post receipts → see bundle finalized and fees split.

---

## 15) Implementation Status

* **Complete Infrastructure** and the **5-node/app Hyperledger layout** are now **harmonized**:
  * Court policies (YAML) directly drive ENC scheduling, BPI validation, PoE math, and Bank Mesh.
  * The same receipts → logbook → bundle path fuels **audit**, **economics**, and **consensus** simultaneously—**one pipeline, three guarantees**.

Ready for implementation with reference repo layout, policy compiler, Phi/Gamma calculator, and fake bank settlement loop.
