# Pravyom / BPI / BPCI – 10‑Minute Reviewer Quickstart

This guide is designed for a **strict technical reviewer** to understand and sanity‑check the system in **~10 minutes**, with:

- A **2‑minute mental model** of the architecture.  
- A **fast compile path**.  
- A **curated command tour (~50 commands)** that shows real behaviour without drowning in detail.

It assumes a Linux dev box with Rust toolchain installed (Rust 1.76+ recommended).

---

## 0. Mental Model in 2 Minutes

- **BPI Core** – Rust blockchain OS kernel + LCCD consensus:
  - `bpi-core` crate & binary: consensus, VO kernel, logbook/6D, QLock, immutable audit, forensic firewall.
  - Talks to real JSON‑RPC/EVM nodes via the logbook bridge.
- **BPCI Enterprise** – Rust enterprise layer on top of BPI:
  - `bpci-enterprise` crate & binaries: consensus server, cluster ledger, CUEDB, economics, readiness tests.
  - Acts as the enterprise API, orchestration and economics brain.
- **Hermes / Web4 Mesh** – self-healing P2P/transport layer:
  - Kappa‑aware routing, W4‑Fluid transport, Web4 addressing, QuantumHeartbeat.
- **Documentation anchors**:
  - `documentation/02-pravyom-mathematical-foundations-49-equations.md` – math for 49 core components.
  - `documentation/03-pravyom-architecture-digraph-index.md` – component indices + Mermaid diagrams.
  - `documentation/pravyom-full-infra-architecture.mmd` + `.svg` – full 7‑step infra digraph.

After reading the bullet points above, a reviewer can jump **straight to code and commands**.

---

## 1. Build & Setup (5 Commands)

From repo root: `/home/umesh/metanode`.

1. **Check toolchain & workspace:**

   ```bash
   rustc --version
   cargo --version
   cargo metadata --no-deps -q
   ```

2. **Build BPI Core and workspace (debug):**

   ```bash
   cargo build -p bpi-core
   ```

3. **Build BPCI Enterprise (debug):**

   ```bash
   cargo build -p bpci-enterprise
   ```

4. **Optional: run core tests (sanity):**

   ```bash
   cargo test -p bpi-core -- --test-threads=1
   ```

5. **Optional: run enterprise tests (targeted):**

   ```bash
   cargo test -p bpci-enterprise -- --test-threads=1
   ```

If build + a focused test subset pass, the reviewer knows the codebase is alive and coherent.

---

## 2. BPI Core CLI – Chain, Mempool, Consensus (≈15 Commands)

> These commands assume the `bpi-core` binary exposes standard subcommands like `chain`, `mempool`, `consensus`, `economics`, etc. Adjust names if your CLI differs.

Run BPI Core with `--help`:

```bash
cargo run -p bpi-core -- --help
```

### 2.1 Chain & Ledger Introspection

1. **Show basic chain info:**

   ```bash
   cargo run -p bpi-core -- chain info
   ```

2. **Show chain statistics (TPS, utilization):**

   ```bash
   cargo run -p bpi-core -- chain stats
   ```

3. **Inspect latest block:**

   ```bash
   cargo run -p bpi-core -- chain head
   ```

4. **Query a specific block (height 1):**

   ```bash
   cargo run -p bpi-core -- chain block --height 1
   ```

### 2.2 Mempool & Fees

5. **Mempool status (size, avg fee, throughput):**

   ```bash
   cargo run -p bpi-core -- mempool status
   ```

6. **List pending transactions:**

   ```bash
   cargo run -p bpi-core -- mempool list --limit 10
   ```

7. **Show economics configuration:**

   ```bash
   cargo run -p bpi-core -- economics show
   ```

8. **Simulate fee under congestion:**

   ```bash
   cargo run -p bpi-core -- economics simulate-fee --tps 1000
   ```

### 2.3 Consensus & LCCD

9. **Show consensus metrics (TPS, finality time):**

   ```bash
   cargo run -p bpi-core -- consensus metrics
   ```

10. **Dump current validator set (if available):**

    ```bash
    cargo run -p bpi-core -- consensus validators
    ```

11. **Run a short local benchmark:**

    ```bash
    cargo run -p bpi-core -- benchmark quick
    ```

### 2.4 Audit, Forensics & QLock

12. **Show immutable audit status:**

    ```bash
    cargo run -p bpi-core -- audit status
    ```

13. **Tail last 5 audit records:**

    ```bash
    cargo run -p bpi-core -- audit tail --limit 5
    ```

14. **Inspect forensic firewall configuration:**

    ```bash
    cargo run -p bpi-core -- forensic firewall-config
    ```

15. **Check QLock / quantum sync status:**

    ```bash
    cargo run -p bpi-core -- qlock status
    ```

Even if some subcommands differ slightly, this gives a reviewer a **map of what to look for** in `main.rs` / CLI modules.

---

## 3. BPCI Enterprise – Consensus Server & Readiness (≈15 Commands)

> These commands assume a BPCI enterprise binary (e.g. `bpci-enterprise` or `bpci-lccd`) with service‑oriented subcommands.

### 3.1 BPCI Server Basics

1. **Show BPCI CLI help:**

   ```bash
   cargo run -p bpci-enterprise -- --help
   ```

2. **Start the BPCI LCCD consensus server (dev mode):**

   ```bash
   cargo run -p bpci-enterprise -- consensus-server --dev
   ```

3. **Check consensus server health:**

   ```bash
   cargo run -p bpci-enterprise -- consensus health
   ```

4. **Show cluster ledger status:**

   ```bash
   cargo run -p bpci-enterprise -- ledger status
   ```

### 3.2 Economics & PoE

5. **Inspect current economics config:**

   ```bash
   cargo run -p bpci-enterprise -- economics show
   ```

6. **List recent PoE events:**

   ```bash
   cargo run -p bpci-enterprise -- economics poe-log --limit 10
   ```

7. **Simulate enterprise workload billing:**

   ```bash
   cargo run -p bpci-enterprise -- economics simulate-usage --tps 500 --duration 60
   ```

### 3.3 CUEDB & Enterprise Data

8. **Show CUEDB tenant overview:**

   ```bash
   cargo run -p bpci-enterprise -- cuedb tenants
   ```

9. **Show database rules (transaction rate triggers, etc.):**

   ```bash
   cargo run -p bpci-enterprise -- cuedb rules
   ```

10. **Run a basic CUEDB query:**

    ```bash
    cargo run -p bpci-enterprise -- cuedb query --sql "SELECT 1"
    ```

### 3.4 Readiness & Real-World Validation

11. **Run LCCD pilot readiness test (short mode):**

    ```bash
    cargo run -p bpci-enterprise -- readiness lccd-pilot --mode short
    ```

12. **Run real-world pilot validation (simulated):**

    ```bash
    cargo run -p bpci-enterprise -- readiness real-world-pilot --dry-run
    ```

13. **Show last readiness report:**

    ```bash
    cargo run -p bpci-enterprise -- readiness report --latest
    ```

14. **List detected limitations blocking 100% readiness:**

    ```bash
    cargo run -p bpci-enterprise -- readiness limitations
    ```

15. **Show interoperability / cross-chain status:**

    ```bash
    cargo run -p bpci-enterprise -- interoperability status
    ```

This set lets a reviewer see **enterprise behaviour and readiness logic** without deep diving into all modules.

---

## 4. Hermes / Mesh & 4D / ZipLock (≈10 Commands)

> These commands assume separate binaries or modes for Hermes mesh and 4D/ZipLock tooling.

### 4.1 Hermes / Web4 Mesh

1. **Show Hermes mesh help:**

   ```bash
   cargo run -p bpci-enterprise -- hermes --help
   ```

2. **Print current mesh topology:**

   ```bash
   cargo run -p bpci-enterprise -- hermes mesh-topology
   ```

3. **Show kappa / mesh health stats:**

   ```bash
   cargo run -p bpci-enterprise -- hermes kappa-health
   ```

4. **Simulate a routing decision between two nodes:**

   ```bash
   cargo run -p bpci-enterprise -- hermes route --from nodeA --to nodeB
   ```

### 4.2 4D Database & ZipLock Bundles

5. **Run a 4D query via the bridge:**

   ```bash
   cargo run -p bpi-core -- fourd query --kind SpatialTemporal --limit 10
   ```

6. **List .zkl bundles:**

   ```bash
   cargo run -p bpi-core -- ziplock list
   ```

7. **Inspect a single bundle’s 4D coordinates:**

   ```bash
   cargo run -p bpi-core -- ziplock inspect --id <BUNDLE_ID>
   ```

8. **Verify a bundle against audit / Merkle root:**

   ```bash
   cargo run -p bpi-core -- ziplock verify --id <BUNDLE_ID>
   ```

9. **Show QuantumHeartbeat configuration:**

   ```bash
   cargo run -p bpci-enterprise -- heartbeat config
   ```

10. **Emit a single quantum heartbeat (test mode):**

    ```bash
    cargo run -p bpci-enterprise -- heartbeat emit --once
    ```

---

## 5. Forensics, Audit & Security (≈10 Commands)

### 5.1 Forensic Firewall & Oracle

1. **Show forensic firewall status:**

   ```bash
   cargo run -p bpi-core -- forensic status
   ```

2. **List recent behavioural anomalies:**

   ```bash
   cargo run -p bpi-core -- forensic anomalies --limit 10
   ```

3. **Show Forensic Oracle performance metrics:**

   ```bash
   cargo run -p bpi-core -- forensic oracle-metrics
   ```

4. **Trigger a synthetic forensic event (test):**

   ```bash
   cargo run -p bpi-core -- forensic simulate-event
   ```

### 5.2 Immutable Audit & Merkle Proofs

5. **Show Merkle root for latest audit tree:**

   ```bash
   cargo run -p bpi-core -- audit merkle-root
   ```

6. **Generate a Merkle proof for an audit record:**

   ```bash
   cargo run -p bpi-core -- audit prove --record-id <ID>
   ```

7. **Verify a proof against the current root:**

   ```bash
   cargo run -p bpi-core -- audit verify --record-id <ID>
   ```

### 5.3 Governance & Geo

8. **Show governance alignment metrics:**

   ```bash
   cargo run -p bpci-enterprise -- governance alignment
   ```

9. **Show dual-majority quorum configuration:**

   ```bash
   cargo run -p bpci-enterprise -- governance quorum
   ```

10. **List GeoDID / GeoLedger partitions:**

    ```bash
    cargo run -p bpci-enterprise -- governance geodid-list
    ```

---

## 6. Suggested 10-Minute Review Flow

1. **Read section 0 (2‑minute mental model).**
2. **Skim diagrams:**
   - Open `documentation/pravyom-full-infra-architecture.svg` (big picture).  
   - Optionally, open `03-pravyom-architecture-digraph-index.md` and view Mermaid blocks.
3. **Build core pieces:** run the commands from **Section 1** (build + 1 test target).
4. **Run 3–5 `bpi-core` commands:** chain stats, consensus metrics, audit status, mempool status.
5. **Run 3–5 `bpci-enterprise` commands:** consensus health, ledger status, readiness short test, economics show.
6. **Optionally explore mesh / 4D:** run 1–2 Hermes and ZipLock commands.
7. **Optionally inspect docs:** open the 49‑equations file and one or two components that match what you just exercised.

In 10–15 minutes a reviewer should see:

- The **architecture is coherent** (via diagrams).  
- The **code builds and tests**.  
- The **CLI surface is rich and aligned** with the architecture (consensus, audit, economics, readiness, mesh).  
- There is **deep math and design documentation** behind each major subsystem.

---

## 7. BPCI Constellation & 1‑Minute Demos (Real Commands)

This section captures the **exact commands** used during the BPCI constellation and Hermès P2P demos on a laptop‑class machine.

Assumed repo root:

```bash
cd /home/umesh/metanode
```

### 7.1 BPCI Constellation – Components & Env

1. **Print constellation layout + commands (controller binary):**

   ```bash
   cargo run -p pravyom-enterprise --bin bpci_constellation_control
   ```

2. **Export core env bindings (used by demos and curl checks):**

   ```bash
   export DEPLOYMENT_TYPE="BSO-K8 orchestrator"
   export NETWORK_BINDING="0.0.0.0 (external access)"
   export CLUSTER_NAME="bpci-local-dev"
   export NAMESPACE="bpci-enterprise"

   export BPCI_CONSENSUS_URL="http://127.0.0.1:9001"
   export BPCI_BLOCKCHAIN_URL="http://127.0.0.1:8082"
   export BPCI_AUCTION_MEMPOOL_URL="http://127.0.0.1:9004"
   export BPCI_AUCTION_DB_URL="http://127.0.0.1:7002"
   ```

3. **Start each BPCI constellation component (separate terminals recommended):**

   - **Component 1 – LCCD Consensus Server:**

     ```bash
     cargo run -p pravyom-enterprise --bin bpci-consensus-server \
       -- --port 9001
     ```

   - **Component 2 – BPCI Blockchain Server:**

     ```bash
     cargo run -p pravyom-enterprise --bin bpci_blockchain_server \
       -- --api-port 8082 \
          --consensus-server-url http://127.0.0.1:9001
     ```

   - **Component 3 – BPCI Auction Mempool Server:**

     ```bash
     cargo run -p pravyom-enterprise --bin bpci_auction_mempool_server \
       -- --api-port 9004
     ```

   - **Component 4 – BPCI Auction DB Server (local maintainer):**

     ```bash
     cargo run -p pravyom-enterprise --bin bpci_auction_db_server
     ```

   - **Component 5 – BPCI‑BPI Bridge (HTTP on 6001):**

     ```bash
     cargo run -p pravyom-enterprise --bin bpci_bpi_bridge \
       -- --port 6001
     ```

4. **Quick health checks for the running constellation:**

   ```bash
   curl -i "$BPCI_CONSENSUS_URL/api/v1/health"
   curl -i "$BPCI_BLOCKCHAIN_URL/health"

   curl -i "$BPCI_AUCTION_MEMPOOL_URL/auction/submit" \
     -X POST -H 'Content-Type: application/json' -d '{}'

   curl -i "$BPCI_AUCTION_DB_URL/api/v1/auction/record" \
     -X POST -H 'Content-Type: application/json' -d '{"tx_id":"test","from_bpi":"bpi","to_bpci":"bpci","amount":1,"record_type":"demo"}'
   ```

If all four calls return HTTP 200, the constellation is considered **healthy** for demo purposes.

### 7.2 BPCI Constellation 1‑Minute Demo

5. **Run the 1‑minute constellation demo against the live constellation:**

   ```bash
   cargo run -p pravyom-enterprise --bin bpci_constellation_demo
   ```

   - **Report output:** `/tmp/bpci_constellation_demo_report.txt`  
   - Contains a per‑transaction table with consensus health, LCCD round status, blockchain / auction / DB results.

6. **Optional – BPI↔BPCI lifecycle demo (accounting‑side economics):**

   ```bash
   cargo run -p pravyom-enterprise --bin bpi_bpci_lifecycle_demo
   ```

   - **Report output:** `/tmp/bpi_bpci_lifecycle_demo_report.txt`

### 7.3 Hermès Mesh & Decentralization Demos

7. **Hermes‑Lite Web‑4 mesh mini‑network demo (mesh health + consensus timeline):**

   ```bash
   cargo run -p pravyom-enterprise --bin hermes_mesh_demo
   ```

   - **Report output:** `/tmp/hermes_mesh_demo_report.txt`  
   - Shows mesh health ratio, node counts, consensus rounds, and throughput over ~60 seconds.

8. **Hermes P2P decentralization phase simulation (cell replication + BPIOS growth):**

   ```bash
   cargo run -p pravyom-enterprise --bin hermes_decentralization_demo
   ```

   - **Report output:** `/tmp/hermes_decentralization_demo_report.txt`  
   - Tracks phases:
     - `Centralized Constellation`  
     - `NodeConnectionSync (Hermes P2P forming)`  
     - `MeshEvolution (decentralization in progress)`  
     - `AutonomousMesh (full decentralization)`
   - Each row records nodes (healthy/total), mesh health ratio, consensus confidence, cellular divisions, and throughput.

These concrete commands give a reviewer a **live, end‑to‑end picture**: real BPCI constellation behaviour, BPI↔BPCI lifecycle economics, and Hermès P2P decentralization, all running safely on a single laptop.
