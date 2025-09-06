# Logic Architecture: `.cpdf` and `.zkzip` Implementation

## Core Logic Flow

### `.cpdf` Creation Logic

```
1. Document Input Processing
   ├── Parse source document (PDF, DOCX, etc.)
   ├── Extract pages as rendered bitmap/vector data
   ├── Generate entropy fingerprints per page
   └── Create global document fingerprint

2. Cryptographic Layer Generation
   ├── Generate owner keypair (ECDSA/Falcon)
   ├── Sign document with private key
   ├── Create ZK proof of authorship
   └── Generate page-level signatures

3. Protection Layer Implementation
   ├── Inject pixel-level noise for screenshot detection
   ├── Create XOR-encoded RGB deltas
   ├── Embed invisible watermarks
   └── Set copy/edit protection flags

4. Assembly & Packaging
   ├── Construct CPDF header
   ├── Package page blocks with signatures
   ├── Build action log structure
   ├── Generate Merkle audit trail
   └── Compress with Zstandard
```

### `.zkzip` Creation Logic

```
1. Archive Preparation
   ├── Validate input .cpdf files
   ├── Generate archive UUID
   ├── Determine unlock protocol (password/DAO/hybrid)
   └── Create vault table structure

2. Encryption Process
   ├── Generate AES-256 master key
   ├── Encrypt each .cpdf file individually
   ├── Create encrypted metadata index
   └── Generate entropy fingerprints

3. ZK Proof Generation
   ├── Create password hash ZK proof
   ├── Generate DAO multisig ZK proof
   ├── Combine proofs for hybrid mode
   └── Validate proof size constraints

4. Archive Assembly
   ├── Package encrypted packets
   ├── Seal vault table with zkSig
   ├── Generate footer hash
   └── Apply ZK barrier
```

## Verification Logic

### `.cpdf` Verification

```rust
fn verify_cpdf(file_path: &str) -> Result<VerificationResult, Error> {
    // 1. Header validation
    let header = parse_header(file_path)?;
    validate_format_version(&header)?;
    
    // 2. Signature verification
    let owner_cluster = extract_owner_cluster(file_path)?;
    verify_creator_signature(&owner_cluster)?;
    
    // 3. Page integrity check
    for page in extract_pages(file_path)? {
        verify_page_signature(&page)?;
        validate_entropy_fingerprint(&page)?;
    }
    
    // 4. Audit trail validation
    let audit_trail = extract_audit_trail(file_path)?;
    verify_merkle_chain(&audit_trail)?;
    
    // 5. ZK proof verification
    let zk_layer = extract_zk_layer(file_path)?;
    verify_zk_proofs(&zk_layer)?;
    
    Ok(VerificationResult::Valid)
}
```

### `.zkzip` Unlock Logic

```rust
fn unlock_zkzip(archive_path: &str, unlock_method: UnlockMethod) -> Result<Vec<CpdfFile>, Error> {
    // 1. Parse archive header
    let header = parse_zkzip_header(archive_path)?;
    
    // 2. Verify ZK barrier
    match unlock_method {
        UnlockMethod::Password(pwd) => {
            let proof = generate_password_proof(&pwd)?;
            verify_password_zk_proof(&header, &proof)?;
        },
        UnlockMethod::DAO(signatures) => {
            verify_multisig_threshold(&header, &signatures)?;
        },
        UnlockMethod::Hybrid(pwd, sigs) => {
            verify_password_zk_proof(&header, &generate_password_proof(&pwd)?)?;
            verify_partial_multisig(&header, &sigs)?;
        }
    }
    
    // 3. Extract vault keys
    let vault_table = decrypt_vault_table(&header, &unlock_method)?;
    
    // 4. Decrypt and extract files
    let mut extracted_files = Vec::new();
    for packet in extract_encrypted_packets(archive_path)? {
        let decrypted = decrypt_packet(&packet, &vault_table)?;
        extracted_files.push(parse_cpdf_from_bytes(&decrypted)?);
    }
    
    Ok(extracted_files)
}
```

## Screenshot Detection Logic

### Pixel Entropy Mapping

```rust
fn inject_screenshot_protection(page: &mut PageData) -> Result<(), Error> {
    // 1. Generate noise pattern based on document hash
    let noise_seed = generate_noise_seed(&page.content_hash);
    
    // 2. Create XOR-encoded RGB deltas
    for pixel in page.pixels.iter_mut() {
        let noise = generate_pixel_noise(&noise_seed, pixel.position);
        pixel.r ^= noise.r_delta;
        pixel.g ^= noise.g_delta;
        pixel.b ^= noise.b_delta;
    }
    
    // 3. Embed invisible watermark
    embed_watermark(&mut page.pixels, &page.owner_signature)?;
    
    // 4. Create detection signature
    page.screenshot_signature = generate_screenshot_signature(&page.pixels);
    
    Ok(())
}

fn detect_screenshot(image_data: &[u8], original_cpdf: &CpdfFile) -> DetectionResult {
    // Extract potential noise patterns from image
    let extracted_noise = extract_noise_patterns(image_data);
    
    // Compare with original document signatures
    for page in &original_cpdf.pages {
        if noise_pattern_matches(&extracted_noise, &page.screenshot_signature) {
            return DetectionResult::Detected {
                page_number: page.number,
                confidence: calculate_confidence(&extracted_noise, &page.screenshot_signature),
                owner: page.owner_signature.clone(),
            };
        }
    }
    
    DetectionResult::NotDetected
}
```

## ZK Proof Logic

### Circuit Design for Document Authorship

```circom
pragma circom 2.0.0;

template DocumentAuthorship() {
    // Private inputs
    signal private input document_hash;
    signal private input private_key;
    signal private input timestamp;
    
    // Public inputs
    signal input public_key;
    signal input document_fingerprint;
    
    // Outputs
    signal output is_valid_author;
    
    // Verify private key corresponds to public key
    component key_verifier = ECDSAVerify();
    key_verifier.private_key <== private_key;
    key_verifier.public_key <== public_key;
    
    // Verify document hash matches fingerprint
    component hash_verifier = SHA3_512();
    hash_verifier.input <== document_hash;
    hash_verifier.output === document_fingerprint;
    
    // Verify signature
    component signature_verifier = SignatureVerify();
    signature_verifier.message <== document_hash;
    signature_verifier.private_key <== private_key;
    signature_verifier.timestamp <== timestamp;
    
    is_valid_author <== signature_verifier.valid;
}
```

## Action Log Logic

### Delta Tracking

```rust
struct ActionLog {
    entries: Vec<ActionEntry>,
    merkle_root: Hash,
}

struct ActionEntry {
    action_type: ActionType,
    timestamp: u64,
    signature: Signature,
    delta_hash: Hash,
    zk_proof: ZKProof,
}

enum ActionType {
    PageModification { page_number: u32, delta: PageDelta },
    CopyAttempt { source_trace: UUID, destination: String },
    ScreenshotAttempt { detection_confidence: f64 },
    ViewAccess { viewer_fingerprint: Hash },
}

impl ActionLog {
    fn add_entry(&mut self, entry: ActionEntry) -> Result<(), Error> {
        // Verify entry signature
        verify_action_signature(&entry)?;
        
        // Update Merkle tree
        self.entries.push(entry);
        self.merkle_root = calculate_merkle_root(&self.entries);
        
        Ok(())
    }
    
    fn verify_integrity(&self) -> Result<bool, Error> {
        let calculated_root = calculate_merkle_root(&self.entries);
        Ok(calculated_root == self.merkle_root)
    }
}
```

## Error Handling & Recovery

### Graceful Degradation

```rust
enum CpdfError {
    CorruptedHeader,
    InvalidSignature,
    ZKProofFailure,
    ScreenshotDetected,
    UnauthorizedAccess,
}

fn handle_cpdf_error(error: CpdfError, file: &CpdfFile) -> RecoveryAction {
    match error {
        CpdfError::CorruptedHeader => RecoveryAction::AttemptRepair,
        CpdfError::InvalidSignature => RecoveryAction::ShowWatermarkedVersion,
        CpdfError::ZKProofFailure => RecoveryAction::RequireRevalidation,
        CpdfError::ScreenshotDetected => RecoveryAction::LogAndContinue,
        CpdfError::UnauthorizedAccess => RecoveryAction::DenyAccess,
    }
}
```

## Performance Optimization

### Lazy Loading Strategy

```rust
struct CpdfFile {
    header: CpdfHeader,
    pages: LazyVec<PageData>,
    action_log: LazyActionLog,
    zk_proofs: LazyZKProofs,
}

impl CpdfFile {
    fn load_page(&mut self, page_number: u32) -> Result<&PageData, Error> {
        if !self.pages.is_loaded(page_number) {
            let page_data = self.decrypt_and_verify_page(page_number)?;
            self.pages.insert(page_number, page_data);
        }
        Ok(self.pages.get(page_number).unwrap())
    }
}
```

This logic architecture ensures that both `.cpdf` and `.zkzip` formats maintain cryptographic integrity while providing practical performance and usability.
