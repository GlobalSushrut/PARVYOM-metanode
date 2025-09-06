# Metanode Token Stack — Formal Mathematics

## 0) Global Notation

**Time:** discrete epochs $t=0,1,2,\dots$ (e.g., 10–30s blocks; governance uses longer windows).

**Addresses:** $i \in \mathcal{H}$ (holders), banks $b\in\mathcal{B}$, validators $v\in\mathcal{V}$.

**Prices & FX:** $\mathsf{PX}_{g}$ = gold price (reference numéraire), $\mathsf{FX}_{x\to g}(t)$ fiat→gold, $\mathsf{FX}_{g\to x}(t)$ gold→fiat.

**PoE inputs:** set of validated jobs in epoch $t$, $\mathcal{J}(t)$.

**Per-epoch fee parameters (from prior spec):**

* Job fee rate $f=1\%$ of job notional (gold-equiv).
* Split: $f_m=0.5\%$ to miner/community, $f_o=0.5\%$ to foundation/ops.
* In miner share: $f_{m,\text{lock}}=0.2\%$ (permanent lock) and $f_{m,\text{sp}}=0.3\%$ (spendable).

**PoE index (normalized $[0,\infty)$):**

$$
\Phi(t)=
w_V\frac{\sum_{J\in\mathcal{J}(t)}V_g(J)}{\text{scale}_V}
+w_L\frac{\sum_{J\in\mathcal{J}(t)}\Delta L(J)}{\text{scale}_L}
+w_U\frac{\sum_{v\in \mathcal{V}}\text{uptime}_v(t)}{|\mathcal{V}|T}
+w_Q\,\text{QualityScore}(t),
\quad \sum w_\bullet=1
$$

where $V_g(J)=V_x(J)\cdot \mathsf{FX}_{x\to g}(t)$.

**Issuance gating function (smooth):**

$$
\Gamma(\Phi)=\frac{\Phi}{1+\Phi}\in(0,1)
$$

**Prestige (for miners/coins/validators as needed):** $P_i(t)\in[0,\infty)$ (derived from ancestry, lock history, reliability).

---

## 1) GEN — Governance / Genesis

**Supply:** fixed at genesis.

$$
S_{\text{GEN}}(t)=S_{\text{GEN}}(0)=100{,}000
$$

Allocations (your plan): treasury 60k, founder 20k (6-mo lock), governance pool 20k.

**Balances:** $B_{\text{GEN}}(i,t)$ with $\sum_i B_{\text{GEN}}(i,t)=S_{\text{GEN}}$.

**Voting Power (epoch-windowed):**

$$
\text{VP}_i(t)=\underbrace{B_{\text{GEN}}(i,t)}_{\text{raw}}
\cdot \underbrace{\lambda_{\text{lock}}(i,t)}_{\text{time/lock boost}}
\cdot \underbrace{\lambda_{P}(i,t)}_{\text{prestige}}
$$

* Lock boost (example, capped): $\lambda_{\text{lock}}(i,t)=\min\{1+\alpha_{\ell}\cdot \text{months\_locked}(i), \Lambda_{\max}\}$.
* Prestige boost: $\lambda_P(i,t)=1+\alpha_P\cdot \frac{P_i(t)}{\overline{P}(t)}$.

**Governance thresholds (constants or governance-set):**

* Proposal stake: $\Theta_{\text{prop}}$ GEN minimum.
* Quorum: $Q = q \cdot S_{\text{GEN}}$ GEN (e.g., $q=10\%$).
* Passage: approval ratio $\ge \xi$ (e.g., $60\%$) among votes cast.

**Governance effectors:** parameter vector $\theta$ (e.g., $f,\ f_m,\ f_o,\ \{ \tau_k\},$ emission caps) is updated only when a proposal passes and timelock $T_{\text{exec}}$ elapses.

**Invariants:** fixed total supply; VP is non-transferable except via GEN transfers/locks.

---

## 2) NEX — Community Rewards & Incentives (Dynamic, PoE-linked)

**Supply evolution:**

$$
S_{\text{NEX}}(t+1)=S_{\text{NEX}}(t)+M_{\text{NEX}}(t)-B_{\text{NEX}}(t)
$$

* Genesis $S_{\text{NEX}}(0)=300{,}000$.
* Mint $M_{\text{NEX}}(t)$ is **PoE-gated and governance-capped**.

**PoE-gated mint (epoch):**

$$
M_{\text{NEX}}(t)=\min\Big\{ C_{\text{NEX}}(t),\ \ \underbrace{\beta_{\text{NEX}}\cdot \Gamma(\Phi(t))\cdot \mathcal{S}_{\text{NEX}}(t)}_{\text{PoE quota}} \Big\}
$$

* $C_{\text{NEX}}(t)$: governance cap per epoch (absolute).
* $\mathcal{S}_{\text{NEX}}(t)$: a scaling base (e.g., rolling mean of $\sum V_g$ or $S_{\text{NEX}}(t)$ for gentle growth).
* $\beta_{\text{NEX}}\in(0,1)$: sensitivity.

**Distribution across recipients $i$:**

$$
M_{\text{NEX}}(i,t)=M_{\text{NEX}}(t)\cdot \frac{W_i(t)}{\sum_j W_j(t)}
$$

with weight

$$
W_i(t)=
\underbrace{\widehat{\text{PoE}}_i(t)}_{\text{normalized miner score}}
\cdot 
\underbrace{\lambda_{P}(i,t)}_{\text{prestige}}
\cdot
\underbrace{\lambda_{D}(i,t)}_{\text{diversity}}
$$

($\lambda_D$ boosts under-represented geos/ASNs, bounded $[1,\Lambda_D]$).

**Burn:** $B_{\text{NEX}}(t)=\zeta_{\text{NEX}}\cdot \text{Fees}_{\text{NEX}}(t)$ (optional—e.g., burn a fraction of fees paid in NEX).

**Invariants:** capped per-epoch mint; if $\Phi(t)<\tau_{\text{NEX}}$ then $M_{\text{NEX}}(t)=0$.

---

## 3) FLX — Operational Payments (High-velocity, elastic)

**Supply evolution:**

$$
S_{\text{FLX}}(t+1)=S_{\text{FLX}}(t)+M_{\text{FLX}}(t)-B_{\text{FLX}}(t)
$$

* Genesis $S_{\text{FLX}}(0)=500{,}000$.

**Mint policy (elastic to usage):**

$$
M_{\text{FLX}}(t)= \min\{C_{\text{FLX}}(t),\ \mu\cdot U_{\text{net}}(t)\}
$$

* $U_{\text{net}}(t)$: net FLX demand proxy (e.g., pending gas buffer + moving average of tx fees & queue length).
* $\mu$: elasticity.
* $C_{\text{FLX}}(t)$: governance hard cap/epoch.

**Burn from fees (your plan 50% of tx fees):**

$$
B_{\text{FLX}}(t)=\beta_{\text{burn}}\cdot \text{Fees}_{\text{FLX}}(t), \quad \beta_{\text{burn}}=0.5
$$

**Fee schedule (per tx):**

$$
\text{Fee}_{\text{FLX}} = F_0 + r_d\cdot \text{KB} + r_c\cdot \text{CU}
$$

($F_0=0.01\ \text{FLX}, r_d=0.001\ \text{FLX/KB}, r_c=0.1\ \text{FLX/CU}$).

**Invariants:** never mint beyond $C_{\text{FLX}}(t)$; burn ≥ 50% of fee flow.

---

## 4) AUR — Gold-Backed Settlement (Bank-only, 1:1 with gold)

**Supply equals audited gold liabilities:**

$$
S_{\text{AUR}}(t) = \sum_{b\in\mathcal{B}} \underbrace{G_b(t)}_{\text{oz or gram equiv}} \quad \text{(scaled to AUR units)}
$$

* Mint by bank $b$: $\Delta^+ S_{\text{AUR}}= \Delta G_b$.
* Burn by bank $b$: $\Delta^- S_{\text{AUR}}= \Delta G_b$.
* Public trading optional; **settlement-only** recommended (bank ↔ bank / user ↔ bank).

**Settlement conversion (atomic):**
Given fiat $x\to y$, amount $X$:

$$
G = X\cdot \mathsf{FX}_{x\to g}(t),\quad
Y = G\cdot \mathsf{FX}_{g\to y}(t)
$$

Mint $G$ AUR at source, transfer, burn $G$ AUR at dest, credit $Y$ fiat.

**Invariants:** Perfect backing: $S_{\text{AUR}}(t)$ must equal on-chain proof of reserves (PoR). No non-bank mint.

---

## 5) PoE Mathematics (Job-level → Epoch-level → Rewards)

**Per job $J\in\mathcal{J}(t)$:**

$$
\begin{aligned}
&V_g(J)= V_x(J)\cdot \mathsf{FX}_{x\to g}(t)\\
&\text{fee}(J)= f\cdot V_g(J)\\
&\text{miner\_share}(J)= f_m\cdot V_g(J),\quad \text{ops\_share}(J)= f_o\cdot V_g(J)\\
&\Delta L(J)= f_{m,\text{lock}}\cdot V_g(J),\quad \text{spendable}(J)= f_{m,\text{sp}}\cdot V_g(J)
\end{aligned}
$$

**Epoch aggregates:**

$$
\begin{aligned}
&V_g^{\Sigma}(t)=\sum_{J\in\mathcal{J}(t)}V_g(J)\\
&\Delta L^{\Sigma}(t)= \sum_{J\in\mathcal{J}(t)}\Delta L(J)\\
&\text{Ops}^{\Sigma}(t)= \sum_{J\in\mathcal{J}(t)} \text{ops\_share}(J)\\
&\Phi(t)=\text{as defined in §0 (with these aggregates)}
\end{aligned}
$$

**Miner normalized score (for recipient $i$):**

$$
\widehat{\text{PoE}}_i(t)=
\frac{\sum_{J\in\mathcal{J}_i(t)} \big(V_g(J)\cdot q(J)\big)}
{\varepsilon + \sum_{k}\sum_{J\in\mathcal{J}_k(t)}\big(V_g(J)\cdot q(J)\big)}
$$

where $q(J)\in[0,1]$ is a per-job quality flag (BISO-compliant), $\varepsilon$ tiny constant.

---

## 6) Cross-Token Interaction Mathematics

### 6.1 GEN → Parameters (governance control)

Let $\theta(t)$ be the parameter vector. A passed proposal $p$ changes $\theta$ after timelock:

$$
\theta(t^+)= \mathcal{U}_p(\theta(t))\quad \text{iff}\quad 
\text{Votes}_{\text{yes}} \ge \xi(\text{Votes}_{\text{yes}}+\text{Votes}_{\text{no}}),\
\text{Votes}_{\text{total}}\ge Q
$$

### 6.2 PoE → NEX Mint

As in §2, conditioned on $\Phi(t)\ge\tau_{\text{NEX}}$:

$$
M_{\text{NEX}}(t)=\min\{C_{\text{NEX}}(t),\ \beta_{\text{NEX}}\Gamma(\Phi(t))\mathcal{S}_{\text{NEX}}(t)\}
$$

and split by weights $W_i(t)$.

### 6.3 Network Usage → FLX Elasticity

$$
M_{\text{FLX}}(t)=\min\{C_{\text{FLX}}(t),\ \mu\cdot U_{\text{net}}(t)\},\quad
B_{\text{FLX}}(t)=\beta_{\text{burn}}\cdot \text{Fees}_{\text{FLX}}(t)
$$

with $U_{\text{net}}(t)$ derived from demand/latency backlogs.

### 6.4 Cross-Border Settlement via AUR

For a settlement request $(x\to y, X)$:

$$
\begin{aligned}
&G=X\cdot \mathsf{FX}_{x\to g}(t) \\
&\text{Mint}_{\text{AUR,src}}(G),\ \text{Transfer},\ \text{Burn}_{\text{AUR,dst}}(G) \\
&Y=G\cdot \mathsf{FX}_{g\to y}(t)\ \Rightarrow\ \text{credit }Y\ \text{fiat}
\end{aligned}
$$

PoR constraint: post-event $S_{\text{AUR}}$ equals audited gold liabilities.

### 6.5 Treasury & Fee Routing

* Ops fee (gold-equiv) accrues to treasury $T$ with mark-to-market accounting:

$$
T_g(t+1)=T_g(t)+\text{Ops}^{\Sigma}(t) - \text{Spend}_T(t)
$$

* If treasury spends in NEX/FLX:

$$
\text{Spend}_T^{\text{NEX/FLX}}(t) \le \text{Budget}_{\text{gov}}(t)
$$

### 6.6 Liquidity & AMM (if DEX pools exist)

For pool $(\text{FLX},\text{NEX})$ with constant-product $k$:

$$
R_{\text{FLX}}(t)\cdot R_{\text{NEX}}(t)=k,\quad
\text{price}_{\text{FLX}/\text{NEX}}(t)=\frac{R_{\text{NEX}}(t)}{R_{\text{FLX}}(t)}
$$

Initial LP seeding from genesis liquidity allocation; LP tokens time-locked.

---

## 7) Safety Constraints & Invariants (Cross-stack)

1. **GEN fixed supply:** $\forall t,\ S_{\text{GEN}}(t)=S_{\text{GEN}}(0)$.
2. **AUR full-backing:** $\forall t,\ S_{\text{AUR}}(t)=\sum_b G_b(t)$ (audited).
3. **NEX mint freeze:** if $\Phi(t)<\tau_{\text{NEX}}\Rightarrow M_{\text{NEX}}(t)=0$.
4. **FLX burn floor:** $\beta_{\text{burn}}\ge 0.5$.
5. **Per-epoch caps:** $M_{\text{NEX}}(t)\le C_{\text{NEX}}(t)$, $M_{\text{FLX}}(t)\le C_{\text{FLX}}(t)$.
6. **Oracle robustness:** use $m$-of-$n$ medianized feeds; settlement blocked if dispersion exceeds bound $\Delta_{\max}$.
7. **Governance timelock:** any $\theta$ change delayed by $T_{\text{exec}}$ (e.g., 24–72h).
8. **Anti-gaming PoE:** require uniqueness proofs for jobs; duplicated or low-quality $q(J)\to 0$ nullifies reward.

---

## 8) Minimal State-Transition Pseudocode (per epoch)

```pseudo
function epoch_close(t):
  // 1) Aggregate PoE
  Vg_sum = sum_J Vg(J); dL_sum = sum_J dL(J)
  Phi = compute_PoE(Vg_sum, dL_sum, uptimes, quality)

  // 2) NEX issuance (gated)
  if Phi >= tau_NEX:
     M_NEX = min(C_NEX(t), beta_NEX * Gamma(Phi) * S_base_NEX(t))
     distribute_NEX_by_weights(M_NEX, W_i(t))
  else:
     M_NEX = 0

  // 3) FLX elasticity
  demand = estimate_usage_pressure()
  M_FLX = min(C_FLX(t), mu * demand)
  B_FLX = burn_fraction * fees_collected_in_FLX
  mint_and_burn_FLX(M_FLX, B_FLX)

  // 4) Fees & Treasury
  Treasury_g += ops_fee_gold_equiv_this_epoch

  // 5) Governance changes after timelock
  apply_passed_proposals_if_ready()
```

---

## 9) Parameter Table (defaults; governance-tunable)

| Parameter | Symbol | Default Value | Description |
|-----------|--------|---------------|-------------|
| **PoE Fee Structure** |
| Job fee rate | $f$ | 1% | Total fee per job (gold-equiv) |
| Miner/community share | $f_m$ | 0.5% | Split to miner/community |
| Foundation/ops share | $f_o$ | 0.5% | Split to foundation treasury |
| Miner locked portion | $f_{m,\text{lock}}$ | 0.2% | Permanent lock increment |
| Miner spendable portion | $f_{m,\text{sp}}$ | 0.3% | Immediate spendable payout |
| **PoE Thresholds** |
| NEX mint threshold | $\tau_{\text{NEX}}$ | TBD | Minimum PoE for NEX issuance |
| **Token Caps & Rates** |
| NEX sensitivity | $\beta_{\text{NEX}}$ | 0.1 | PoE → NEX mint sensitivity |
| NEX epoch cap | $C_{\text{NEX}}(t)$ | 1000 | Max NEX mint per epoch |
| FLX elasticity | $\mu$ | 0.05 | Usage → FLX mint elasticity |
| FLX epoch cap | $C_{\text{FLX}}(t)$ | 5000 | Max FLX mint per epoch |
| FLX burn rate | $\beta_{\text{burn}}$ | 0.5 | Fraction of fees burned |
| **Governance** |
| Proposal stake | $\Theta_{\text{prop}}$ | 100 GEN | Minimum GEN to propose |
| Quorum rate | $q$ | 10% | Quorum as % of total GEN |
| Passage threshold | $\xi$ | 60% | Approval ratio required |
| Execution timelock | $T_{\text{exec}}$ | 48h | Delay before param changes |
| **Oracle & Security** |
| FX tolerance | $\Delta_{\max}$ | 2% | Max oracle dispersion |
| PoR cadence | - | Monthly | Proof of reserves frequency |

---

## 10) Implementation Priority

### Phase 1: Core Accounting
1. **Token state structs:** $S_\bullet(t)$, balances, supply tracking
2. **Fee router:** Job fees → miner/foundation split
3. **Basic mint/burn:** Capped issuance, burn mechanisms

### Phase 2: PoE Pipeline
1. **Job validation:** $V_g(J)$, quality scoring $q(J)$
2. **PoE calculation:** $\Phi(t)$ computation
3. **NEX distribution:** Weight-based allocation to miners

### Phase 3: Settlement & Governance
1. **AUR mechanics:** Bank-only mint/burn with PoR checks
2. **Governance kernel:** VP calculation, quorum, timelock
3. **Parameter updates:** $\theta$ vector management

### Phase 4: Advanced Features
1. **FLX elasticity:** Demand estimation + responsive minting
2. **Cross-token interactions:** AMM pools, treasury operations
3. **Monitoring & safety:** Oracle robustness, anti-gaming

---

## 11) Test Scenarios

### Unit Tests
- Token supply invariants (GEN fixed, AUR = gold backing)
- PoE fee split calculations (1% → 0.2%/0.3%/0.5%)
- Governance threshold enforcement
- Mint cap compliance

### Integration Tests
- Full epoch cycle: jobs → PoE → NEX mint → distribution
- AUR settlement flow: mint → transfer → burn
- Governance proposal lifecycle: stake → vote → timelock → execute

### Stress Tests
- High PoE activity (mint cap testing)
- Oracle failure scenarios (settlement blocking)
- Deep governance parameter changes
- Anti-gaming resistance (duplicate jobs, quality manipulation)

---

This formal mathematical specification provides the complete foundation for implementing the Metanode four-token economic system with precise PoE linkage, governance controls, and cross-token interactions. All formulas are deterministic and ready for direct translation into production code.
