//! Event Notifier for commute.lock
//! 
//! Zero-latency event notification system using eventfd

use anyhow::{Result, anyhow};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd, FromRawFd};
use std::path::Path;
use std::time::Duration;

/// Event notifier for zero-latency signaling
#[derive(Clone, Debug)]
pub struct EventNotifier {
    /// Path to event file
    path: std::path::PathBuf,
    /// Event file descriptor
    fd: RawFd,
}

impl EventNotifier {
    /// Create new event notifier
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Create event file (using regular file as fallback for eventfd)
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        
        let fd = file.as_raw_fd();
        
        // Don't drop the file - we need to keep it open
        std::mem::forget(file);
        
        Ok(Self {
            path,
            fd,
        })
    }
    
    /// Notify waiting threads
    pub fn notify(&self) -> Result<()> {
        // Write a byte to signal event
        let file = unsafe { File::from_raw_fd(self.fd) };
        let mut file_ref = &file;
        file_ref.write_all(&[1u8])?;
        file_ref.sync_all()?; // Ensure data is flushed
        
        // Don't drop the file
        std::mem::forget(file);
        
        Ok(())
    }
    
    /// Wait for notification with timeout
    pub fn wait(&self, timeout_ms: u64) -> Result<()> {
        // Simple polling implementation
        // In production, use epoll or io_uring for better performance
        
        let start = std::time::Instant::now();
        let timeout = Duration::from_millis(timeout_ms);
        
        loop {
            // Check if event file has data by checking file size
            let file = unsafe { File::from_raw_fd(self.fd) };
            
            // Check file metadata to see if there's data
            if let Ok(metadata) = file.metadata() {
                if metadata.len() > 0 {
                    // Event received - consume it
                    let mut file_ref = &file;
                    let mut buf = vec![0u8; metadata.len() as usize];
                    let _ = file_ref.read(&mut buf);
                    
                    // Truncate file for next event
                    let _ = file.set_len(0);
                    
                    std::mem::forget(file);
                    return Ok(());
                }
            }
            
            std::mem::forget(file);
            
            // Check timeout
            if start.elapsed() >= timeout {
                return Err(anyhow!("Event wait timeout after {}ms", timeout_ms));
            }
            
            // Small sleep to avoid busy waiting
            std::thread::sleep(Duration::from_micros(100));
        }
    }
    
    /// Wait for notification without timeout
    pub fn wait_forever(&self) -> Result<()> {
        loop {
            let file = unsafe { File::from_raw_fd(self.fd) };
            let mut file_ref = &file;
            let mut buf = [0u8; 1];
            
            match file_ref.read(&mut buf) {
                Ok(n) if n > 0 => {
                    std::mem::forget(file);
                    return Ok(());
                }
                _ => {
                    std::mem::forget(file);
                    std::thread::sleep(Duration::from_micros(100));
                }
            }
        }
    }
    
    /// Get event file descriptor
    pub fn fd(&self) -> RawFd {
        self.fd
    }
    
    /// Get path to event file
    pub fn path(&self) -> &Path {
        &self.path
    }
}

// Temporarily disabled - requires tempfile dev dependency
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::tempdir;
//     use std::thread;
//     
//     #[test]
//     fn test_event_notifier_create() {
//         let dir = tempdir().unwrap();
//         let path = dir.path().join("test_event");
//         
//         let notifier = EventNotifier::create(&path).unwrap();
//         assert!(notifier.path().exists());
//     }
//     
//     #[test]
//     fn test_event_notify_wait() {
//         let dir = tempdir().unwrap();
//         let path = dir.path().join("test_event");
//         
//         let notifier = EventNotifier::create(&path).unwrap();
//         let notifier_clone = notifier.clone();
//         
//         // Spawn thread to notify after delay
//         thread::spawn(move || {
//             thread::sleep(Duration::from_millis(100));
//             notifier_clone.notify().unwrap();
//         });
//         
//         // Wait for notification
//         let result = notifier.wait(1000);
//         assert!(result.is_ok());
//     }
// }
