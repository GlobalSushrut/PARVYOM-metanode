//! Shared Memory Region for commute.lock
//! 
//! Memory-mapped files for zero-copy inter-component communication

use anyhow::{Result, anyhow};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use memmap2::{MmapMut, MmapOptions};
use std::sync::Arc;
use parking_lot::RwLock;

use super::message::Message;

/// Shared memory region backed by memory-mapped file
#[derive(Clone, Debug)]
pub struct SharedMemoryRegion {
    /// Path to shared memory file
    path: std::path::PathBuf,
    /// Size in bytes
    size: usize,
    /// Memory-mapped region (wrapped in Arc for cloning)
    mmap: Arc<RwLock<MmapMut>>,
}

impl SharedMemoryRegion {
    /// Create new shared memory region
    pub fn create<P: AsRef<Path>>(path: P, size: usize) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Create or open file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        
        // Set file size
        file.set_len(size as u64)?;
        
        // Create memory map
        let mmap = unsafe {
            MmapOptions::new()
                .len(size)
                .map_mut(&file)?
        };
        
        Ok(Self {
            path,
            size,
            mmap: Arc::new(RwLock::new(mmap)),
        })
    }
    
    /// Open existing shared memory region
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)?;
        
        let metadata = file.metadata()?;
        let size = metadata.len() as usize;
        
        let mmap = unsafe {
            MmapOptions::new()
                .len(size)
                .map_mut(&file)?
        };
        
        Ok(Self {
            path,
            size,
            mmap: Arc::new(RwLock::new(mmap)),
        })
    }
    
    /// Write message to shared memory
    pub fn write_message(&self, message: &Message) -> Result<()> {
        let serialized = message.serialize()?;
        
        if serialized.len() > self.size {
            return Err(anyhow!("Message too large for shared memory region: {} > {}", 
                serialized.len(), self.size));
        }
        
        let mut mmap = self.mmap.write();
        
        // Write message size (first 8 bytes)
        let size_bytes = (serialized.len() as u64).to_le_bytes();
        mmap[0..8].copy_from_slice(&size_bytes);
        
        // Write message data
        mmap[8..8 + serialized.len()].copy_from_slice(&serialized);
        
        // Flush to ensure data is written
        mmap.flush()?;
        
        Ok(())
    }
    
    /// Read message from shared memory
    pub fn read_message(&self) -> Result<Message> {
        let mmap = self.mmap.read();
        
        // Read message size (first 8 bytes)
        let mut size_bytes = [0u8; 8];
        size_bytes.copy_from_slice(&mmap[0..8]);
        let size = u64::from_le_bytes(size_bytes) as usize;
        
        if size == 0 || size > self.size - 8 {
            return Err(anyhow!("Invalid message size in shared memory: {}", size));
        }
        
        // Read message data
        let message_data = &mmap[8..8 + size];
        
        // Deserialize message
        Message::deserialize(message_data)
    }
    
    /// Write raw data to shared memory
    pub fn write(&self, offset: usize, data: &[u8]) -> Result<()> {
        if offset + data.len() > self.size {
            return Err(anyhow!("Write would exceed shared memory bounds"));
        }
        
        let mut mmap = self.mmap.write();
        mmap[offset..offset + data.len()].copy_from_slice(data);
        mmap.flush()?;
        
        Ok(())
    }
    
    /// Read raw data from shared memory
    pub fn read(&self, offset: usize, len: usize) -> Result<Vec<u8>> {
        if offset + len > self.size {
            return Err(anyhow!("Read would exceed shared memory bounds"));
        }
        
        let mmap = self.mmap.read();
        Ok(mmap[offset..offset + len].to_vec())
    }
    
    /// Get size of shared memory region
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get path to shared memory file
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// Clear shared memory region
    pub fn clear(&self) -> Result<()> {
        let mut mmap = self.mmap.write();
        for byte in mmap.iter_mut() {
            *byte = 0;
        }
        mmap.flush()?;
        Ok(())
    }
}

// Temporarily disabled - requires tempfile dev dependency
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::tempdir;
//     
//     #[test]
//     fn test_shared_memory_create() {
//         let dir = tempdir().unwrap();
//         let path = dir.path().join("test_shm");
//         
//         let shm = SharedMemoryRegion::create(&path, 1024 * 1024).unwrap();
//         assert_eq!(shm.size(), 1024 * 1024);
//     }
//     
//     #[test]
//     fn test_shared_memory_write_read() {
//         let dir = tempdir().unwrap();
//         let path = dir.path().join("test_shm");
//         
//         let shm = SharedMemoryRegion::create(&path, 1024 * 1024).unwrap();
//         
//         let data = b"Hello, commute.lock!";
//         shm.write(0, data).unwrap();
//         
//         let read_data = shm.read(0, data.len()).unwrap();
//         assert_eq!(data, &read_data[..]);
//     }
// }
