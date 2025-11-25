//! CommuteLock Integration for P2P Mesh
//! 
//! Zero-copy communication using shared memory and file locks.
//! 
//! # Purpose
//! 
//! Enable microsecond-latency messaging between P2P mesh nodes:
//! - Shared memory channels (/dev/shm/bpci/p2p_mesh/)
//! - Wave-based message multiplexing
//! - Lock-based synchronization (flock)
//! - Non-blocking receive operations
//! 
//! # Architecture
//! 
//! ```text
//! /dev/shm/bpci/p2p_mesh/
//! ├── wave_channels/      # Wave-multiplexed channels
//! │   ├── wave_0.lock
//! │   ├── wave_0.data
//! │   └── ...
//! ├── peer_channels/      # Direct peer channels
//! │   ├── peer_<id>.lock
//! │   ├── peer_<id>.data
//! │   └── ...
//! └── control.lock        # Control channel
//! ```

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use super::wave::WaveToken;

/// CommuteLock base directory
const COMMUTELOCK_BASE: &str = "/dev/shm/bpci/p2p_mesh";

/// Message envelope for CommuteLock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommuteLockMessage {
    /// Source node ID
    pub from: String,
    
    /// Destination node ID
    pub to: String,
    
    /// Wave token (for wave-multiplexed channels)
    pub wave: Option<WaveToken>,
    
    /// Message payload
    pub payload: Vec<u8>,
    
    /// Timestamp
    pub timestamp: u64,
}

/// CommuteLock channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    /// Wave-multiplexed channel
    Wave(usize),
    
    /// Direct peer channel
    Peer,
    
    /// Control channel
    Control,
}

/// CommuteLock channel
pub struct CommuteLockChannel {
    /// Channel type
    channel_type: ChannelType,
    
    /// Channel ID
    channel_id: String,
    
    /// Lock file path
    lock_path: PathBuf,
    
    /// Data file path
    data_path: PathBuf,
    
    /// Lock file
    lock_file: Option<File>,
}

impl CommuteLockChannel {
    /// Create a new CommuteLock channel
    pub fn new(channel_type: ChannelType, channel_id: String) -> io::Result<Self> {
        // Ensure base directory exists
        fs::create_dir_all(COMMUTELOCK_BASE)?;
        
        let (lock_path, data_path) = match channel_type {
            ChannelType::Wave(wave_id) => {
                let dir = format!("{}/wave_channels", COMMUTELOCK_BASE);
                fs::create_dir_all(&dir)?;
                (
                    PathBuf::from(format!("{}/wave_{}.lock", dir, wave_id)),
                    PathBuf::from(format!("{}/wave_{}.data", dir, wave_id)),
                )
            }
            ChannelType::Peer => {
                let dir = format!("{}/peer_channels", COMMUTELOCK_BASE);
                fs::create_dir_all(&dir)?;
                (
                    PathBuf::from(format!("{}/peer_{}.lock", dir, channel_id)),
                    PathBuf::from(format!("{}/peer_{}.data", dir, channel_id)),
                )
            }
            ChannelType::Control => {
                (
                    PathBuf::from(format!("{}/control.lock", COMMUTELOCK_BASE)),
                    PathBuf::from(format!("{}/control.data", COMMUTELOCK_BASE)),
                )
            }
        };
        
        Ok(Self {
            channel_type,
            channel_id,
            lock_path,
            data_path,
            lock_file: None,
        })
    }
    
    /// Send a message (blocking with lock)
    pub fn send(&mut self, message: &CommuteLockMessage) -> io::Result<()> {
        // Acquire lock
        self.acquire_lock()?;
        
        // Serialize message
        let data = serde_json::to_vec(message)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // Write to data file
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.data_path)?;
        
        file.write_all(&data)?;
        file.sync_all()?;
        
        // Release lock
        self.release_lock()?;
        
        Ok(())
    }
    
    /// Receive a message (non-blocking)
    pub fn try_recv(&mut self) -> io::Result<Option<CommuteLockMessage>> {
        // Try to acquire lock (non-blocking)
        if !self.try_acquire_lock()? {
            return Ok(None);
        }
        
        // Read from data file
        let mut file = match OpenOptions::new().read(true).open(&self.data_path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                self.release_lock()?;
                return Ok(None);
            }
            Err(e) => {
                self.release_lock()?;
                return Err(e);
            }
        };
        
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        
        if data.is_empty() {
            self.release_lock()?;
            return Ok(None);
        }
        
        // Deserialize message
        let message: CommuteLockMessage = serde_json::from_slice(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // Clear data file
        fs::remove_file(&self.data_path)?;
        
        // Release lock
        self.release_lock()?;
        
        Ok(Some(message))
    }
    
    /// Acquire lock (blocking)
    fn acquire_lock(&mut self) -> io::Result<()> {
        use std::os::unix::fs::OpenOptionsExt;
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o666)
            .open(&self.lock_path)?;
        
        // Use flock for advisory locking
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = file.as_raw_fd();
            
            unsafe {
                if libc::flock(fd, libc::LOCK_EX) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }
        
        self.lock_file = Some(file);
        Ok(())
    }
    
    /// Try to acquire lock (non-blocking)
    fn try_acquire_lock(&mut self) -> io::Result<bool> {
        use std::os::unix::fs::OpenOptionsExt;
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o666)
            .open(&self.lock_path)?;
        
        // Use flock with LOCK_NB for non-blocking
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            let fd = file.as_raw_fd();
            
            unsafe {
                if libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) != 0 {
                    let err = io::Error::last_os_error();
                    if err.kind() == io::ErrorKind::WouldBlock {
                        return Ok(false);
                    }
                    return Err(err);
                }
            }
        }
        
        self.lock_file = Some(file);
        Ok(true)
    }
    
    /// Release lock
    fn release_lock(&mut self) -> io::Result<()> {
        if let Some(file) = self.lock_file.take() {
            #[cfg(unix)]
            {
                use std::os::unix::io::AsRawFd;
                let fd = file.as_raw_fd();
                
                unsafe {
                    if libc::flock(fd, libc::LOCK_UN) != 0 {
                        return Err(io::Error::last_os_error());
                    }
                }
            }
        }
        Ok(())
    }
}

impl Drop for CommuteLockChannel {
    fn drop(&mut self) {
        let _ = self.release_lock();
    }
}

/// CommuteLock manager for P2P mesh
pub struct CommuteLockManager {
    /// Node ID
    node_id: String,
    
    /// Wave channels
    wave_channels: Vec<CommuteLockChannel>,
    
    /// Peer channels (node_id -> channel)
    peer_channels: std::collections::HashMap<String, CommuteLockChannel>,
    
    /// Control channel
    control_channel: CommuteLockChannel,
}

impl CommuteLockManager {
    /// Create a new CommuteLock manager
    pub fn new(node_id: String, num_waves: usize) -> io::Result<Self> {
        // Create wave channels
        let mut wave_channels = Vec::with_capacity(num_waves);
        for i in 0..num_waves {
            let channel = CommuteLockChannel::new(
                ChannelType::Wave(i),
                format!("wave_{}", i),
            )?;
            wave_channels.push(channel);
        }
        
        // Create control channel
        let control_channel = CommuteLockChannel::new(
            ChannelType::Control,
            "control".to_string(),
        )?;
        
        Ok(Self {
            node_id,
            wave_channels,
            peer_channels: std::collections::HashMap::new(),
            control_channel,
        })
    }
    
    /// Send message on wave channel
    pub fn send_wave(&mut self, wave_id: usize, message: CommuteLockMessage) -> io::Result<()> {
        if let Some(channel) = self.wave_channels.get_mut(wave_id) {
            channel.send(&message)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Wave channel {} not found", wave_id),
            ))
        }
    }
    
    /// Send message on peer channel
    pub fn send_peer(&mut self, peer_id: &str, message: CommuteLockMessage) -> io::Result<()> {
        // Get or create peer channel
        if !self.peer_channels.contains_key(peer_id) {
            let channel = CommuteLockChannel::new(
                ChannelType::Peer,
                peer_id.to_string(),
            )?;
            self.peer_channels.insert(peer_id.to_string(), channel);
        }
        
        self.peer_channels
            .get_mut(peer_id)
            .unwrap()
            .send(&message)
    }
    
    /// Receive from all wave channels (non-blocking)
    pub fn recv_waves(&mut self) -> Vec<(usize, CommuteLockMessage)> {
        let mut messages = Vec::new();
        
        for (wave_id, channel) in self.wave_channels.iter_mut().enumerate() {
            if let Ok(Some(msg)) = channel.try_recv() {
                messages.push((wave_id, msg));
            }
        }
        
        messages
    }
    
    /// Receive from all peer channels (non-blocking)
    pub fn recv_peers(&mut self) -> Vec<(String, CommuteLockMessage)> {
        let mut messages = Vec::new();
        
        for (peer_id, channel) in self.peer_channels.iter_mut() {
            if let Ok(Some(msg)) = channel.try_recv() {
                messages.push((peer_id.clone(), msg));
            }
        }
        
        messages
    }
    
    /// Receive from control channel (non-blocking)
    pub fn recv_control(&mut self) -> io::Result<Option<CommuteLockMessage>> {
        self.control_channel.try_recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_creation() {
        let channel = CommuteLockChannel::new(
            ChannelType::Wave(0),
            "test_wave".to_string(),
        );
        assert!(channel.is_ok());
    }
    
    #[test]
    fn test_message_send_recv() {
        let mut channel = CommuteLockChannel::new(
            ChannelType::Control,
            "test_control".to_string(),
        ).unwrap();
        
        let message = CommuteLockMessage {
            from: "node1".to_string(),
            to: "node2".to_string(),
            wave: None,
            payload: vec![1, 2, 3, 4],
            timestamp: 0,
        };
        
        // Send message
        channel.send(&message).unwrap();
        
        // Receive message
        let received = channel.try_recv().unwrap();
        assert!(received.is_some());
        
        let received_msg = received.unwrap();
        assert_eq!(received_msg.from, "node1");
        assert_eq!(received_msg.to, "node2");
        assert_eq!(received_msg.payload, vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_manager_creation() {
        let manager = CommuteLockManager::new("test_node".to_string(), 10);
        assert!(manager.is_ok());
    }
}
