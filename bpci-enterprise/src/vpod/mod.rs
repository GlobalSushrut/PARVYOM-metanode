pub mod vpod_node;
pub mod actor;
pub mod actor_types;
pub mod runtime;
pub mod scheduler;

// Re-export commonly used types
pub use vpod_node::{VPodNode, NodeSpecialization, ResourcePool, AllocationStrategy};
pub use actor::{VPodActor, ActorState, Message, MessagePayload, ControlMessage, ActorBudget, ActorMetrics, ActorStatus, ActorSpecialization, ResourceLimits};
pub use actor_types::{AppHostActor, ContainerRuntime, AppStatus, AppMetrics, ConsensusValidatorActor, ValidatorKey, ConsensusState, ValidatorInfo, ValidationMetrics};
pub use runtime::{VPodRuntime, VPodConfig, RuntimeMetrics, RuntimeStatus};
pub use scheduler::{VPodScheduler, SchedulerMetrics, VirtualNodeLane};

// Type aliases for commonly used types
pub type ActorId = uuid::Uuid;

// Ring buffer - check if it exists or create a simple placeholder
pub mod ring_buffer {
    use std::collections::VecDeque;
    
    #[derive(Debug)]
pub struct SPSCRingBuffer<T> {
        buffer: VecDeque<T>,
        capacity: usize,
    }
    
    impl<T> SPSCRingBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            Self {
                buffer: VecDeque::with_capacity(capacity),
                capacity,
            }
        }
        
        pub fn push(&mut self, item: T) -> Result<(), T> {
            if self.buffer.len() >= self.capacity {
                Err(item)
            } else {
                self.buffer.push_back(item);
                Ok(())
            }
        }
        
        pub fn pop(&mut self) -> Option<T> {
            self.buffer.pop_front()
        }
        
        pub fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }
        
        pub fn len(&self) -> usize {
            self.buffer.len()
        }
    }
}

pub use ring_buffer::SPSCRingBuffer;
