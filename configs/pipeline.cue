// Pravyom Standard Pipeline v1.0 Configuration
// Single source of truth for all pipeline parameters and thresholds

pipeline: {
  // Core thresholds and timing parameters
  thresholds: {
    recordsPerSegment: 1000
    segmentMaxDuration: "60s"
    poePerBpiBundle: 100
    bpiBundlesPerBpci: 100
    poeBundleMaxAge: "10m"
    bpciAuctionMaxAge: "60m"
    anomalySpikeFactor: 10
  }
  
  // VM configuration - 8 standardized VMs
  vms: {
    "vmapp01": {
      type: "VM-APP"
      description: "Client app execution"
      image: "app@biso#1.2.3"
    }
    "vmorch01": {
      type: "VM-ORCH"
      description: "Orchestration/controller"
      image: "orch@biso#1.2.3"
    }
    "vmcluster01": {
      type: "VM-CLUSTER"
      description: "Scheduler, vPod control"
      image: "cluster@biso#1.2.3"
    }
    "vmstorage01": {
      type: "VM-STORAGE"
      description: "CUEDB, bucket, FS adapters"
      image: "storage@biso#1.2.3"
    }
    "vmfirewall01": {
      type: "VM-FIREWALL"
      description: "Net policy, QLOCK/TLSLS enforcement"
      image: "firewall@biso#1.2.3"
    }
    "vmcourt01": {
      type: "VM-COURT"
      description: "Warrant/de-pseudonymization (sealed)"
      image: "court@biso#1.2.3"
    }
    "vmbiso01": {
      type: "VM-BISO"
      description: "Immutable OS services"
      image: "biso@biso#1.2.3"
    }
    "vmtrafficlight01": {
      type: "VM-TRAFFICLIGHT"
      description: "Resource governor RAM/CPU/I/O"
      image: "trafficlight@biso#1.2.3"
    }
  }
  
  // Cryptographic signing configuration
  signing: {
    record: {
      ed25519: true
      pqc: "dilithium2"
    }
    aggregate: {
      bls: true
      pqc_multi: true
    }
  }
  
  // Time synchronization and clock proofs
  time: {
    skewTolerance: "3s"
    roughtimeServers: [
      "time.cloudflare.com",
      "roughtime.intel.com"
    ]
  }
  
  // Storage and CID configuration
  storage: {
    ziplockPath: "/ziplock"
    cidBackend: ["cuedb", "ipfs", "filecoin"]
    redundancy: 3
    preimageVoucher: true
  }
  
  // Governance and treasury configuration
  governance: {
    treasurySplit: {
      treasury: 0.80
      partners: 0.20
    }
    courtQuorum: 5
    warrantTTL: "24h"
  }
  
  // ID format specifications
  idFormats: {
    recordId: "R-{YYYYMMDD}-{vmid}-{nonce16}"
    segmentId: "seg-{6digit}"
    ticketId: "ZT-{YYYYMMDD}-{HH:MM:SS}Z-batch-{6digit}"
    poeId: "POE-{timestampZ}-{6digit}"
    bpiBundleId: "BPIB-{timestampZ}-{6digit}"
    bpciAuctionId: "BPCIA-{timestampZ}-{6digit}"
  }
  
  // Performance targets for devnet demo
  performance: {
    ziplockWriteRps: 100000  // records/s/VM (CBOR)
    ticketCommitP95Ms: 400   // segment seal → BPI inclusion
    bundleSealP95Ms: 50      // 100-leaf Merkle + BLS aggregate
    cidRetrievalP999LocalMs: 150   // local pin
    cidRetrievalP999RemoteMs: 900  // remote
  }
  
  // Failure handling configuration
  failureHandling: {
    clockSkewTolerance: "±3s"
    partialBundleAllowed: true
    auctionTimeoutRetries: 3
    cidUnavailableGracePeriod: "1h"
  }
}
