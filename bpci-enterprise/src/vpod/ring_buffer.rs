//! # Lock-Free Single Producer Single Consumer (SPSC) Ring Buffer
//! 
//! High-performance ring buffer for actor message passing with microsecond latency.
//! Uses atomic operations for lock-free communication between actors.

use anyhow::{Result, anyhow};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Ring buffer error types
#[derive(Debug, thiserror::Error)]
pub enum RingBufferError {
    #[error("Ring buffer is full")]
    Full,
    
    #[error("Ring buffer is empty")]
    Empty,
    
    #[error("Invalid capacity: {0}")]
    InvalidCapacity(usize),
    
    #[error("Buffer overflow detected")]
    Overflow,
}

/// Lock-free SPSC ring buffer
/// 
/// This implementation uses atomic operations for head and tail pointers
/// to achieve lock-free operation between a single producer and consumer.
#[derive(Debug)]
pub struct SPSCRingBuffer<T> {
    /// Ring buffer storage
    buffer: Vec<MaybeUninit<T>>,
    
    /// Head pointer (consumer reads from here)
    head: AtomicUsize,
    
    /// Tail pointer (producer writes to here)
    tail: AtomicUsize,
    
    /// Buffer capacity (power of 2 for efficient modulo)
    capacity: usize,
    
    /// Capacity mask for fast modulo operation
    mask: usize,
}

impl<T> SPSCRingBuffer<T> {
    /// Create a new ring buffer with specified capacity
    /// 
    /// Capacity will be rounded up to the next power of 2 for efficiency.
    pub fn new(capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(anyhow!(RingBufferError::InvalidCapacity(capacity)));
        }
        
        // Round up to next power of 2
        let capacity = capacity.next_power_of_two();
        let mask = capacity - 1;
        
        // Initialize buffer with uninitialized memory
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(MaybeUninit::uninit());
        }
        
        Ok(SPSCRingBuffer {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity,
            mask,
        })
    }
    
    /// Push an item to the ring buffer (producer side)
    /// 
    /// Returns the item back if the buffer is full.
    pub fn push(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & self.mask;
        
        // Check if buffer is full
        let head = self.head.load(Ordering::Acquire);
        if next_tail == head {
            return Err(item); // Buffer full
        }
        
        // Safety: We've verified the slot is available
        unsafe {
            let slot = self.buffer.get_unchecked(tail);
            (slot as *const MaybeUninit<T> as *mut MaybeUninit<T>)
                .write(MaybeUninit::new(item));
        }
        
        // Update tail pointer
        self.tail.store(next_tail, Ordering::Release);
        
        Ok(())
    }
    
    /// Pop an item from the ring buffer (consumer side)
    /// 
    /// Returns None if the buffer is empty.
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        
        // Check if buffer is empty
        if head == tail {
            return None;
        }
        
        // Safety: We've verified an item is available
        let item = unsafe {
            let slot = self.buffer.get_unchecked(head);
            slot.as_ptr().read()
        };
        
        // Update head pointer
        let next_head = (head + 1) & self.mask;
        self.head.store(next_head, Ordering::Release);
        
        Some(item)
    }
    
    /// Try to push an item without blocking
    /// 
    /// Returns true if successful, false if buffer is full.
    pub fn try_push(&self, item: T) -> Result<bool, T> {
        match self.push(item) {
            Ok(()) => Ok(true),
            Err(_item) => Ok(false), // Return false but don't consume the item
        }
    }
    
    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        head == tail
    }
    
    /// Check if the buffer is full
    pub fn is_full(&self) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & self.mask;
        let head = self.head.load(Ordering::Relaxed);
        next_tail == head
    }
    
    /// Get the current number of items in the buffer
    pub fn len(&self) -> usize {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Relaxed);
        (tail.wrapping_sub(head)) & self.mask
    }
    
    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get buffer utilization as a percentage (0.0 to 1.0)
    pub fn utilization(&self) -> f64 {
        self.len() as f64 / self.capacity as f64
    }
    
    /// Peek at the next item without removing it
    pub fn peek(&self) -> Option<&T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        
        if head == tail {
            return None; // Buffer empty
        }
        
        // Safety: We've verified an item is available
        unsafe {
            let slot = self.buffer.get_unchecked(head);
            Some(&*slot.as_ptr())
        }
    }
    
    /// Clear all items from the buffer
    /// 
    /// This is not thread-safe and should only be called when no other
    /// threads are accessing the buffer.
    pub unsafe fn clear(&self) {
        while self.pop().is_some() {
            // Drop all items
        }
    }
    
    /// Get buffer statistics for monitoring
    pub fn stats(&self) -> RingBufferStats {
        let len = self.len();
        let capacity = self.capacity();
        let utilization = len as f64 / capacity as f64;
        
        RingBufferStats {
            capacity,
            length: len,
            utilization,
            is_empty: len == 0,
            is_full: len == capacity - 1, // Account for the sentinel slot
        }
    }
}

/// Ring buffer statistics
#[derive(Debug, Clone)]
pub struct RingBufferStats {
    /// Buffer capacity
    pub capacity: usize,
    
    /// Current number of items
    pub length: usize,
    
    /// Utilization percentage (0.0 to 1.0)
    pub utilization: f64,
    
    /// Whether buffer is empty
    pub is_empty: bool,
    
    /// Whether buffer is full
    pub is_full: bool,
}

/// Thread-safe wrapper for SPSC ring buffer
/// 
/// Provides Arc-wrapped access for sharing between threads.
pub type SharedRingBuffer<T> = Arc<SPSCRingBuffer<T>>;

impl<T> SPSCRingBuffer<T> {
    /// Create a shared ring buffer wrapped in Arc
    pub fn new_shared(capacity: usize) -> Result<SharedRingBuffer<T>> {
        Ok(Arc::new(Self::new(capacity)?))
    }
}

// Safety: SPSCRingBuffer is safe to send between threads
unsafe impl<T: Send> Send for SPSCRingBuffer<T> {}

// Safety: SPSCRingBuffer supports concurrent access from single producer/consumer
unsafe impl<T: Send> Sync for SPSCRingBuffer<T> {}

impl<T> Drop for SPSCRingBuffer<T> {
    fn drop(&mut self) {
        // Safely drop all remaining items
        while self.pop().is_some() {
            // Items are automatically dropped
        }
    }
}

/// Batch operations for improved performance
impl<T> SPSCRingBuffer<T> {
    /// Push multiple items in a batch
    /// 
    /// Returns the number of items successfully pushed.
    pub fn push_batch(&self, items: &mut Vec<T>) -> usize {
        let mut pushed = 0;
        
        while !items.is_empty() && !self.is_full() {
            let item = items.remove(0);
            if self.push(item).is_ok() {
                pushed += 1;
            } else {
                break;
            }
        }
        
        pushed
    }
    
    /// Pop multiple items in a batch
    /// 
    /// Returns a vector of popped items.
    pub fn pop_batch(&self, max_items: usize) -> Vec<T> {
        let mut items = Vec::with_capacity(max_items);
        
        for _ in 0..max_items {
            if let Some(item) = self.pop() {
                items.push(item);
            } else {
                break;
            }
        }
        
        items
    }
}

/// Performance monitoring for ring buffers
pub struct RingBufferMonitor<T> {
    buffer: SharedRingBuffer<T>,
    stats_history: Vec<RingBufferStats>,
    max_history: usize,
}

impl<T> RingBufferMonitor<T> {
    /// Create a new monitor for a ring buffer
    pub fn new(buffer: SharedRingBuffer<T>, max_history: usize) -> Self {
        Self {
            buffer,
            stats_history: Vec::with_capacity(max_history),
            max_history,
        }
    }
    
    /// Record current buffer statistics
    pub fn record_stats(&mut self) {
        let stats = self.buffer.stats();
        
        self.stats_history.push(stats);
        
        // Keep only the most recent stats
        if self.stats_history.len() > self.max_history {
            self.stats_history.remove(0);
        }
    }
    
    /// Get average utilization over recorded history
    pub fn average_utilization(&self) -> f64 {
        if self.stats_history.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.stats_history.iter()
            .map(|s| s.utilization)
            .sum();
        
        sum / self.stats_history.len() as f64
    }
    
    /// Get peak utilization
    pub fn peak_utilization(&self) -> f64 {
        self.stats_history.iter()
            .map(|s| s.utilization)
            .fold(0.0, f64::max)
    }
    
    /// Check if buffer has been consistently full
    pub fn is_consistently_full(&self, threshold: f64) -> bool {
        if self.stats_history.len() < 3 {
            return false;
        }
        
        let recent_stats = &self.stats_history[self.stats_history.len() - 3..];
        recent_stats.iter().all(|s| s.utilization > threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_ring_buffer_creation() {
        let buffer = SPSCRingBuffer::<i32>::new(16).unwrap();
        assert_eq!(buffer.capacity(), 16);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
    }

    #[test]
    fn test_push_pop() {
        let buffer = SPSCRingBuffer::new(4).unwrap();
        
        // Push items
        assert!(buffer.push(1).is_ok());
        assert!(buffer.push(2).is_ok());
        assert!(buffer.push(3).is_ok());
        
        // Pop items
        assert_eq!(buffer.pop(), Some(1));
        assert_eq!(buffer.pop(), Some(2));
        assert_eq!(buffer.pop(), Some(3));
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_buffer_full() {
        let buffer = SPSCRingBuffer::new(2).unwrap(); // Capacity 2, but 1 usable slot
        
        assert!(buffer.push(1).is_ok());
        assert!(buffer.push(2).is_err()); // Should be full
        assert!(buffer.is_full());
    }

    #[test]
    fn test_concurrent_access() {
        let buffer = Arc::new(SPSCRingBuffer::new(1000).unwrap());
        let buffer_producer = buffer.clone();
        let buffer_consumer = buffer.clone();
        
        // Producer thread
        let producer = thread::spawn(move || {
            for i in 0..500 {
                while buffer_producer.push(i).is_err() {
                    thread::yield_now();
                }
            }
        });
        
        // Consumer thread
        let consumer = thread::spawn(move || {
            let mut received = Vec::new();
            while received.len() < 500 {
                if let Some(item) = buffer_consumer.pop() {
                    received.push(item);
                } else {
                    thread::yield_now();
                }
            }
            received
        });
        
        producer.join().unwrap();
        let received = consumer.join().unwrap();
        
        assert_eq!(received.len(), 500);
        // Verify all items were received (may not be in order due to concurrency)
        let mut sorted_received = received;
        sorted_received.sort();
        let expected: Vec<i32> = (0..500).collect();
        assert_eq!(sorted_received, expected);
    }

    #[test]
    fn test_batch_operations() {
        let buffer = SPSCRingBuffer::new(10).unwrap();
        
        let mut items = vec![1, 2, 3, 4, 5];
        let pushed = buffer.push_batch(&mut items);
        assert_eq!(pushed, 5);
        assert!(items.is_empty());
        
        let popped = buffer.pop_batch(3);
        assert_eq!(popped, vec![1, 2, 3]);
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn test_buffer_stats() {
        let buffer = SPSCRingBuffer::new(8).unwrap();
        
        buffer.push(1).unwrap();
        buffer.push(2).unwrap();
        
        let stats = buffer.stats();
        assert_eq!(stats.capacity, 8);
        assert_eq!(stats.length, 2);
        assert_eq!(stats.utilization, 0.25);
        assert!(!stats.is_empty);
        assert!(!stats.is_full);
    }

    #[test]
    fn test_peek() {
        let buffer = SPSCRingBuffer::new(4).unwrap();
        
        assert!(buffer.peek().is_none());
        
        buffer.push(42).unwrap();
        assert_eq!(buffer.peek(), Some(&42));
        assert_eq!(buffer.len(), 1); // Peek shouldn't remove item
        
        assert_eq!(buffer.pop(), Some(42));
        assert!(buffer.peek().is_none());
    }

    #[test]
    fn test_monitor() {
        let buffer = SPSCRingBuffer::new_shared(10).unwrap();
        let mut monitor = RingBufferMonitor::new(buffer.clone(), 5);
        
        // Record some stats
        buffer.push(1).unwrap();
        monitor.record_stats();
        
        buffer.push(2).unwrap();
        buffer.push(3).unwrap();
        monitor.record_stats();
        
        assert!(monitor.average_utilization() > 0.0);
        assert!(monitor.peak_utilization() >= monitor.average_utilization());
    }
}
