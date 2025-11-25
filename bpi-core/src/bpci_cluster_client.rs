use anyhow::Result;
use serde_json::{json, Value};
use std::env;

/// Minimal BPCI Cluster Ledger client for control-plane operations from BPI OS.
///
/// All URLs are env/config-driven; no IPs or domains are hardcoded here.
pub struct BpciClusterClient {
    base_url: String,
    http: reqwest::Client,
}

impl BpciClusterClient {
    /// Construct from environment. Requires BPCI_CLUSTER_LEDGER_BASE_URL.
    pub fn from_env() -> Result<Self> {
        let base_url = env::var("BPCI_CLUSTER_LEDGER_BASE_URL")
            .map_err(|_| anyhow::anyhow!("BPCI_CLUSTER_LEDGER_BASE_URL is not set"))?;
        Ok(Self {
            base_url,
            http: reqwest::Client::new(),
        })
    }

    /// Register a BPI wallet with the BPCI Cluster Ledger.
    ///
    /// This calls /api/v1/bpi/wallets/register on the cluster ledger server.
    pub async fn register_wallet(
        &self,
        wallet_address: &str,
        auth_token: Option<String>,
        capabilities: Vec<String>,
        client_info: Value,
    ) -> Result<Value> {
        let url = format!(
            "{}/api/v1/bpi/wallets/register",
            self.base_url.trim_end_matches('/')
        );

        let body = json!({
            "wallet_address": wallet_address,
            "auth_token": auth_token.unwrap_or_default(),
            "client_info": client_info,
            "capabilities": capabilities,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let value = resp.json::<Value>().await?;

        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "BPCI cluster wallet registration failed: {} - {}",
                status,
                value
            ));
        }

        Ok(value)
    }
}
