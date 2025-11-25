use crate::blockchain_os_kernel::commute_link::{CommuteLink, MessageHandler};
use crate::blockchain_os_kernel::commute_lock::{
    MessageType, Priority, ZeroCopyMessage, MessageMetadata, CompressionType, RoutingInfo, MemoryBlock,
};
use crate::vpods_control_handler::VpodsControlHandler;
use crate::vpods_daemon::VpodsDaemon;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::path::Path;
use anyhow::{Result, anyhow};
use tracing::{info, warn, error, debug};
use tokio::sync::Mutex;

/// Unix socket transport for vPods daemon communication
pub struct VpodsUnixTransport {
    /// Socket path
    socket_path: String,
    /// Unix listener
    listener: Option<Arc<Mutex<UnixListener>>>,
    /// vPods control handler
    control_handler: Arc<VpodsControlHandler>,
}

impl VpodsUnixTransport {
    /// Create new Unix transport
    pub fn new(socket_path: String, control_handler: Arc<VpodsControlHandler>) -> Self {
        Self {
            socket_path,
            listener: None,
            control_handler,
        }
    }

    /// Start Unix socket server
    pub async fn start(&mut self) -> Result<()> {
        // Remove existing socket file if it exists
        if Path::new(&self.socket_path).exists() {
            std::fs::remove_file(&self.socket_path)
                .map_err(|e| anyhow!("Failed to remove existing socket: {}", e))?;
        }

        // Create parent directory if it doesn't exist
        if let Some(parent) = Path::new(&self.socket_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create socket directory: {}", e))?;
        }

        // Bind Unix socket
        let listener = UnixListener::bind(&self.socket_path)
            .map_err(|e| anyhow!("Failed to bind Unix socket {}: {}", self.socket_path, e))?;

        info!("vPods Unix transport listening on: {}", self.socket_path);
        self.listener = Some(Arc::new(Mutex::new(listener)));

        // Start accepting connections
        self.accept_connections().await
    }

    /// Accept incoming connections
    async fn accept_connections(&self) -> Result<()> {
        let listener = self.listener.as_ref()
            .ok_or_else(|| anyhow!("Unix listener not initialized"))?;

        loop {
            let listener_guard = listener.lock().await;
            match listener_guard.accept().await {
                Ok((stream, _addr)) => {
                    let control_handler = self.control_handler.clone();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, control_handler).await {
                            error!("Error handling Unix socket connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept Unix socket connection: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }
    }

    /// Handle individual connection
    async fn handle_connection(
        mut stream: UnixStream,
        control_handler: Arc<VpodsControlHandler>,
    ) -> Result<()> {
        debug!("New Unix socket connection established");

        loop {
            // Read message length (4 bytes big-endian)
            let mut length_buf = [0u8; 4];
            match stream.read_exact(&mut length_buf).await {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    debug!("Unix socket connection closed by client");
                    break;
                }
                Err(e) => {
                    error!("Failed to read message length: {}", e);
                    break;
                }
            }

            let message_length = u32::from_be_bytes(length_buf) as usize;
            if message_length == 0 || message_length > 1024 * 1024 {
                error!("Invalid message length: {}", message_length);
                break;
            }

            // Read message content
            let mut message_buf = vec![0u8; message_length];
            if let Err(e) = stream.read_exact(&mut message_buf).await {
                error!("Failed to read message content: {}", e);
                break;
            }

            // Create zero-copy message for handler
            let zero_copy_msg = Self::create_zero_copy_message(&message_buf)?;

            // Process message through control handler (synchronous trait method)
            match control_handler.handle_message(&zero_copy_msg) {
                Ok(Some(response_bytes)) => {
                    // Send response length
                    let response_length = response_bytes.len() as u32;
                    if let Err(e) = stream.write_all(&response_length.to_be_bytes()).await {
                        error!("Failed to write response length: {}", e);
                        break;
                    }

                    // Send response content
                    if let Err(e) = stream.write_all(&response_bytes).await {
                        error!("Failed to write response content: {}", e);
                        break;
                    }

                    debug!("Sent response: {} bytes", response_bytes.len());
                }
                Ok(None) => {
                    // No response - send empty response
                    if let Err(e) = stream.write_all(&0u32.to_be_bytes()).await {
                        error!("Failed to write empty response: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Message processing failed: {}", e);
                    // Send error response
                    let error_response = format!(r#"{{"error": "Internal server error: {}"}}"#, e);
                    let error_bytes = error_response.as_bytes();
                    let error_length = error_bytes.len() as u32;
                    
                    if stream.write_all(&error_length.to_be_bytes()).await.is_ok() {
                        let _ = stream.write_all(error_bytes).await;
                    }
                    break;
                }
            }
        }

        debug!("Unix socket connection closed");
        Ok(())
    }

    /// Create zero-copy message from buffer
    fn create_zero_copy_message(buffer: &[u8]) -> Result<ZeroCopyMessage> {
        use std::alloc::Layout;
        use std::ptr::NonNull;
        use std::sync::atomic::AtomicU64;
        use crate::blockchain_os_kernel::commute_lock::QuantumBlockState;
        use chrono::Utc;
        use uuid::Uuid;

        let size = buffer.len();
        let layout = Layout::from_size_align(size.max(1), 64)?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(anyhow!("Failed to allocate memory for zero-copy message"));
        }

        let memory_block = Arc::new(MemoryBlock {
            ptr: NonNull::new(ptr).unwrap(),
            size,
            ref_count: Arc::new(AtomicU64::new(1)),
            block_id: Uuid::new_v4(),
            quantum_state: Arc::new(std::sync::RwLock::new(QuantumBlockState {
                entanglement_id: None,
                fidelity: 0.0,
                last_sync: Utc::now(),
                coherent: false,
            })),
        });

        // Copy data into the memory block
        unsafe {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), memory_block.ptr.as_ptr(), size);
        }

        let metadata = MessageMetadata {
            message_type: MessageType::Control,
            content_length: size,
            priority: Priority::High,
            ttl: std::time::Duration::from_secs(30),
            created_at: Utc::now(),
            compression: Some(CompressionType::None),
        };

        let routing_info = RoutingInfo {
            source_address: vec![],
            target_address: vec![],
            routing_path: vec![],
            hop_count: 0,
            weight: 1.0,
        };

        Ok(ZeroCopyMessage {
            message_id: Uuid::new_v4(),
            memory_block,
            metadata,
            quantum_signature: Vec::new(),
            routing_info,
        })
    }

    /// Stop the Unix transport
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(_listener) = self.listener.take() {
            info!("Stopping vPods Unix transport");
            
            // Remove socket file
            if Path::new(&self.socket_path).exists() {
                std::fs::remove_file(&self.socket_path)
                    .map_err(|e| anyhow!("Failed to remove socket file: {}", e))?;
            }
        }
        Ok(())
    }
}

/// Unix socket client for connecting to vPods daemon
pub struct VpodsUnixClient {
    socket_path: String,
}

impl VpodsUnixClient {
    /// Create new Unix client
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    /// Send message and receive response
    pub async fn send_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        // Connect to Unix socket
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| anyhow!("Failed to connect to vPods daemon at {}: {}", self.socket_path, e))?;

        // Send message length
        let message_length = message.len() as u32;
        stream.write_all(&message_length.to_be_bytes()).await
            .map_err(|e| anyhow!("Failed to write message length: {}", e))?;

        // Send message content
        stream.write_all(message).await
            .map_err(|e| anyhow!("Failed to write message content: {}", e))?;

        // Read response length
        let mut length_buf = [0u8; 4];
        stream.read_exact(&mut length_buf).await
            .map_err(|e| anyhow!("Failed to read response length: {}", e))?;

        let response_length = u32::from_be_bytes(length_buf) as usize;
        if response_length == 0 {
            return Ok(Vec::new()); // Empty response
        }

        if response_length > 1024 * 1024 {
            return Err(anyhow!("Response too large: {} bytes", response_length));
        }

        // Read response content
        let mut response_buf = vec![0u8; response_length];
        stream.read_exact(&mut response_buf).await
            .map_err(|e| anyhow!("Failed to read response content: {}", e))?;

        Ok(response_buf)
    }
}

/// Integration with CommuteLink for vPods daemon
impl CommuteLink {
    /// Start vPods daemon with Unix socket transport
    pub async fn start_vpods_daemon(
        &self,
        vpods_daemon: Arc<VpodsDaemon>,
        socket_path: String,
    ) -> Result<()> {
        let node_id = "local-node".to_string(); // Could be from config
        
        // Register a dedicated control handler with CommuteLink
        {
            let handler_box: Box<dyn MessageHandler + Send + Sync> = Box::new(VpodsControlHandler::new(
                vpods_daemon.clone(),
                self.commute_lock.clone(),
                node_id.clone(),
            ));
            let mut handlers = self.message_handlers.write().await;
            handlers.insert(MessageType::Control, handler_box);
        }

        // Create and start Unix transport with its own handler instance
        let control_handler = Arc::new(VpodsControlHandler::new(
            vpods_daemon,
            self.commute_lock.clone(),
            node_id,
        ));

        let mut unix_transport = VpodsUnixTransport::new(socket_path, control_handler);
        unix_transport.start().await?;

        info!("vPods daemon started with Unix socket transport");
        Ok(())
    }

    /// Connect to external vPods daemon via Unix socket
    pub async fn connect_to_vpods_daemon(&self, socket_path: String) -> Result<VpodsUnixClient> {
        let client = VpodsUnixClient::new(socket_path);
        
        // Test connection with hello message
        let hello_msg = r#"{"version":"0.1","id":"test","type":"request","method":"node.hello","payload":{}}"#;
        let response = client.send_message(hello_msg.as_bytes()).await?;
        
        debug!("vPods daemon hello response: {} bytes", response.len());
        Ok(client)
    }
}
