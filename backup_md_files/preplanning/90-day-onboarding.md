# Metanode / BPI Mesh — 90-Day Onboarding Plan
No-nonsense plan to make the project stupid-simple for beginners. "Docker meets Heroku": one file, one command, happy path first, guardrails everywhere.

---

## Product Principles (Never Break These)
- **One file, one command.** Everything starts from `bpicompose.yml` and `bpi up`.
- **Safe defaults.** If a user does nothing, they still get a secure, working setup.
- **Zero new concepts up front.** Hide "IBFT, DA, anchors, VRF" under a "Noob Mode" until they opt in to "Pro Mode".
- **Always fix or explain.** Every error provides a one-line fix and a `--autofix` option.

---

## The Funnel (How a Noob Becomes a Power User)
1. **5-minute Playground** (in-browser) →
2. **One-click installer** →
3. **`bpi init` scaffold** →
4. **`bpi up` success** →
5. **Ship a sample app** →
6. **Turn on Agreements/Receipts** →
7. **Connect real K8s** →
8. **Add validators/DA** (Pro Mode)

---

## Phase 1 (Days 1–30): Make the "Hello World" Impossible to Fail

### A. One-click Install
**Deliverables:**
- `get.bpi.run` bootstrapper (Mac/Linux/WSL)
- Windows MSI with embedded k3d/kubectl
- Preflight: checks CPU/RAM/ports; offers `--fix-all`

**Success metric:** Install < 60s, >95% pass rate on first try.

### B. CLI "Noob Mode"
**Commands (v1):**
- `bpi init` → writes **bpicompose.yml**, `agreements/basic.yaml`, sample React+Node app
- `bpi up` → runs local devnet (3 validators auto), dashboard link
- `bpi verify` → human text ✓ Finality ✓ DA ✓ Anchors
- `bpi doctor` → diagnoses/fixes common issues

**UX details:**
- Emoji status, progress bars, ✅/⚠️ markers
- Copy-paste cURL at end to test the app

**Success metric:** Time-to-first-response (TTFR) < 3 minutes.

### C. Templates Gallery
**Deliverables:**
- `bpi templates list|use`: `react-spa`, `node-api`, `python-fastapi`, `go-http`, `nextjs`, `ai-inference`
- `bpi init --template ai-inference` seeds a working ML demo

**Success metric:** 3+ templates used per day by week 4.

### D. Playground (No Install)
- **Hosted sandbox**: browser-based VM with `bpi` pre-installed
- **Guided tour**: left pane instructions, right pane terminal; "Run all" button

**Success metric:** 60% of visitors reach a successful `bpi verify`.

### E. Docs That Read Like Recipes
**Structure:**
- "5-minute start" (single page)
- "Deploy your first service" (copy/paste)
- "Agreements in 10 lines of YAML"
- "Upgrade to Pro Mode (validators, DA, anchors)"

**Add** short GIFs (15–30s), not 30-min videos.

---

## Phase 2 (Days 31–60): Agreements & Receipts Without Pain

### A. YAML → WASM, But Invisible by Default
- **`bpi agreement new basic`** → generates a 10-line YAML with common rules (size limits, no egress)
- **`bpi agreement simulate`** → runs pre/post locally on a sample request. Shows PASS/FAIL with reasons
- **`bpi agreement pin`** → pins the compiled WASM; `bpi up` applies it

### B. Auto-policy Wizard
- `bpi policy wizard` asks 6 questions (max request size, allow egress?, signer allow-list?, redact fields?, rate limit?, receipts on?)
- Writes YAML, compiles, pins. Done.

### C. Receipts "On" Switch
- `bpi receipts on|off` toggles deterministic receipts
- `bpi receipts get <id>` prints a **simple** view first; `--raw` for nerds

### D. Error Messages That Teach
If policy denies, show **exact rule** and the **smallest change** to pass.

Example:
> ❌ Denied: `outputs.json.$.amount` (6000) exceeds limit (5000).
> Fix: `agreement.post.checks[0].lte: 7000` or reduce `amount`.

**Success metrics:**
- <10% policy-denial rage quits
- 80% of users can create/edit an Agreement without docs

---

## Phase 3 (Days 61–90): K8s + Testnet + "Pro Mode" Unlocked

### A. Real K8s in One Line
- `bpi k8 connect --name prod --testnet https://…`
  - Generates certs, SVIDs, peering; runs readiness checks; prints success URL
- **If no cluster?** `bpi k8 create --local` spins a k3d cluster for them

### B. Validators & DA Without Knowing IBFT/RS
- `bpi mainnet scale --validators 3` (or `9`)
  - Explains: "You'll get ~0.8s finality. Cost approx $X/mo."
- `bpi da enable`
  - Explains: "Erasure coding on, we'll challenge nodes for you. Cost approx $Y/mo."
- **Pro Mode toggle** shows extra metrics but keeps one-line commands

### C. Anchors & Force-inclusion
- `bpi anchors enable --chains sepolia,polygon`
  - Prints a link to the anchor txs
- Force-inclusion is **on by default**; `bpi policy show inclusion` explains it in one paragraph

### D. Dashboard That Speaks Human
Cards: **Finality**, **Throughput**, **Costs this month**, **Agreement passes/denies**, **DA health**.
Each card has a "What is this?" hover (two sentences).

**Success metrics:**
- 70% of users connect to testnet on day 1
- 30% enable validators/DA by day 7

---

## Golden-path Scripts (Copy/Paste)

### 1) First Run
```bash
curl -sSL https://get.bpi.run | bash
bpi init --template node-api
bpi up
bpi verify
curl -s "mainnet://registry.submit" -d '{"ping":"mesh"}'
```

### 2) Turn on Receipts & a Basic Agreement
```bash
bpi policy wizard           # answer 6 questions
bpi agreement simulate
bpi agreement pin
bpi receipts on
bpi verify
```

### 3) Connect Real K8s + Scale Security
```bash
bpi k8 connect --name prod --testnet https://testnet.bpi.dev
bpi mainnet scale --validators 3
bpi da enable
bpi anchors enable --chains sepolia,polygon
bpi verify --last 50
```

---

## Packaging & Onboarding Tactics

### Installers
- Brew, APT, Winget + signed binaries
- Docker Desktop extension: "Run on BPI Mesh"

### VS Code Extension
- Inline `bpicompose.yml` schema + autocompletion
- "Run/Verify/Receipts" buttons in the editor

### Telemetry (Opt-in)
- Collect only UX events (install success, command latencies, errors)
- Show you a live adoption funnel

---

## Content & Community That Drives Virality

### 3 "Wow" Demos
1. Real-time chat with sub-second **finality receipts**
2. AI inference service with **policy-enforced** outputs
3. File drop with **DA proofs** and L1 anchors

### Community Building
- **30-minute workshop**: "From zero to auditable API"
- **Starter bounties** ($200–$1000) for building templates, writing guides, hosting validators
- **Discord** with #help-now (SLA: first response <10 min during launch week)

---

## Testing & Quality Gates (So Noobs Don't Hit Weird Stuff)

### Smoke Matrix (CI)
- OS: macOS, Ubuntu, Windows
- Runtimes: Docker Desktop, colima, rancher-desktop
- Network: online/offline, blocked UDP (force TCP fallback)

### Determinism CI
- Run sample app 1000 times; event roots identical

### Chaos Tests
- Kill 1–2 validators; finality stays <1.5s p95

### Doc Tests
- Every snippet in docs is executed in CI

---

## Pricing & Incentives That Don't Scare Noobs

- **Free dev mode:** local devnet, zero fees
- **Testnet faucet:** $5 worth of credits per GitHub account
- **Simple paid tier:** $19/mo "Hobby" (testnet + anchors + receipts cap)
  "Pro" adds validators/DA with usage-based pricing

---

## KPIs to Watch Weekly

- **Install → first successful `bpi up` conversion** (>70%)
- **First app request within 10 minutes** (>60%)
- **Agreement wizard completion rate** (>50%)
- **`bpi verify` success rate** (>85%)
- **Docs bounce rate** (<30%)
- **Discord time-to-first-answer** (<10 min)

---

## Roles & Who Does What (If It's Just You, Do It in Order)

1. **Week 1–2:** CLI + templates + installer (DX engineer / you)
2. **Week 3–4:** Agreements wizard + receipts on/off (backend / you)
3. **Week 5–6:** K8s connect + validation/DA toggles (platform / you)
4. **Week 7–8:** Dashboard + docs + playground (frontend / you)
5. **Week 9–10:** Chaos tests, polish errors, publish workshop
6. **Week 11–12:** Launch testnet + community bounties

---

## "Noob Mode" Defaults (Ship These)

- Validators: **auto-3** (in local devnet), **off** on first cloud run until enabled
- DA: **off** in dev, **guided** in testnet (one command)
- Anchors: **off** in dev, **guided** in testnet
- Agreements: **basic.yaml** applied automatically (no egress, 512KB req limit)
- Receipts: **off** by default; `bpi receipts on` enables

---

## Anti-footgun Guardrails

- Refuse to run if Docker daemon missing; prompt to install
- Warn if less than 2 GB RAM free
- Detect corporate proxies; auto-switch to TCP/TLS
- On failures, print **exact fix** and a `bpi doctor --autofix` hint
