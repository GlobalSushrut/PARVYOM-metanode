# Infra Test Plan: Pravyom / BPI / BPCI / Mesh

This document defines **70 responsive tests** that together should exercise the full stack:

- **30 tests** for `bpi-core` (BPI side)
- **20 tests** for `pravyom-enterprise` / BPCI side
- **10 tests** for Hermes / mesh / control-plane
- **10 tests** for BPI ↔ BPCI communication

The goal is **not** just pass/fail, but **rich, human-readable output** that proves real behavior: IDs, metrics, snapshots, logs, and cross-checks between components.

Each test below should be implemented as a Rust test (or binary test harness) that:

- Prints a short **"Test: ..."** header.
- Prints **key steps and decisions** (e.g., ports used, node IDs, component states).
- Prints a compact **summary block** at the end with `status`, important metrics, and any follow-up actions.

---

## 1. BPI Core (30 tests)

### 1.1 Kernel & Config (5 tests)

1. **BPI-CORE-01: Kernel boots with pilot profile**  
   - **Components:** `start_kernel`, `KernelConfig`, `nx_network_plane`, `vm_server`.  
   - **Flow:** Start kernel with `profile = "pilot"`. Validate config load, NX profile, and VM server startup.  
   - **Output:**
     - Effective `node_id`, environment (`BPI_ENV`), bound ports.  
     - One-line health summary (`status`, `pilot_ready`).

2. **BPI-CORE-02: Config validation rejects invalid ports**  
   - **Components:** `config::NetworkConfig::validate`.  
   - **Flow:** Construct `NetworkConfig` with out-of-range ports; expect `Err` with clear reason.  
   - **Output:** JSON-like error block showing offending fields and message.

3. **BPI-CORE-03: Dynamic port config reload**  
   - **Components:** `dynamic_port_config`, `GLOBAL_PORT_CONFIG`.  
   - **Flow:** Update in-memory port config, then read from multiple tasks.  
   - **Output:** Before/after snapshots of port allocations and any conflicts detected.

4. **BPI-CORE-04: Wallet config + BPI config coherence**  
   - **Components:** `bpi_wallet_command`, `config`.  
   - **Flow:** Load wallet config and BPI config, ensure network domain and ports match expectations.  
   - **Output:** Side-by-side table of wallet vs BPI network fields, highlighting mismatches.

5. **BPI-CORE-05: Diagnostics self-check report**  
   - **Components:** `diagnostics`, `health`.  
   - **Flow:** Run a self-check that inspects key subsystems (config, storage, audit).  
   - **Output:** Structured list of checks with `OK` / `WARN` / `FAIL` and remediation hints.

### 1.2 Immutable Audit & VM Server (6 tests)

6. **BPI-CORE-06: Immutable audit append + readback**  
   - **Components:** `immutable_audit_system`, `logbook_6d_bridge`.  
   - **Flow:** Append a handful of runtime + security events, then query them.  
   - **Output:**
     - Printed events with IDs, timestamp, component type.  
     - Count of records per component.

7. **BPI-CORE-07: VM server HTTP cage basic route**  
   - **Components:** `vm_server`, `audit_http_server`.  
   - **Flow:** Start `VmServer` on random port, execute a simple HTTP cage route.  
   - **Output:**
     - Request/response summary (method, path, status).  
     - Any audit events created.

8. **BPI-CORE-08: HTTP cage demo app template served**  
   - **Components:** `vm_server`, `include_str!("../web_apps/httpcg_demo_app.html")`.  
   - **Flow:** Call the route that serves the demo HTML and assert the placeholder signature is present.  
   - **Output:** First N characters of rendered HTML and the final route status.

9. **BPI-CORE-09: Forensic firewall integration path**  
   - **Components:** `forensic_firewall::forensic_vm`, `immutable_audit_system`.  
   - **Flow:** Simulate an attack event and verify that forensic VM and audit system both record it.  
   - **Output:**
     - Attack event ID, type, severity.  
     - Mapping between forensic record and immutable audit record.

10. **BPI-CORE-10: Court node receives and classifies disputes**  
    - **Components:** `court_node`, `court_vm_audit`.  
    - **Flow:** Submit a few synthetic dispute cases and ensure court pipeline classifies them into queues.  
    - **Output:** Case IDs, queue assignment, and summary counts per category.

11. **BPI-CORE-11: Universal audit VM multi-component view**  
    - **Components:** `universal_audit_vm`, `immutable_audit_system`.  
    - **Flow:** Ingest mixed events (VM, BPI core, DockLock, ENC cluster) and query a unified view.  
    - **Output:**
      - Aggregated table: `component_type`, `events`, `last_timestamp`.  
      - Sanity check on event ordering.

### 1.3 Blockchain OS Kernel & QGC / 6D (9 tests)

12. **BPI-CORE-12: Kernel resource manager tracks CPU/memory**  
    - **Components:** `blockchain_os_kernel::resource_manager`.  
    - **Flow:** Allocate and release synthetic workloads; ensure utilization history records trends.  
    - **Output:** Human-readable utilization timeline (e.g., percentile CPU, memory over time).

13. **BPI-CORE-13: QGC consensus happy-path round**  
    - **Components:** `qgc_consensus`, `bpi_packet`.  
    - **Flow:** Simulate a small validator set; run through a full consensus round.  
    - **Output:**
      - States per validator (pre-vote, pre-commit, commit).  
      - Final committed block ID and proof.

14. **BPI-CORE-14: 6D logbook write + read cross-check**  
    - **Components:** `logbook_6d_bridge`, `six_d_blockchain`.  
    - **Flow:** Write N synthetic transactions and read them back by coordinate and by ID.  
    - **Output:**
      - Mapping from transaction IDs to 6D coordinates.  
      - Sample decoded record with proof summary.

15. **BPI-CORE-15: Multi-cloud storage layout sanity**  
    - **Components:** `distributed_storage`, `EnhancedCdnStorage`.  
    - **Flow:** Store a bundle; verify hash, size, and distribution map across providers.  
    - **Output:**
      - Logical structure: container ID, providers, regions.  
      - Human-readable confirmation of redundancy.

16. **BPI-CORE-16: Security supervisor firewall hook**  
    - **Components:** `os_security_supervisor`, `forensic_firewall`.  
    - **Flow:** Pass a sample HTTP cage request through the OS security supervisor; ensure alerts propagate.  
    - **Output:**
      - Decision (allow/deny/inspect) and any generated security events.

17. **BPI-CORE-17: SOAR engine incident classification**  
    - **Components:** `security::soar_engine`.  
    - **Flow:** Feed multiple security events; inspect incident classification and recommended playbooks.  
    - **Output:** Incident list with severity, confidence, and chosen playbook IDs.

18. **BPI-CORE-18: Deception technology honeyfile trigger**  
    - **Components:** `security::deception_technology`.  
    - **Flow:** Simulate a file access to a honeyfile path.  
    - **Output:** Alert record showing attacker fingerprint, trigger file, and suggested response.

19. **BPI-CORE-19: AgiDigitalNation storage integrity**  
    - **Components:** `agi_digital_nation_storage`.  
    - **Flow:** Write a small "citizen" record and verify retrieval and integrity metadata.  
    - **Output:** Record key, storage tier, and integrity flags.

20. **BPI-CORE-20: Immutable OS bridge endpoint mapping**  
    - **Components:** `blockchain_os_kernel::immutable_os_bridge`.  
    - **Flow:** Register a few OS services and verify mapping from BPI service names to OS endpoints.  
    - **Output:** Table of `bpi_service` → `os_service` mappings.

### 1.4 CLI & Node Coordinator flows (10 tests)

21. **BPI-CORE-21: `bpi-core node status` happy path**  
    - **Components:** `cli::commands::infra`, `node_coordinator`.  
    - **Flow:** Run the CLI status command against a mocked healthy node.  
    - **Output:** A small status dashboard with node type, health, and key metrics.

22. **BPI-CORE-22: Logbook node type with enc-cluster + docklock sources**  
    - **Components:** `bpi_node_coordinator`, `LogbookType`.  
    - **Flow:** Create a logbook node using `receipt_sources = ["http-cage","docklock","enc-cluster"]`.  
    - **Output:** Printed config of sources and any validation errors.

23. **BPI-CORE-23: Enc-cluster command wiring**  
    - **Components:** `commands::enc_cluster`.  
    - **Flow:** Run an ENC operation (e.g. `status`) via CLI; assert correct command-line is logged.  
    - **Output:** Operation name, exit status, and a human summary line.

24. **BPI-CORE-24: DockLock metrics JSON pretty-print**  
    - **Components:** `commands::docklock`.  
    - **Flow:** Simulate metrics; ensure they render as structured JSON for `--format json`.  
    - **Output:** Example JSON response with fields for container usage, witness entries, receipts.

25. **BPI-CORE-25: Domain registry health snapshot**  
    - **Components:** `httpcg_domain_registry`, `domain_authority_system`.  
    - **Flow:** Create a few domains and query health/usage endpoints.  
    - **Output:**
      - Domain list with pricing tier, resolution stats.  
      - Any staking metrics.

26. **BPI-CORE-26: Quantum entanglement transaction test**  
    - **Components:** `quantum_entanglement`.  
    - **Flow:** Create a transaction entanglement between two tx IDs and check the result.  
    - **Output:** Entanglement ID, type, and a short explanation of linkage.

27. **BPI-CORE-27: BPI packet encode/decode roundtrip**  
    - **Components:** `bpi_packet`, `crypto-primitives`.  
    - **Flow:** Build a realistic BPI packet, serialize, deserialize, and compare.  
    - **Output:** Hex digest of packet, equality result, and any mismatch fields.

28. **BPI-CORE-28: Mesh-native communication envelope**  
    - **Components:** `mesh_native_communication`.  
    - **Flow:** Create a ZeroCopyMessage and verify headers, compression, and priority.  
    - **Output:** Encoded header preview and routing decision.

29. **BPI-CORE-29: Pravyom integration summary ticket**  
    - **Components:** `pravyom_integration::summary_ticket_generator`.  
    - **Flow:** Feed a batch of audit/operation events and generate a summary ticket.  
    - **Output:** Ticket ID, key metrics (success/fail count), and a short narrative line.

30. **BPI-CORE-30: BPI service orchestrator progress steps**  
    - **Components:** `bpi_service_orchestrator`.  
    - **Flow:** Initialize orchestrator and walk through progress steps (wallet, auth, services).  
    - **Output:** Ordered list of progress messages with percentages.

---

## 2. BPCI / Pravyom Enterprise (20 tests)

### 2.1 CLI & Config (5 tests)

31. **BPCI-01: CLI logo + status banner**  
    - **Components:** `bpci-enterprise/src/cli/mod.rs`, shared `assets::logos`.  
    - **Flow:** Run `pravyom --format human status` and ensure the shared Vedic logo and status text render.  
    - **Output:** Logo block plus a structured status line with network and config dir.

32. **BPCI-02: Config load + validation**  
    - **Components:** `EnvIniParser`, `wallet_registry`.  
    - **Flow:** Load `config/` and assert key fields exist (cluster ID, network, DB).  
    - **Output:** Pretty table of loaded values and any missing keys.

33. **BPCI-03: Wallet subcommand basic flows**  
    - **Components:** `cli::wallet`, `wallet_registry`.  
    - **Flow:** Create a test wallet, list wallets, fetch one by ID in a dry-run mode.  
    - **Output:** Wallet IDs, public keys, and state changes.

34. **BPCI-04: Governance parameters snapshot**  
    - **Components:** `cli::governance`, governance core modules.  
    - **Flow:** Query governance state (quorum, roles, thresholds).  
    - **Output:** Printable summary of governance parameters.

35. **BPCI-05: Maintenance status**  
    - **Components:** `cli::maintenance`.  
    - **Flow:** Run a maintenance check command; ensure it surfaces any warnings.  
    - **Output:** Table of subsystem statuses (OK/WARN/FAIL) with descriptions.

### 2.2 Cluster Ledger & Nodes (8 tests)

36. **BPCI-06: Cluster ledger list domains**  
    - **Components:** `bpci_cluster_ledger_server` domain handlers.  
    - **Flow:** Insert a few domain records into `domain_registry`; query list endpoint.  
    - **Output:** JSON domain array (name, status, pricing, staking).

37. **BPCI-07: Cluster ledger get domain**  
    - **Components:** same as above.  
    - **Flow:** Query one domain by ID that exists and one that doesn’t.  
    - **Output:** Success response with full record, and a clear not-found JSON.

38. **BPCI-08: BPCI blockchain server health**  
    - **Components:** `bpci_blockchain_server`.  
    - **Flow:** Start server in test mode, call `/health` or similar endpoint.  
    - **Output:** Status JSON with chain height, peer count, and last block hash.

39. **BPCI-09: Consensus server round-trip**  
    - **Components:** `bpci-consensus-server`.  
    - **Flow:** Simulate a single block proposal and acceptance path.  
    - **Output:** State transitions and final commit summary.

40. **BPCI-10: Auction mempool server enqueues bids**  
    - **Components:** `bpci_auction_mempool_server`.  
    - **Flow:** Submit several bids, ensure ordering and deduplication logic.  
    - **Output:** Printed mempool contents and winning bid candidate.

41. **BPCI-11: BPI bridge registers cluster node**  
    - **Components:** `bpci_bpi_bridge`.  
    - **Flow:** Simulate handshake from BPI core; ensure node entry created in BPCI side.  
    - **Output:** Mapping of BPI node ID to BPCI cluster node record.

42. **BPCI-12: Shadow registry server lookup**  
    - **Components:** `bpci_shadow_registry_server`.  
    - **Flow:** Insert a mapping and query by key.  
    - **Output:** Key/value record and any TTL or versioning info.

43. **BPCI-13: Network server peer discovery**  
    - **Components:** `bpci_network_server`.  
    - **Flow:** Bring up a few fake peers, ensure discovery and peer list endpoint behave.  
    - **Output:** Peer list with addresses and roles.

### 2.3 Wallet, Governance, Mother Coin (7 tests)

44. **BPCI-14: Wallet registry end-to-end flow**  
    - **Components:** `wallet_registry` module + CLI.  
    - **Flow:** Register, query, and update a wallet, verifying invariants.  
    - **Output:** JSON snapshots before/after updates.

45. **BPCI-15: Governance vote lifecycle**  
    - **Components:** governance modules + CLI.  
    - **Flow:** Create proposal, cast votes, close proposal.  
    - **Output:** Timeline of proposal state changes and final tally.

46. **BPCI-16: Internal governance ticket issuance**  
    - **Components:** `internal_governance`.  
    - **Flow:** Issue internal tickets (e.g., ops tasks) and query their status.  
    - **Output:** Ticket IDs, assignees, and a status histogram.

47. **BPCI-17: Mother Coin distribution sample run**  
    - **Components:** `mother_coin` CLI module.  
    - **Flow:** Run a dry-run distribution for a small list of addresses.  
    - **Output:** Table of recipients, weights, and simulated amounts.

48. **BPCI-18: Mesh deploy manifest validation**  
    - **Components:** `mesh_deploy` CLI module.  
    - **Flow:** Load a sample mesh manifest; validate structure and references.  
    - **Output:** Validation report highlighting missing or inconsistent fields.

49. **BPCI-19: Web interface status command**  
    - **Components:** `cli::web`.  
    - **Flow:** Query status of web dashboard components (if enabled) in dry-run.  
    - **Output:** Component list and readiness state.

50. **BPCI-20: Maintenance backup + restore manifest**  
    - **Components:** `cli::maintenance`.  
    - **Flow:** Simulate creating and validating a backup manifest.  
    - **Output:** Manifest summary and any warnings about coverage.

---

## 3. Hermes / Mesh / Control Plane (10 tests)

51. **MESH-01: Dynaroute endpoint resolution**  
    - **Components:** `dynaroute_client`, `dynaroute_registry`.  
    - **Flow:** Register a couple of logical services and resolve them to URLs.  
    - **Output:** Logical → physical endpoint mapping.

52. **MESH-02: Control fedrate network balancing**  
    - **Components:** `control_fedrate_network`.  
    - **Flow:** Simulate nodes with differing loads and verify balancing decisions.  
    - **Output:** Per-node load scores and chosen routing decisions.

53. **MESH-03: VPods daemon registration**  
    - **Components:** `vpods_daemon`, `vpods_control_handler`.  
    - **Flow:** Register multiple vPods and ensure control handler sees them.  
    - **Output:** vPod list with resources and ring assignments.

54. **MESH-04: VPods DockLock integration handshake**  
    - **Components:** `vpods_docklock_integration`.  
    - **Flow:** Simulate handshake between vPods and DockLock control plane.  
    - **Output:** Connection state and any security tokens used.

55. **MESH-05: VPods Unix transport basic I/O**  
    - **Components:** `vpods_unix_transport`.  
    - **Flow:** Send a small buffer through the transport; check integrity.  
    - **Output:** Bytes sent/received, latency, and any errors.

56. **MESH-06: Mesh-native comm metrics**  
    - **Components:** `mesh_native_communication`.  
    - **Flow:** Simulate a small burst of messages and gather metrics.  
    - **Output:** Message counts, drops, and compression ratio summary.

57. **MESH-07: Immutable OS manager component list**  
    - **Components:** tools `immutable_os_manager`.  
    - **Flow:** Query list of core components and their readiness.  
    - **Output:** Table of components with readiness %, matching existing descriptions.

58. **MESH-08: Advanced downloader sanity**  
    - **Components:** `bpi-advanced-downloader` tool.  
    - **Flow:** Run a dry-run download of a couple of artifacts.  
    - **Output:** Which URLs would be fetched, destinations, and any missing pieces.

59. **MESH-09: Enc cluster download manifest**  
    - **Components:** downloader `DownloadTask` entries for `enc-cluster`.  
    - **Flow:** Inspect tasks and verify enc-cluster artifacts are present and coherent.  
    - **Output:** Task list and any missing enc-related artifacts.

60. **MESH-10: Mesh health rollup**  
    - **Components:** combination of mesh-related modules.  
    - **Flow:** Collect synthetic health info from multiple mesh pieces and display a rollup.  
    - **Output:** Single summary line "Mesh: OK/WARN/FAIL" plus contributing details.

---

## 4. BPI ↔ BPCI Communication (10 tests)

61. **LINK-01: BPI node registers with BPCI bridge**  
    - **Components:** `bpci_bpi_bridge`, `bpi_node_coordinator`.  
    - **Flow:** Simulate a BPI node announcing itself; ensure bridge records it.  
    - **Output:** Mapping `bpi_node_id` → `bpci_cluster_node_id`.

62. **LINK-02: Domain info from BPCI into BPI core**  
    - **Components:** BPCI cluster ledger domain API + BPI domain management client (once wired).  
    - **Flow:** Query domain info from BPI side via bridge.  
    - **Output:** Domain record as observed by BPI, including pricing and staking.

63. **LINK-03: Audit trail continuity across BPI/BPCI**  
    - **Components:** `immutable_audit_system`, BPCI audit writers.  
    - **Flow:** Generate an event in BPI, propagate a corresponding entry in BPCI, and cross-check IDs.  
    - **Output:** Pairing of BPI and BPCI audit IDs for the same logical event.

64. **LINK-04: Payment flow from BPI to BPCI**  
    - **Components:** BPI wallet side + `bpci_payment_server`.  
    - **Flow:** Simulate a simple payment instruction from BPI to BPCI.  
    - **Output:** Payment request ID, status, and ledger confirmation.

65. **LINK-05: ENC cluster + DockLock receipts across boundary**  
    - **Components:** BPI logbook + BPCI cluster ledger.  
    - **Flow:** Generate ENC/DockLock receipts in BPI, ensure BPCI can reference them.  
    - **Output:** Receipt IDs and cross-system linkage.

66. **LINK-06: Governance signals crossing layers**  
    - **Components:** BPI governance hooks + BPCI governance modules.  
    - **Flow:** Simulate a policy update from BPCI that BPI core respects (e.g., tightening a limit).  
    - **Output:** Before/after snapshot of the controlled parameter.

67. **LINK-07: Health status aggregation**  
    - **Components:** BPI health, BPCI health.  
    - **Flow:** Pull health snapshots from both sides and render a combined status.  
    - **Output:** One-line rollup plus per-system sublines.

68. **LINK-08: Wallet registry cross-check**  
    - **Components:** BPI wallet identity vs BPCI wallet registry.  
    - **Flow:** Ensure a given identity appears consistently in both systems.  
    - **Output:** Identity record comparison and any mismatch warnings.

69. **LINK-09: Error propagation path**  
    - **Components:** any BPI-to-BPCI API boundary.  
    - **Flow:** Force a controlled error on BPCI side and verify BPI displays a clear, user-facing error message.  
    - **Output:** Error stack from BPCI and simplified message shown by BPI.

70. **LINK-10: Shutdown/maintenance signal**  
    - **Components:** maintenance CLIs on both sides.  
    - **Flow:** Trigger a planned maintenance window from BPCI and verify BPI nodes go into correct mode.  
    - **Output:** Timeline of events (signal sent, nodes quiesced, maintenance state).

---

## 5. Implementation notes

- **Responsiveness:** Every test should print *what it is doing* and *what it observed*, not just `ok`.  
- **Repeatability:** Use deterministic IDs where possible (or print seeds), so failures can be reproduced.  
- **Non-destructive:** Default to in-memory / local-only data paths; avoid altering real production configs.  
- **Placement:**
  - BPI tests under `bpi-core/tests/infra_*.rs` or targeted binaries in `src/bin/` with `--test-mode` flags.
  - BPCI tests under `bpci-enterprise/tests/` or dedicated CLI integration tests.  
  - Mesh and link tests can live in cross-crate integration folders referencing both sides.

This plan can be implemented incrementally, starting with a minimal happy-path in each group (e.g., BPI-CORE-01, BPCI-01, MESH-01, LINK-01) and expanding outward.
