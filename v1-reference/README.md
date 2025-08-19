# Metanode v1.0 Reference Implementation

This is the deployable, testable reference implementation of Metanode v1.0 following the complete blueprint specification.

## Repository Structure

```
v1-reference/
├── README.md                    # This file
├── docker-compose.dev.yml       # Development deployment
├── Cargo.toml                   # Workspace configuration
├── src/
│   ├── lib.rs                   # Core library exports
│   ├── types.rs                 # Data structures (StepReceipt, LogBlock, PoE)
│   ├── crypto.rs                # Ed25519, BLS, Blake3 implementations
│   └── math/
│       ├── mod.rs               # PoE math module
│       ├── phi_gamma.rs         # Φ/Γ calculator with golden tests
│       └── economics.rs         # NEX minting, fee splits
├── components/
│   ├── cage/                    # HTTP Cage (unified ingress)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   └── src/handlers.rs
│   ├── court/                   # Court Node (YAML policy engine)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/compiler.rs      # YAML → JSON state machine
│   │   └── policies/
│   │       └── appA.yaml        # Example agreement
│   ├── bpi/                     # BPI nodes (comm, validators)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/comm.rs          # BPI-comm gateway
│   │   ├── src/validator.rs     # IBFT validator
│   │   └── src/ibft.rs          # IBFT consensus
│   ├── enc/                     # ENC cluster (notary, validator)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/notary.rs        # Receipt → LogBlock aggregation
│   │   ├── src/validator.rs     # ENC validator
│   │   └── src/scheduler.rs     # K8s++ scheduler
│   ├── bpci/                    # BPCI HQ (final IBFT ring)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/consensus.rs     # BPCI IBFT
│   │   └── src/mempool.rs       # PoE bundle processing
│   ├── docklock/                # DockLock runtime
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/runtime.rs       # Container runtime
│   │   └── src/receipts.rs      # StepReceipt generation
│   ├── bank/                    # Bank Mesh (fake settlement)
│   │   ├── Cargo.toml
│   │   ├── src/main.rs
│   │   ├── src/settlement.rs    # Fake ACH/SWIFT/Wire
│   │   └── src/tokens.rs        # GEN/NEX/FLX/AUR
│   └── blockbook/               # Audit storage & indexing
│       ├── Cargo.toml
│       ├── src/main.rs
│       └── src/indexer.rs
├── sdk/
│   ├── rust/                    # Rust SDK
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── typescript/              # TypeScript SDK
│       ├── package.json
│       └── src/index.ts
├── cli/
│   ├── Cargo.toml
│   └── src/main.rs              # metanode CLI tool
├── tests/
│   ├── integration/
│   │   ├── determinism_test.rs  # 10k DockLock ops → identical roots
│   │   ├── ibft_chaos_test.rs   # Validator partition tests
│   │   ├── billing_test.rs      # Φ/Γ reproducibility
│   │   ├── court_upgrade_test.rs # Policy rollback tests
│   │   ├── bank_settlement_test.rs # Fiat settlement tests
│   │   └── throughput_test.rs   # 10k TPS PoE bundles
│   └── golden/
│       ├── phi_gamma_vectors.json # Golden test vectors
│       └── policy_examples.yaml
└── docs/
    ├── DEPLOYMENT.md            # Deployment guide
    ├── API.md                   # HTTP Cage API reference
    └── ONBOARDING.md            # <30min developer onboarding
```

## Quick Start (Development)

```bash
# Clone and build
git clone <repo> && cd v1-reference
cargo build --release

# Start development cluster
docker-compose -f docker-compose.dev.yml up

# Run integration tests
cargo test --test integration

# Generate keys and submit first receipt
./target/release/metanode keys generate
./target/release/metanode receipts submit --app APP_A --container test-001
```

## SLO Targets

- API p95 < 100ms, p99 < 250ms
- Consensus finality p95 < 3s
- Container cold start p95 < 500ms  
- Receipt throughput: 1k/s sustained, 5k/s burst
- HA: 99.99% uptime

## Architecture Overview

5-node BPCI attachment per app + shared BPCI-HQ backbone:
- **HTTP Cage**: Unified ingress with auth/rate limiting
- **DockLock**: Container runtime with <500ms start, <10MB overhead
- **ENC Cluster**: K8s++ scheduler with receipt aggregation
- **BPI Network**: IBFT consensus with <3s finality
- **Court Node**: YAML smart contracts with deterministic execution
- **Bank Mesh**: Multi-token settlement (GEN/NEX/FLX/AUR)
- **BPCI HQ**: Final consensus ring with enterprise APIs

Ready for deployment and testing!
