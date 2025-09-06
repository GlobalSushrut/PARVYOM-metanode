use anyhow::Result;
use tokio;
use wallet_identity::*;
use wallet_identity::client::transport::cross_domain_httpcg::*;

/// Comprehensive integration test for cross-domain httpcg protocol
/// Tests httpcg://google.com, httpcg://amazon.com with wallet location integration
#[tokio::test]
async fn test_cross_domain_httpcg_integration() -> Result<()> {
    println!("🌐 Testing Cross-Domain httpcg Protocol Integration");

    // 1. Create wallet with location-aware DID
    let mut wallet = create_test_wallet_with_location("US").await?;
    
    // 2. Initialize cross-domain httpcg client
    let client = CrossDomainHttpcgClient::new(wallet.clone()).await?;
    println!("✅ Cross-domain httpcg client initialized");

    // 3. Test cross-domain requests to major web2 services
    test_google_cross_domain(&client).await?;
    test_amazon_cross_domain(&client).await?;
    test_microsoft_cross_domain(&client).await?;
    
    // 4. Test ERB (Excess Resource Billing) scenarios
    test_erb_billing_scenarios(&client).await?;
    
    // 5. Test jurisdiction compliance
    test_jurisdiction_compliance(&client).await?;
    
    // 6. Test wallet location integration
    test_wallet_location_integration(&client).await?;

    println!("🎉 All cross-domain httpcg integration tests passed!");
    Ok(())
}

async fn create_test_wallet_with_location(location: &str) -> Result<WalletIdentity> {
    let mut wallet = WalletIdentity::new_random(WalletProvider::BPI).await?;
    
    // Add location-aware DID
    wallet.did = Some(format!("did:bpi:{}:test-wallet-{}", location.to_lowercase(), uuid::Uuid::new_v4()));
    
    // Add required capabilities
    wallet.capabilities.push(WalletCapability::SecureMessaging);
    wallet.capabilities.push(WalletCapability::CrossDomainAccess);
    wallet.capabilities.push(WalletCapability::ERBBilling);
    
    println!("📱 Created test wallet with location: {} (DID: {})", location, wallet.did.as_ref().unwrap());
    Ok(wallet)
}

async fn test_google_cross_domain(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("🔍 Testing cross-domain request to Google...");
    
    // Test httpcg://google.com with wallet integration
    let response = client.get_cross_domain("https://google.com/search?q=pravyom").await?;
    
    // Validate response structure
    assert_eq!(response.httpcg_response.status, 200);
    assert!(response.jurisdiction_info.cross_border_allowed);
    assert_eq!(response.jurisdiction_info.country_code, "US");
    assert_eq!(response.wallet_location, "US");
    assert!(response.cross_domain_metadata.jurisdiction_validated);
    
    println!("✅ Google cross-domain request successful");
    println!("   Status: {}", response.httpcg_response.status);
    println!("   Jurisdiction: {}", response.jurisdiction_info.country_code);
    println!("   Wallet Location: {}", response.wallet_location);
    
    Ok(())
}

async fn test_amazon_cross_domain(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("🛒 Testing cross-domain request to Amazon...");
    
    // Test httpcg://amazon.com with POST request
    let request_body = serde_json::json!({
        "search_query": "blockchain books",
        "wallet_preference": "crypto_payment"
    });
    
    let response = client.post_cross_domain(
        "https://amazon.com/api/search",
        serde_json::to_vec(&request_body)?.as_slice()
    ).await?;
    
    // Validate response
    assert_eq!(response.httpcg_response.status, 200);
    assert!(response.cross_domain_metadata.jurisdiction_validated);
    
    println!("✅ Amazon cross-domain POST request successful");
    println!("   Response size: {} bytes", response.httpcg_response.body.len());
    
    Ok(())
}

async fn test_microsoft_cross_domain(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("💼 Testing cross-domain request to Microsoft...");
    
    // Test httpcg://microsoft.com with custom headers
    let response = client.get_cross_domain("https://microsoft.com/api/azure/status").await?;
    
    // Validate response
    assert_eq!(response.httpcg_response.status, 200);
    assert!(response.jurisdiction_info.cross_border_allowed);
    
    println!("✅ Microsoft cross-domain request successful");
    
    Ok(())
}

async fn test_erb_billing_scenarios(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("💰 Testing ERB (Excess Resource Billing) scenarios...");
    
    // Test compute ERB billing
    let compute_response = client.get_with_erb(
        "https://google.com/compute/heavy-task",
        ERBType::ComputeERB
    ).await?;
    
    assert!(compute_response.erb_session.is_some());
    let erb_session = compute_response.erb_session.unwrap();
    assert_eq!(erb_session.erb_type, ERBType::ComputeERB);
    assert!(erb_session.resource_usage.api_calls > 0);
    assert_eq!(erb_session.currency, "USD");
    
    println!("✅ Compute ERB billing session created");
    println!("   Session ID: {}", erb_session.session_id);
    println!("   API Calls: {}", erb_session.resource_usage.api_calls);
    println!("   Billing Rate: ${}/unit", erb_session.billing_rate);
    
    // Test bandwidth ERB billing
    let bandwidth_response = client.get_with_erb(
        "https://amazon.com/large-download",
        ERBType::BandwidthERB
    ).await?;
    
    assert!(bandwidth_response.erb_session.is_some());
    let bandwidth_session = bandwidth_response.erb_session.unwrap();
    assert_eq!(bandwidth_session.erb_type, ERBType::BandwidthERB);
    
    println!("✅ Bandwidth ERB billing session created");
    println!("   Bandwidth Usage: {} bytes", bandwidth_session.resource_usage.bandwidth_bytes);
    
    // Test API ERB billing
    let api_response = client.get_with_erb(
        "https://microsoft.com/api/premium-service",
        ERBType::ApiERB
    ).await?;
    
    assert!(api_response.erb_session.is_some());
    println!("✅ API ERB billing session created");
    
    Ok(())
}

async fn test_jurisdiction_compliance(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("🏛️ Testing jurisdiction compliance...");
    
    // Test US jurisdiction compliance
    let us_response = client.get_cross_domain("https://google.com/us-only-service").await?;
    assert_eq!(us_response.jurisdiction_info.country_code, "US");
    assert!(us_response.jurisdiction_info.cross_border_allowed);
    assert_eq!(us_response.jurisdiction_info.erb_billing_currency, "USD");
    
    println!("✅ US jurisdiction compliance validated");
    println!("   Country Code: {}", us_response.jurisdiction_info.country_code);
    println!("   Cross-border Allowed: {}", us_response.jurisdiction_info.cross_border_allowed);
    println!("   Billing Currency: {}", us_response.jurisdiction_info.erb_billing_currency);
    
    // Test EU jurisdiction (simulated with .eu domain)
    let eu_response = client.get_cross_domain("https://example.eu/gdpr-service").await?;
    assert_eq!(eu_response.jurisdiction_info.country_code, "EU");
    assert!(eu_response.jurisdiction_info.data_sovereignty_rules.contains(&"GDPR-strict".to_string()));
    
    println!("✅ EU jurisdiction compliance validated");
    println!("   GDPR Rules: {:?}", eu_response.jurisdiction_info.data_sovereignty_rules);
    
    Ok(())
}

async fn test_wallet_location_integration(client: &CrossDomainHttpcgClient) -> Result<()> {
    println!("📍 Testing wallet location integration...");
    
    // Test wallet location detection
    let response = client.get_cross_domain("https://google.com/location-aware-service").await?;
    
    // Validate wallet location is properly integrated
    assert_eq!(response.wallet_location, "US");
    assert!(response.cross_domain_metadata.wallet_did.is_some());
    assert!(response.cross_domain_metadata.wallet_did.as_ref().unwrap().contains(":us:"));
    
    println!("✅ Wallet location integration validated");
    println!("   Wallet Location: {}", response.wallet_location);
    println!("   Wallet DID: {}", response.cross_domain_metadata.wallet_did.as_ref().unwrap());
    
    // Test location-based access control
    let metadata = &response.cross_domain_metadata;
    assert_eq!(metadata.security_level, SecurityLevel::Enhanced);
    assert!(metadata.jurisdiction_validated);
    
    println!("✅ Location-based access control validated");
    println!("   Security Level: {:?}", metadata.security_level);
    println!("   Jurisdiction Validated: {}", metadata.jurisdiction_validated);
    
    Ok(())
}

/// Test cross-domain httpcg with different wallet locations
#[tokio::test]
async fn test_multi_location_cross_domain() -> Result<()> {
    println!("🌍 Testing multi-location cross-domain httpcg...");
    
    // Test US wallet
    let us_wallet = create_test_wallet_with_location("US").await?;
    let us_client = CrossDomainHttpcgClient::new(us_wallet).await?;
    let us_response = us_client.get_cross_domain("https://google.com/").await?;
    assert_eq!(us_response.wallet_location, "US");
    
    // Test EU wallet
    let eu_wallet = create_test_wallet_with_location("EU").await?;
    let eu_client = CrossDomainHttpcgClient::new(eu_wallet).await?;
    let eu_response = eu_client.get_cross_domain("https://example.eu/").await?;
    assert_eq!(eu_response.wallet_location, "EU");
    
    // Test CA wallet
    let ca_wallet = create_test_wallet_with_location("CA").await?;
    let ca_client = CrossDomainHttpcgClient::new(ca_wallet).await?;
    let ca_response = ca_client.get_cross_domain("https://example.ca/").await?;
    assert_eq!(ca_response.wallet_location, "CA");
    
    println!("✅ Multi-location cross-domain testing completed");
    println!("   US Response: {} bytes", us_response.httpcg_response.body.len());
    println!("   EU Response: {} bytes", eu_response.httpcg_response.body.len());
    println!("   CA Response: {} bytes", ca_response.httpcg_response.body.len());
    
    Ok(())
}

/// Test ERB billing accuracy and resource tracking
#[tokio::test]
async fn test_erb_billing_accuracy() -> Result<()> {
    println!("📊 Testing ERB billing accuracy...");
    
    let wallet = create_test_wallet_with_location("US").await?;
    let client = CrossDomainHttpcgClient::new(wallet).await?;
    
    // Make multiple requests with ERB tracking
    let mut total_api_calls = 0;
    let mut total_bandwidth = 0;
    
    for i in 0..5 {
        let response = client.get_with_erb(
            &format!("https://google.com/api/test-{}", i),
            ERBType::ComputeERB
        ).await?;
        
        if let Some(session) = response.erb_session {
            total_api_calls += session.resource_usage.api_calls;
            total_bandwidth += session.resource_usage.bandwidth_bytes;
            
            println!("   Request {}: {} API calls, {} bytes", 
                i + 1, 
                session.resource_usage.api_calls,
                session.resource_usage.bandwidth_bytes
            );
        }
    }
    
    assert!(total_api_calls >= 5); // At least one call per request
    assert!(total_bandwidth > 0);  // Some bandwidth usage
    
    println!("✅ ERB billing accuracy validated");
    println!("   Total API Calls: {}", total_api_calls);
    println!("   Total Bandwidth: {} bytes", total_bandwidth);
    
    Ok(())
}

/// Test error handling for unsupported domains
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    println!("⚠️ Testing error handling for cross-domain requests...");
    
    let wallet = create_test_wallet_with_location("US").await?;
    let client = CrossDomainHttpcgClient::new(wallet).await?;
    
    // Test invalid URL
    let invalid_result = client.get_cross_domain("invalid-url").await;
    assert!(invalid_result.is_err());
    println!("✅ Invalid URL error handling works");
    
    // Test unsupported domain (should still work via gateway)
    let unknown_response = client.get_cross_domain("https://unknown-domain.xyz/").await?;
    assert_eq!(unknown_response.httpcg_response.status, 200);
    println!("✅ Unknown domain handled via gateway");
    
    Ok(())
}

/// Performance test for cross-domain httpcg
#[tokio::test]
async fn test_cross_domain_performance() -> Result<()> {
    println!("⚡ Testing cross-domain httpcg performance...");
    
    let wallet = create_test_wallet_with_location("US").await?;
    let client = CrossDomainHttpcgClient::new(wallet).await?;
    
    let start_time = std::time::Instant::now();
    
    // Make 10 concurrent requests
    let mut handles = vec![];
    for i in 0..10 {
        let client_clone = &client;
        let handle = tokio::spawn(async move {
            client_clone.get_cross_domain(&format!("https://google.com/test-{}", i)).await
        });
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    let mut successful_requests = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
            successful_requests += 1;
        }
    }
    
    let elapsed = start_time.elapsed();
    
    assert!(successful_requests >= 8); // At least 80% success rate
    assert!(elapsed.as_secs() < 30);   // Complete within 30 seconds
    
    println!("✅ Cross-domain performance test completed");
    println!("   Successful Requests: {}/10", successful_requests);
    println!("   Total Time: {:?}", elapsed);
    println!("   Average Time per Request: {:?}", elapsed / 10);
    
    Ok(())
}
