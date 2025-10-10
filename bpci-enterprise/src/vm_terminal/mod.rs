//! Revolutionary OCI VM Terminal System
//! 
//! The most sophisticated VM terminal ever built - runs inside OCI containers
//! but provides complete OS-level operations through Oracle integration and
//! advanced VM abstraction. Breaks through typical cloud restrictions to
//! enable full system control in any cloud environment.
//!
//! This is the most sophisticated project built after Linux.

pub mod oci_vm_terminal;
pub mod oracle_integration;
pub mod vm_abstraction_engine;
pub mod bpi_core_bridge;
pub mod bpci_registry_integration;
pub mod container_escape_engine;
pub mod terminal_interface;

pub use oci_vm_terminal::*;
pub use oracle_integration::*;
pub use vm_abstraction_engine::*;
pub use bpi_core_bridge::*;
pub use bpci_registry_integration::*;
pub use container_escape_engine::*;
pub use terminal_interface::*;
