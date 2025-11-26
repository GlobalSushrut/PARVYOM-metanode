# Pravyom – A Living Infrastructure for Community and Enterprise

This repository is the **Pravyom Metanode**: a real, runnable BPI/BPCI/Hermès
infrastructure stack. It is meant to be inspected, not believed:

- You can build it and run short, laptop‑safe demos.
- You can read the logs and mathematical foundations that explain what is
  happening.
- Every major claim is tied to code, a demo, and a proof/log.

For the full, professor‑grade README (architecture, demos, and math), see:

- [`documentation/README.md`](documentation/README.md)
- [`documentation/pdf/main_pravyom_report.pdf`](documentation/pdf/main_pravyom_report.pdf)
- [`documentation/02-pravyom-mathematical-foundations-49-equations.md`](documentation/02-pravyom-mathematical-foundations-49-equations.md)

## Quick start

From the repo root:

```bash
cd /home/umesh/metanode
cargo build -p bpi-core -p bpci-enterprise
```

Then follow:

- [`documentation/01-pravyom-quickstart-reviewer-guide.md`](documentation/01-pravyom-quickstart-reviewer-guide.md)

for a curated 10‑minute review flow and exact demo commands.
