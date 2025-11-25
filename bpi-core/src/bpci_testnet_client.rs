use anyhow::Result;
use serde_json::Value;
use std::env;

/// Simple BPCI testnet client for talking to the Auction DB Maintainer
/// to store and fetch mock "mainnet-style" auction results for a BPI node.
///
/// This client is intentionally minimal and config-driven:
/// - Base URL comes from BPCI_AUCTION_DB_BASE_URL (e.g. https://auction-db.testnet.pravyom.global)
/// - No IPs or domains are hardcoded here.
pub struct BpciTestnetClient {
    base_url: String,
}

impl BpciTestnetClient {
    /// Construct from environment. If unset, returns an error so callers can decide
    /// how to behave (e.g. skip testnet reporting).
    pub fn from_env() -> Result<Self> {
        let base_url = env::var("BPCI_AUCTION_DB_BASE_URL")
            .map_err(|_| anyhow::anyhow!("BPCI_AUCTION_DB_BASE_URL is not set"))?;
        Ok(Self { base_url })
    }

    /// Store a mock "mainnet-style" result for this BPI node during testnet.
    ///
    /// `payload` should already contain fields like bpi_node_id, auction_id, etc.
    pub async fn store_mock_mainnet_result(&self, payload: Value) -> Result<Value> {
        let url = format!("{}/testnet/results/store", self.base_url.trim_end_matches('/'));
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        let body = resp.json::<Value>().await?;
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "BPCI testnet store call failed: {} - {}",
                status,
                body
            ));
        }
        Ok(body)
    }

    /// Fetch all mock results for a given BPI node ID.
    pub async fn get_mock_results_for_bpi(&self, bpi_node_id: &str) -> Result<Value> {
        let url = format!(
            "{}/testnet/results/{}",
            self.base_url.trim_end_matches('/'),
            bpi_node_id
        );
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;

        let status = resp.status();
        let body = resp.json::<Value>().await?;
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "BPCI testnet fetch call failed: {} - {}",
                status,
                body
            ));
        }
        Ok(body)
    }
}
