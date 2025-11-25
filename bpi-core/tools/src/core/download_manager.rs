// Download Manager - Handles BPI component downloads with resume capability
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Client;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub name: String,
    pub url: String,
    pub destination: PathBuf,
    pub expected_size: u64,
    pub expected_checksum: String,
    pub priority: DownloadPriority,
    pub status: DownloadStatus,
    pub progress: DownloadProgress,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DownloadPriority {
    Critical,    // Core BPI components
    High,        // Essential services
    Medium,      // Optional features
    Low,         // Documentation, examples
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed(String),
    Verifying,
    Verified,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub speed_bps: u64,
    pub eta_seconds: u64,
    pub percentage: f64,
}

pub struct DownloadManager {
    client: Client,
    active_downloads: HashMap<String, DownloadTask>,
    download_queue: Vec<DownloadTask>,
    max_concurrent_downloads: usize,
    base_urls: Vec<String>,
}

impl DownloadManager {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("BPI-Advanced-Downloader/1.0.0")
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        let base_urls = vec![
            "https://releases.bpi.dev".to_string(),
            "https://github.com/GlobalSushrut/PARVYOM-metanode/releases/download".to_string(),
            "https://cdn.bpi.network".to_string(),
        ];

        Self {
            client,
            active_downloads: HashMap::new(),
            download_queue: Vec::new(),
            max_concurrent_downloads: 4,
            base_urls,
        }
    }

    /// Create download tasks for all BPI components
    pub fn create_bpi_download_tasks(&mut self, installation_path: &PathBuf, system_arch: &str) -> Result<()> {
        let tasks = vec![
            // Core BPI components (78% production-ready)
            DownloadTask {
                id: "bpi-core".to_string(),
                name: "BPI Core Node".to_string(),
                url: format!("{}/v1.0.0/bpi-core-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/bpi-core.tar.gz"),
                expected_size: 50_000_000, // 50MB
                expected_checksum: "sha256:abc123...".to_string(),
                priority: DownloadPriority::Critical,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },
            
            DownloadTask {
                id: "bpci-enterprise".to_string(),
                name: "BPCI Enterprise".to_string(),
                url: format!("{}/v1.0.0/bpci-enterprise-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/bpci-enterprise.tar.gz"),
                expected_size: 75_000_000, // 75MB
                expected_checksum: "sha256:def456...".to_string(),
                priority: DownloadPriority::Critical,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "immutable-os-kernel".to_string(),
                name: "BPI Immutable OS Kernel".to_string(),
                url: format!("{}/v1.0.0/bpi-immutable-kernel-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/bpi-immutable-kernel.tar.gz"),
                expected_size: 100_000_000, // 100MB
                expected_checksum: "sha256:ghi789...".to_string(),
                priority: DownloadPriority::Critical,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "quantum-consensus".to_string(),
                name: "Quantum Consensus Engine".to_string(),
                url: format!("{}/v1.0.0/quantum-consensus-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/quantum-consensus.tar.gz"),
                expected_size: 30_000_000, // 30MB
                expected_checksum: "sha256:jkl012...".to_string(),
                priority: DownloadPriority::High,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "neural-network".to_string(),
                name: "Neural Network Engine".to_string(),
                url: format!("{}/v1.0.0/neural-network-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/neural-network.tar.gz"),
                expected_size: 40_000_000, // 40MB
                expected_checksum: "sha256:mno345...".to_string(),
                priority: DownloadPriority::High,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "enc-cluster".to_string(),
                name: "ENC Cluster System".to_string(),
                url: format!("{}/v1.0.0/enc-cluster-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/enc-cluster.tar.gz"),
                expected_size: 35_000_000, // 35MB
                expected_checksum: "sha256:pqr678...".to_string(),
                priority: DownloadPriority::High,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "docklock-platform".to_string(),
                name: "DockLock Container Platform".to_string(),
                url: format!("{}/v1.0.0/docklock-platform-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/docklock-platform.tar.gz"),
                expected_size: 60_000_000, // 60MB
                expected_checksum: "sha256:stu901...".to_string(),
                priority: DownloadPriority::High,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "banking-integration".to_string(),
                name: "Banking Integration APIs".to_string(),
                url: format!("{}/v1.0.0/banking-integration-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/banking-integration.tar.gz"),
                expected_size: 25_000_000, // 25MB
                expected_checksum: "sha256:vwx234...".to_string(),
                priority: DownloadPriority::Medium,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "government-integration".to_string(),
                name: "Government Integration APIs".to_string(),
                url: format!("{}/v1.0.0/government-integration-{}.tar.gz", self.base_urls[0], system_arch),
                destination: installation_path.join("downloads/government-integration.tar.gz"),
                expected_size: 20_000_000, // 20MB
                expected_checksum: "sha256:yzab567...".to_string(),
                priority: DownloadPriority::Medium,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },

            DownloadTask {
                id: "bpi-documentation".to_string(),
                name: "BPI Documentation & Examples".to_string(),
                url: format!("{}/v1.0.0/bpi-documentation.tar.gz", self.base_urls[0]),
                destination: installation_path.join("downloads/bpi-documentation.tar.gz"),
                expected_size: 15_000_000, // 15MB
                expected_checksum: "sha256:cdef890...".to_string(),
                priority: DownloadPriority::Low,
                status: DownloadStatus::Pending,
                progress: DownloadProgress::new(),
            },
        ];

        // Sort by priority and add to queue
        let mut sorted_tasks = tasks;
        sorted_tasks.sort_by(|a, b| {
            let priority_order = |p: &DownloadPriority| match p {
                DownloadPriority::Critical => 0,
                DownloadPriority::High => 1,
                DownloadPriority::Medium => 2,
                DownloadPriority::Low => 3,
            };
            priority_order(&a.priority).cmp(&priority_order(&b.priority))
        });

        self.download_queue = sorted_tasks;
        Ok(())
    }

    /// Start downloading all queued tasks
    pub async fn start_downloads(&mut self) -> Result<()> {
        tracing::info!("Starting BPI component downloads...");

        while !self.download_queue.is_empty() && self.active_downloads.len() < self.max_concurrent_downloads {
            if let Some(mut task) = self.download_queue.pop() {
                let task_id = task.id.clone();
                task.status = DownloadStatus::Downloading;
                
                // Create download directory if it doesn't exist
                if let Some(parent) = task.destination.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                self.active_downloads.insert(task_id.clone(), task.clone());
                
                // Start download in background
                let client = self.client.clone();
                let mut task_clone = task.clone();
                
                tokio::spawn(async move {
                    if let Err(e) = Self::download_file(&client, &mut task_clone).await {
                        tracing::error!("Download failed for {}: {}", task_clone.name, e);
                        task_clone.status = DownloadStatus::Failed(e.to_string());
                    }
                });
            }
        }

        Ok(())
    }

    /// Download a single file with resume capability
    async fn download_file(client: &Client, task: &mut DownloadTask) -> Result<()> {
        tracing::info!("Downloading: {}", task.name);

        // Check if partial file exists
        let mut start_byte = 0u64;
        if task.destination.exists() {
            start_byte = tokio::fs::metadata(&task.destination).await?.len();
            task.progress.bytes_downloaded = start_byte;
        }

        // Build request with range header for resume
        let mut request = client.get(&task.url);
        if start_byte > 0 {
            request = request.header("Range", format!("bytes={}-", start_byte));
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", response.status()));
        }

        // Get total size from headers
        if let Some(content_length) = response.headers().get("content-length") {
            if let Ok(length_str) = content_length.to_str() {
                if let Ok(length) = length_str.parse::<u64>() {
                    task.progress.total_bytes = start_byte + length;
                }
            }
        }

        // Open file for writing (append mode for resume)
        let mut file = if start_byte > 0 {
            File::options().append(true).open(&task.destination).await?
        } else {
            File::create(&task.destination).await?
        };

        // Download with progress tracking
        let mut stream = response.bytes_stream();
        let mut hasher = Sha256::new();
        let start_time = std::time::Instant::now();

        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            hasher.update(&chunk);
            
            task.progress.bytes_downloaded += chunk.len() as u64;
            task.progress.percentage = (task.progress.bytes_downloaded as f64 / task.progress.total_bytes as f64) * 100.0;
            
            // Calculate speed and ETA
            let elapsed = start_time.elapsed().as_secs();
            if elapsed > 0 {
                task.progress.speed_bps = task.progress.bytes_downloaded / elapsed;
                let remaining_bytes = task.progress.total_bytes - task.progress.bytes_downloaded;
                task.progress.eta_seconds = remaining_bytes / task.progress.speed_bps.max(1);
            }
        }

        file.flush().await?;
        task.status = DownloadStatus::Verifying;

        // Verify checksum
        let calculated_hash = format!("sha256:{:x}", hasher.finalize());
        if calculated_hash == task.expected_checksum {
            task.status = DownloadStatus::Verified;
            tracing::info!("Download completed and verified: {}", task.name);
        } else {
            task.status = DownloadStatus::Failed("Checksum verification failed".to_string());
            // Remove corrupted file
            let _ = tokio::fs::remove_file(&task.destination).await;
        }

        Ok(())
    }

    /// Get download progress for all tasks
    pub fn get_download_progress(&self) -> Vec<&DownloadTask> {
        self.active_downloads.values().collect()
    }

    /// Get overall download progress
    pub fn get_overall_progress(&self) -> (f64, String) {
        if self.active_downloads.is_empty() && self.download_queue.is_empty() {
            return (100.0, "All downloads completed".to_string());
        }

        let total_tasks = self.active_downloads.len() + self.download_queue.len();
        let completed_tasks = self.active_downloads.values()
            .filter(|task| matches!(task.status, DownloadStatus::Verified))
            .count();

        let progress = (completed_tasks as f64 / total_tasks as f64) * 100.0;
        let status = if completed_tasks == total_tasks {
            "All downloads completed".to_string()
        } else {
            format!("Downloading BPI components ({}/{})", completed_tasks, total_tasks)
        };

        (progress, status)
    }

    /// Check if all downloads are completed
    pub fn are_downloads_complete(&self) -> bool {
        self.download_queue.is_empty() && 
        self.active_downloads.values().all(|task| matches!(task.status, DownloadStatus::Verified))
    }

    /// Get failed downloads
    pub fn get_failed_downloads(&self) -> Vec<&DownloadTask> {
        self.active_downloads.values()
            .filter(|task| matches!(task.status, DownloadStatus::Failed(_)))
            .collect()
    }

    /// Retry failed downloads
    pub async fn retry_failed_downloads(&mut self) -> Result<()> {
        let failed_tasks: Vec<DownloadTask> = self.active_downloads.values()
            .filter(|task| matches!(task.status, DownloadStatus::Failed(_)))
            .cloned()
            .collect();

        for mut task in failed_tasks {
            task.status = DownloadStatus::Pending;
            task.progress = DownloadProgress::new();
            self.download_queue.push(task);
        }

        // Remove failed tasks from active downloads
        self.active_downloads.retain(|_, task| !matches!(task.status, DownloadStatus::Failed(_)));

        self.start_downloads().await
    }
}

impl DownloadProgress {
    fn new() -> Self {
        Self {
            bytes_downloaded: 0,
            total_bytes: 0,
            speed_bps: 0,
            eta_seconds: 0,
            percentage: 0.0,
        }
    }
}
