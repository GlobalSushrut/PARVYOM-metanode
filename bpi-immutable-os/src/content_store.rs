//! ERA-FS Content-Addressed Store
//!
//! Minimal implementation to store immutable objects under `/era/store/objects`
//! using cryptographic hashes as addresses.

use anyhow::{anyhow, Result};
use blake3::Hasher;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Supported content address algorithms (start with Blake3, can extend later)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Blake3,
}

/// Cryptographic content address for an immutable object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentAddress {
    pub algo: HashAlgorithm,
    /// Raw hash bytes
    pub hash: [u8; 32],
}

impl ContentAddress {
    /// Directory name component, e.g. `blake3-<hex>`
    pub fn to_dir_name(&self) -> String {
        let hex = hex::encode(self.hash);
        match self.algo {
            HashAlgorithm::Blake3 => format!("blake3-{}", hex),
        }
    }
}

/// Simple content-addressed store rooted at `/era/store/objects` by default
#[derive(Debug, Clone)]
pub struct ContentStore {
    root: PathBuf,
}

impl ContentStore {
    /// Create a store rooted at the default ERA-FS objects directory.
    /// This does not create directories; callers should ensure `/era` layout exists.
    pub fn new_default() -> Self {
        Self {
            root: PathBuf::from("/era/store/objects"),
        }
    }

    /// Create a store rooted at a custom path (mostly for testing).
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Store an immutable object and return its content address.
    ///
    /// Layout:
    /// `/era/store/objects/blake3-<hex-hash>/data`
    pub fn write_object(&self, bytes: &[u8]) -> Result<ContentAddress> {
        // Compute Blake3 hash
        let mut hasher = Hasher::new();
        hasher.update(bytes);
        let hash = hasher.finalize();

        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(hash.as_bytes());

        let addr = ContentAddress {
            algo: HashAlgorithm::Blake3,
            hash: hash_bytes,
        };

        let dir = self.root.join(addr.to_dir_name());
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .map_err(|e| anyhow!("failed to create content dir {}: {}", dir.display(), e))?;
        }

        let data_path = dir.join("data");
        let mut file = fs::File::create(&data_path)
            .map_err(|e| anyhow!("failed to create content file {}: {}", data_path.display(), e))?;
        file.write_all(bytes)
            .map_err(|e| anyhow!("failed to write content to {}: {}", data_path.display(), e))?;

        Ok(addr)
    }

    /// Read an immutable object by its address.
    pub fn read_object(&self, addr: &ContentAddress) -> Result<Vec<u8>> {
        let dir = self.root.join(addr.to_dir_name());
        let data_path = dir.join("data");
        if !data_path.exists() {
            return Err(anyhow!("content object not found at {}", data_path.display()));
        }
        let bytes = fs::read(&data_path)
            .map_err(|e| anyhow!("failed to read content from {}: {}", data_path.display(), e))?;
        Ok(bytes)
    }
}
