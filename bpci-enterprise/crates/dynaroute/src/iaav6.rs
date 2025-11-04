//! # Identity Anycast Addressing (IAAv6)
//! 
//! Deterministic IPv6 address computation for identity-based anycast routing.
//! No ports, no service IPs - only cryptographic identity.

use std::net::Ipv6Addr;
use blake3;
use serde::{Serialize, Deserialize};

/// IAAv6 Address - Identity-based anycast IPv6 address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IAAv6Address(Ipv6Addr);

impl IAAv6Address {
    /// Create new IAAv6 address from IPv6
    pub fn new(addr: Ipv6Addr) -> Self {
        Self(addr)
    }
    
    /// Get inner IPv6 address
    pub fn inner(&self) -> Ipv6Addr {
        self.0
    }
    
    /// Convert to string
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Ipv6Addr> for IAAv6Address {
    fn from(addr: Ipv6Addr) -> Self {
        Self(addr)
    }
}

impl From<IAAv6Address> for Ipv6Addr {
    fn from(addr: IAAv6Address) -> Self {
        addr.0
    }
}

/// Compute deterministic IAAv6 address
/// 
/// Formula: IAAv6 = base_prefix ⊕ blake3(holder||service||epoch||realm)
/// 
/// # Arguments
/// 
/// * `holder_addr` - Holder address (e.g., "consensus.bpci.local")
/// * `service_id` - Service identifier (e.g., "cluster-ledger")
/// * `epoch` - Current epoch timestamp
/// * `realm` - Realm (e.g., "production", "staging")
/// * `base_prefix` - Base IPv6 prefix (e.g., "2001:db8:03ba::")
/// 
/// # Example
/// 
/// ```
/// use dynaroute::compute_iaav6;
/// use std::net::Ipv6Addr;
/// 
/// let base = Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0);
/// let iaav6 = compute_iaav6(
///     "consensus.bpci.local",
///     "cluster-ledger",
///     1730000000,
///     "production",
///     base,
/// );
/// println!("IAAv6: {}", iaav6.to_string());
/// ```
pub fn compute_iaav6(
    holder_addr: &str,
    service_id: &str,
    epoch: u64,
    realm: &str,
    base_prefix: Ipv6Addr,
) -> IAAv6Address {
    // Construct hash input: holder||service||epoch||realm
    let hash_input = format!("{}||{}||{}||{}", holder_addr, service_id, epoch, realm);
    
    // Compute Blake3 hash (32 bytes)
    let hash = blake3::hash(hash_input.as_bytes());
    let hash_bytes = hash.as_bytes();
    
    // XOR base prefix with hash to get IAAv6
    let mut addr_bytes = base_prefix.octets();
    for i in 0..16 {
        addr_bytes[i] ^= hash_bytes[i];
    }
    
    IAAv6Address::new(Ipv6Addr::from(addr_bytes))
}

/// Compute IAAv6 with cohort support (for blue/green deployments)
/// 
/// Canary cohort gets different epoch slice to generate different IAAv6
pub fn compute_iaav6_with_cohort(
    holder_addr: &str,
    service_id: &str,
    epoch: u64,
    realm: &str,
    cohort: &str,
    base_prefix: Ipv6Addr,
) -> IAAv6Address {
    // Canary cohort gets offset epoch
    let effective_epoch = if cohort == "canary" {
        epoch.wrapping_add(1_000_000)  // Offset for canary
    } else {
        epoch
    };
    
    compute_iaav6(holder_addr, service_id, effective_epoch, realm, base_prefix)
}

/// Parse base prefix from string (e.g., "2001:db8:03ba::/64")
pub fn parse_base_prefix(prefix_str: &str) -> anyhow::Result<Ipv6Addr> {
    // Remove CIDR notation if present
    let addr_str = prefix_str.split('/').next().unwrap_or(prefix_str);
    
    // Parse IPv6 address
    addr_str.parse::<Ipv6Addr>()
        .map_err(|e| anyhow::anyhow!("Failed to parse base prefix: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_iaav6() {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0);
        let iaav6 = compute_iaav6(
            "consensus.bpci.local",
            "cluster-ledger",
            1730000000,
            "production",
            base,
        );
        
        // Should be deterministic
        let iaav6_2 = compute_iaav6(
            "consensus.bpci.local",
            "cluster-ledger",
            1730000000,
            "production",
            base,
        );
        
        assert_eq!(iaav6, iaav6_2);
    }
    
    #[test]
    fn test_different_inputs_different_addresses() {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0);
        
        let iaav6_1 = compute_iaav6("consensus.bpci.local", "service1", 1000, "prod", base);
        let iaav6_2 = compute_iaav6("consensus.bpci.local", "service2", 1000, "prod", base);
        let iaav6_3 = compute_iaav6("blockchain.bpci.local", "service1", 1000, "prod", base);
        
        assert_ne!(iaav6_1, iaav6_2);
        assert_ne!(iaav6_1, iaav6_3);
        assert_ne!(iaav6_2, iaav6_3);
    }
    
    #[test]
    fn test_cohort_canary_different() {
        let base = Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0);
        
        let prod = compute_iaav6_with_cohort(
            "consensus.bpci.local",
            "service1",
            1000,
            "prod",
            "prod",
            base,
        );
        
        let canary = compute_iaav6_with_cohort(
            "consensus.bpci.local",
            "service1",
            1000,
            "prod",
            "canary",
            base,
        );
        
        assert_ne!(prod, canary);
    }
    
    #[test]
    fn test_parse_base_prefix() {
        let prefix = parse_base_prefix("2001:db8:03ba::/64").unwrap();
        assert_eq!(prefix, Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0));
        
        let prefix2 = parse_base_prefix("2001:db8:03ba::").unwrap();
        assert_eq!(prefix2, Ipv6Addr::new(0x2001, 0x0db8, 0x03ba, 0, 0, 0, 0, 0));
    }
}
