# Pravyom – A Living Infrastructure for Community and Enterprise

Pravyom is a real, runnable infrastructure stack that connects community BPI
nodes with enterprise‑grade BPCI servers. It is meant to be inspected, not
believed: you can build it, run short demos, and read the logs and math that
explain what is happening.

---

## 1. The Essence of Pravyom

Pravyom is built around a simple but powerful separation of concerns:

- **BPI Plane – Community / Edge**  
  BPI OS nodes are run by community members. They contribute work, proofs, and
  resources. Economically, this is where bundles are born and where usage is
  measured.

- **BPCI Plane – Enterprise / Coordination**  
  BPCI servers run consensus, blockchain, auction, registry, and audit
  infrastructure. They settle, aggregate, and certify what the BPI plane does.

The real innovation is not a single algorithm, but the **bridge** between these
planes:

- BPI ↔ BPCI flows are explicit.  
- Fees, balances, and bundles have a **clear lifecycle** from community wallets
  through enterprise settlement and back.
- Every important step has a corresponding **log, report, or proof** in this
  repository.

Pravyom is deliberately opinionated: it treats mathematics, code, and runtime
behaviour as a single object. You are not asked to “trust the idea” – you are
invited to run the demos and inspect the evidence.

---

## 2. What Actually Exists in This Repository

At a high level, this repo gives you four concrete things:

1. **BPCI Enterprise Stack (Rust)**  
   Real servers for consensus, blockchain, auction mempool, auction DB,
   registries, wallet stamping, and stamped bank/government APIs. They are
   wired together via a constellation of HTTP ports and a unified networking
   layer.

2. **BPI–BPCI Economic Bridge**  
   Code that moves BPI balances through a BPCI constellation with explicit
   principal, gas, and bundle accounting. The lifecycle demo shows how a single
   BPI account’s free allocation is consumed by real or simulated flows.

3. **Hermès P2P Mesh**  
   A kappa‑aware Web‑4 mesh that models how BPCI servers can evolve from a
   centralized constellation into a fully decentralized, cellular P2P organism.
   The mesh and decentralization demos simulate this path on a single machine.

4. **Mathematical Foundations & Proof Logs**  
   A 49‑component mathematical index (6D blockchain, QLock, vPods, governance,
   economics, readiness metrics, and more), and a LaTeX proof log that embeds
   real demo reports as they are.

Each of these is backed by executable Rust binaries and human‑readable reports
under `/tmp` and `documentation/pdf`.

---

## 3. How a Professor or Reviewer Should Read This Project

If you are evaluating Pravyom academically or professionally, the recommended
path is:

1. **Start with the conceptual overview (Chapter 1).**  
   See `documentation/pdf/chapter1_pravyom_overview.tex` or the compiled
   `main_pravyom_report.pdf`. This chapter explains motivation, architecture,
   and research questions:
   - Why BPI and BPCI are separated.
   - How Hermès provides a decentralization path.
   - How demos, logs, and equations fit together.

2. **Inspect the proof log (Chapter 2).**  
   The file `documentation/pdf/bpci_proof_log.tex` (and its compiled chapter in
   `main_pravyom_report.pdf`) contains:
   - Exact `cargo run` commands for each demo.
   - The ports, URLs, and environment variables used.
   - Full, verbatim contents of demo reports from `/tmp`.
   - A short explanation of what each demo proves about the system.

3. **Drill into the mathematics (Chapter 3).**  
   The chapter `documentation/pdf/chapter3_pravyom_mathematical_foundations.tex`
   pulls in `02-pravyom-mathematical-foundations-49-equations.md`. This index
   shows where the real math lives in the codebase, including:
   - 6D coordinate mapping and placement proofs.
   - LCCD cell metabolism, division, and consensus thresholds.
   - QLock trigonometric identities and integrity hashes.
   - Auction economics, throughput targets, governance quorums, and readiness
     metrics.

4. **Finally, browse the Rust code.**  
   Once you see the demos and equations, you can inspect the concrete
   implementations under `bpi-core/src` and `bpci-enterprise/src`. Every major
   idea has at least one real code anchor.

---

## 4. Umesh + Saturn – Design Philosophy

Two complementary disciplines shape this project:

- **Umesh’s discipline:** Keep everything runnable, debuggable, and laptop‑safe.
  - Demos are one‑minute, not three‑hour clusters.
  - Logs live under `/tmp` and are easy to inspect and paste into LaTeX.
  - Components are wired with explicit ports and environment variables.

- **Saturn’s discipline:** Keep the theory clean and globally consistent.
  - The same 6D and consensus parameters appear in documentation, code, and
    demos.
  - Each “big word” (e.g., quantum sync, 6D coordinate, readiness index) is
    tied to a concrete equation and code location.

Pravyom only claims something once all three layers agree:

1. **Math:** There is a clear equation or invariant.  
2. **Code:** There is a concrete Rust implementation.  
3. **Evidence:** There is a demo and log showing it in action.

This is the standard we try to hold throughout the repository.

---

## 5. Quick Start for Running the Real Demos

For a hands‑on tour, see
`documentation/01-pravyom-quickstart-reviewer-guide.md`. In short:

1. **Build:**

   ```bash
   cd /home/umesh/metanode
   cargo build -p bpi-core -p bpci-enterprise
   ```

2. **Run the constellation control helper:**

   ```bash
   cargo run -p pravyom-enterprise --bin bpci_constellation_control
   ```

   This prints recommended env vars and `cargo run` commands for all
   BPCI components.

3. **Start the constellation services** (consensus, blockchain, auction
   mempool, auction DB, bridge) using the printed commands.

4. **Run the demos:**

   - BPCI Constellation 1‑Minute Demo:

     ```bash
     cargo run -p pravyom-enterprise --bin bpci_constellation_demo
     ```

   - BPI ↔ BPCI Lifecycle Demo:

     ```bash
     cargo run -p pravyom-enterprise --bin bpi_bpci_lifecycle_demo
     ```

   - Hermès Mesh Demo:

     ```bash
     cargo run -p pravyom-enterprise --bin hermes_mesh_demo
     ```

   - Hermès Decentralization Demo:

     ```bash
     cargo run -p pravyom-enterprise --bin hermes_decentralization_demo
     ```

5. **Inspect the reports under `/tmp`** and the compiled LaTeX proof log under
   `documentation/pdf/main_pravyom_report.pdf`.

---

## 6. How to Use This Repo in Serious Work

- **As a reference implementation:**  
  Use the BPCI constellation, Hermès mesh, and 6D/QLock math as a blueprint for
  building your own infrastructure, or as a benchmark to compare against.

- **As a research artefact:**  
  Cite the mathematical foundations and proof logs when discussing new
  consensus, audit, or P2P designs. The goal is to provide a concrete, inspectable
  baseline rather than a purely theoretical proposal.

- **As a teaching tool:**  
  Walk students through the path from equations → Rust structs → running demos →
  logs and LaTeX. Pravyom is designed to make that end‑to‑end story visible.

If you are reading this as a professor or advanced reviewer, you should expect
that every strong claim in the surrounding documentation has a corresponding
piece of code and a reproducible log somewhere in this tree. That is the
standard Pravyom holds itself to.
