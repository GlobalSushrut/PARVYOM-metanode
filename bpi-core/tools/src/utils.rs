// Utils module for BPI Advanced Downloader
// Utility functions and helpers

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileUtils;

impl FileUtils {
    pub fn get_bpi_install_dir() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join(".bpi")
    }

    pub fn get_config_dir() -> PathBuf {
        Self::get_bpi_install_dir().join("config")
    }

    pub fn get_data_dir() -> PathBuf {
        Self::get_bpi_install_dir().join("data")
    }

    pub async fn ensure_directories() -> Result<()> {
        let dirs = vec![
            Self::get_bpi_install_dir(),
            Self::get_config_dir(),
            Self::get_data_dir(),
        ];

        for dir in dirs {
            if !dir.exists() {
                tokio::fs::create_dir_all(&dir).await?;
                tracing::info!("Created directory: {:?}", dir);
            }
        }

        Ok(())
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

pub fn validate_system_requirements() -> Result<bool> {
    // Basic system validation
    Ok(true)
}
