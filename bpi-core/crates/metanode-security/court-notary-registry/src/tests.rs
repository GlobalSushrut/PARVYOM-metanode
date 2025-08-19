use super::*;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_cnr_creation() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);
    
    let stats = cnr.get_stats().await;
    assert_eq!(stats.total_notaries, 0);
    assert_eq!(stats.total_documents, 0);
    assert_eq!(stats.active_disputes, 0);
}

#[tokio::test]
async fn test_notary_registration() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    let notary = create_test_notary();
    let result = cnr.register_notary(notary.clone()).await.unwrap();
    assert!(result.success);
    assert!(result.message.contains("registered successfully"));

    let stats = cnr.get_stats().await;
    assert_eq!(stats.total_notaries, 1);
    assert_eq!(stats.active_notaries, 1);

    // Verify notary can be retrieved
    let retrieved_notary = cnr.get_notary(notary.id).await.unwrap();
    assert_eq!(retrieved_notary.name, notary.name);
}

#[tokio::test]
async fn test_notary_registration_disabled() {
    let mut config = CnrConfig::default();
    config.notary_registration_enabled = false;
    let cnr = CourtNotaryRegistry::new(config);

    let notary = create_test_notary();
    let result = cnr.register_notary(notary).await.unwrap();
    assert!(!result.success);
    assert!(result.message.contains("disabled"));
}

#[tokio::test]
async fn test_document_notarization() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // First register a notary
    let notary = create_test_notary();
    cnr.register_notary(notary.clone()).await.unwrap();

    // Now notarize a document
    let document = create_test_document(notary.id);
    let result = cnr.notarize_document(document.clone()).await.unwrap();
    assert!(result.success);
    assert!(result.message.contains("notarized successfully"));

    let stats = cnr.get_stats().await;
    assert_eq!(stats.total_documents, 1);

    // Verify document can be retrieved
    let retrieved_document = cnr.get_document(document.id).await.unwrap();
    assert_eq!(retrieved_document.metadata.title, document.metadata.title);
}

#[tokio::test]
async fn test_document_notarization_inactive_notary() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // Register an inactive notary
    let mut notary = create_test_notary();
    notary.status = NotaryStatus::Suspended;
    cnr.register_notary(notary.clone()).await.unwrap();

    // Try to notarize a document
    let document = create_test_document(notary.id);
    let result = cnr.notarize_document(document).await.unwrap();
    assert!(!result.success);
    assert!(result.message.contains("not active"));
}

#[tokio::test]
async fn test_document_verification() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // Register notary and notarize document
    let notary = create_test_notary();
    cnr.register_notary(notary.clone()).await.unwrap();

    let document = create_test_document(notary.id);
    let document_id = document.id;
    cnr.notarize_document(document).await.unwrap();

    // Verify the document
    let is_verified = cnr.verify_document(document_id).await.unwrap();
    assert!(is_verified);
}

#[tokio::test]
async fn test_document_verification_inactive_document() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // Register notary and notarize document
    let notary = create_test_notary();
    cnr.register_notary(notary.clone()).await.unwrap();

    let mut document = create_test_document(notary.id);
    document.status = DocumentStatus::Expired;
    let document_id = document.id;
    cnr.notarize_document(document).await.unwrap();

    // Verify the document (should fail)
    let is_verified = cnr.verify_document(document_id).await.unwrap();
    assert!(!is_verified);
}

#[tokio::test]
async fn test_dispute_resolution() {
    let resolver = DisputeResolver::new();

    let dispute = create_test_dispute();
    let dispute_id = resolver.file_dispute(dispute.clone()).await.unwrap();
    assert_eq!(dispute_id, dispute.id);

    // Verify dispute can be retrieved
    let retrieved_dispute = resolver.get_dispute(dispute_id).await.unwrap();
    assert_eq!(retrieved_dispute.reason, dispute.reason);

    // Resolve the dispute
    resolver.resolve_dispute(dispute_id, "Resolved in favor of plaintiff".to_string()).await.unwrap();

    // Verify dispute status changed
    let resolved_dispute = resolver.get_dispute(dispute_id).await.unwrap();
    assert_eq!(resolved_dispute.status, DisputeStatus::Resolved);
}

#[tokio::test]
async fn test_arbitrator_management() {
    let resolver = DisputeResolver::new();

    let arbitrator = create_test_arbitrator();
    resolver.add_arbitrator(arbitrator.clone()).await.unwrap();

    // Verify arbitrator can be retrieved
    let retrieved_arbitrator = resolver.get_arbitrator(arbitrator.id).await.unwrap();
    assert_eq!(retrieved_arbitrator.name, arbitrator.name);
}

#[tokio::test]
async fn test_legal_compliance() {
    let compliance_engine = LegalComplianceEngine::new();

    let notary = create_test_notary();

    // Test US compliance
    let is_compliant_us = compliance_engine.check_compliance("US", &notary);
    assert!(is_compliant_us);

    // Test EU compliance
    let is_compliant_eu = compliance_engine.check_compliance("EU", &notary);
    assert!(is_compliant_eu);

    // Test unknown jurisdiction
    let is_compliant_unknown = compliance_engine.check_compliance("UNKNOWN", &notary);
    assert!(!is_compliant_unknown);
}

#[tokio::test]
async fn test_compliance_inactive_notary() {
    let compliance_engine = LegalComplianceEngine::new();

    let mut notary = create_test_notary();
    notary.status = NotaryStatus::Revoked;

    let is_compliant = compliance_engine.check_compliance("US", &notary);
    assert!(!is_compliant);
}

#[tokio::test]
async fn test_verification_service() {
    let verification_service = VerificationService::new();

    // Test digital signature verification
    let is_verified = verification_service.verify("digital_signature", "test_signature_data");
    assert!(is_verified);

    // Test timestamp proof verification
    let is_verified = verification_service.verify("timestamp_proof", "test_timestamp_data");
    assert!(is_verified);

    // Test unknown method
    let is_verified = verification_service.verify("unknown_method", "test_data");
    assert!(!is_verified);
}

#[tokio::test]
async fn test_verification_method_retrieval() {
    let verification_service = VerificationService::new();

    let method = verification_service.get_method("digital_signature").unwrap();
    assert_eq!(method.id, "digital_signature");
    assert!(method.reliability_score > 0.9);

    let unknown_method = verification_service.get_method("unknown_method");
    assert!(unknown_method.is_none());
}

#[tokio::test]
async fn test_trust_anchor_management() {
    let verification_service = VerificationService::new();

    let anchor = verification_service.get_trust_anchor("metanode_root_ca").unwrap();
    assert_eq!(anchor.authority, "Metanode Root Certificate Authority");

    let is_valid = verification_service.is_trust_anchor_valid("metanode_root_ca");
    assert!(is_valid);
}

#[tokio::test]
async fn test_cnr_list_operations() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // Register multiple notaries
    let notary1 = create_test_notary();
    let notary2 = create_test_notary();
    cnr.register_notary(notary1.clone()).await.unwrap();
    cnr.register_notary(notary2.clone()).await.unwrap();

    // Notarize multiple documents
    let document1 = create_test_document(notary1.id);
    let document2 = create_test_document(notary2.id);
    cnr.notarize_document(document1).await.unwrap();
    cnr.notarize_document(document2).await.unwrap();

    // Test list operations
    let notary_list = cnr.list_notaries().await;
    assert_eq!(notary_list.len(), 2);

    let document_list = cnr.list_documents().await;
    assert_eq!(document_list.len(), 2);
}

#[tokio::test]
async fn test_stage50_exit_criteria() {
    let config = CnrConfig::default();
    let cnr = CourtNotaryRegistry::new(config);

    // Test 1: Notary registration and management
    let notary = create_test_notary();
    let result = cnr.register_notary(notary.clone()).await.unwrap();
    assert!(result.success);

    // Test 2: Document notarization
    let document = create_test_document(notary.id);
    let document_id = document.id;
    let result = cnr.notarize_document(document).await.unwrap();
    assert!(result.success);

    // Test 3: Document verification
    let is_verified = cnr.verify_document(document_id).await.unwrap();
    assert!(is_verified);

    // Test 4: Dispute resolution system
    let resolver = DisputeResolver::new();
    let dispute = create_test_dispute();
    let dispute_id = resolver.file_dispute(dispute).await.unwrap();
    resolver.resolve_dispute(dispute_id, "Test resolution".to_string()).await.unwrap();

    // Test 5: Legal compliance checking
    let compliance_engine = LegalComplianceEngine::new();
    let is_compliant = compliance_engine.check_compliance("US", &notary);
    assert!(is_compliant);

    // Test 6: Verification services
    let verification_service = VerificationService::new();
    let is_verified = verification_service.verify("digital_signature", "test_data");
    assert!(is_verified);

    // Verify statistics
    let stats = cnr.get_stats().await;
    assert!(stats.total_notaries > 0);
    assert!(stats.total_documents > 0);
    assert!(stats.active_notaries > 0);

    println!("🎉 Stage 50 exit criteria verified successfully!");
}

// Helper functions for creating test data

fn create_test_notary() -> RegisteredNotary {
    RegisteredNotary {
        id: Uuid::new_v4(),
        name: "Test Notary Public".to_string(),
        credentials: NotaryCredentials {
            license_number: "NP123456".to_string(),
            issuing_authority: "State Notary Commission".to_string(),
            expires_at: Utc::now() + chrono::Duration::days(365),
            verified: true,
            credential_hash: "sha256_credential_hash".to_string(),
        },
        registered_at: Utc::now(),
        status: NotaryStatus::Active,
        jurisdiction: "US".to_string(),
        public_key: "ed25519_public_key_placeholder".to_string(),
        document_count: 0,
        reputation_score: 4.8,
    }
}

fn create_test_document(notary_id: Uuid) -> NotarizedDocument {
    NotarizedDocument {
        id: Uuid::new_v4(),
        document_hash: "sha256_document_hash".to_string(),
        metadata: DocumentMetadata {
            title: "Test Legal Contract".to_string(),
            document_type: DocumentType::Contract,
            parties: vec!["Alice Corp".to_string(), "Bob LLC".to_string()],
            legal_significance: LegalSignificance::High,
            retention_years: 7,
        },
        notarization: NotarizationDetails {
            notary_id,
            notarized_at: Utc::now(),
            notary_signature: "ed25519_notary_signature".to_string(),
            witness_signatures: vec!["witness1_signature".to_string()],
            location: "New York, NY".to_string(),
            method: NotarizationMethod::Digital,
        },
        proofs: vec![
            VerificationProof {
                id: Uuid::new_v4(),
                proof_type: ProofType::CryptographicSignature,
                proof_data: "ed25519_signature_proof".to_string(),
                verified_at: Utc::now(),
                verifier_id: "metanode_verifier".to_string(),
            },
        ],
        status: DocumentStatus::Active,
        created_at: Utc::now(),
    }
}

fn create_test_dispute() -> Dispute {
    Dispute {
        id: Uuid::new_v4(),
        document_id: Uuid::new_v4(),
        parties: vec!["Plaintiff Corp".to_string(), "Defendant LLC".to_string()],
        reason: DisputeReason::DocumentAuthenticity,
        evidence: vec![
            Evidence {
                id: Uuid::new_v4(),
                evidence_type: EvidenceType::Document,
                data: "evidence_document_hash".to_string(),
                submitted_by: "Plaintiff Corp".to_string(),
                submitted_at: Utc::now(),
            },
        ],
        status: DisputeStatus::Open,
        created_at: Utc::now(),
        arbitrator_id: None,
    }
}

fn create_test_arbitrator() -> Arbitrator {
    Arbitrator {
        id: Uuid::new_v4(),
        name: "Judge Sarah Wilson".to_string(),
        qualifications: vec![
            "Juris Doctor".to_string(),
            "Certified Arbitrator".to_string(),
            "20 years experience".to_string(),
        ],
        specializations: vec![
            "Contract Law".to_string(),
            "Digital Evidence".to_string(),
        ],
        reputation_score: 4.9,
        active_cases: 3,
    }
}
