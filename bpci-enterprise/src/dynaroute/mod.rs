//! Dynamic Route Management System
//! 
//! This module provides dynamic routing capabilities for the BPCI Enterprise system,
//! enabling real-time route discovery, load balancing, and failover mechanisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub id: String,
    pub source: String,
    pub destination: String,
    pub priority: u8,
    pub active: bool,
}

#[derive(Debug)]
pub struct DynamicRouter {
    routes: Arc<RwLock<HashMap<String, RouteConfig>>>,
}

// Unified Networking Layer for wallet address orchestration
#[derive(Debug, Clone)]
pub struct UnifiedNetworkingLayer {
    pub router: Arc<RwLock<HashMap<String, RouteConfig>>>,
}

impl DynamicRouter {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_route(&self, route: RouteConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut routes = self.routes.write().await;
        routes.insert(route.id.clone(), route);
        Ok(())
    }

    pub async fn get_route(&self, id: &str) -> Option<RouteConfig> {
        let routes = self.routes.read().await;
        routes.get(id).cloned()
    }

    pub async fn remove_route(&self, id: &str) -> Option<RouteConfig> {
        let mut routes = self.routes.write().await;
        routes.remove(id)
    }

    pub async fn list_active_routes(&self) -> Vec<RouteConfig> {
        let routes = self.routes.read().await;
        routes.values()
            .filter(|route| route.active)
            .cloned()
            .collect()
    }
}

impl Default for DynamicRouter {
    fn default() -> Self {
        Self::new()
    }
}
