use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::io::Write;
use tabled::{Table, Tabled};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Human-readable table format
    Table,
    /// JSON format for machine parsing
    Json,
    /// CSV format for data analysis
    Csv,
    /// YAML format for configuration
    Yaml,
    /// Raw text output
    Raw,
}

pub fn format_output<T>(
    data: &T,
    format: &OutputFormat,
    writer: &mut dyn Write,
) -> Result<()>
where
    T: Serialize + Tabled + std::fmt::Debug,
{
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(data)?;
            writeln!(writer, "{}", json)?;
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(data)?;
            writeln!(writer, "{}", yaml)?;
        }
        OutputFormat::Csv => {
            let mut csv_writer = csv::Writer::from_writer(writer);
            csv_writer.serialize(data)?;
            csv_writer.flush()?;
        }
        OutputFormat::Table => {
            let table = Table::new([data]).to_string();
            writeln!(writer, "{}", table)?;
        }
        OutputFormat::Raw => {
            writeln!(writer, "{:?}", data)?;
        }
    }
    Ok(())
}

pub fn format_list<T>(
    data: &[T],
    format: &OutputFormat,
    writer: &mut dyn Write,
) -> Result<()>
where
    T: Serialize + Tabled + std::fmt::Debug,
{
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(data)?;
            writeln!(writer, "{}", json)?;
        }
        OutputFormat::Yaml => {
            let yaml = serde_yaml::to_string(data)?;
            writeln!(writer, "{}", yaml)?;
        }
        OutputFormat::Csv => {
            let mut csv_writer = csv::Writer::from_writer(writer);
            for item in data {
                csv_writer.serialize(item)?;
            }
            csv_writer.flush()?;
        }
        OutputFormat::Table => {
            if !data.is_empty() {
                let table = Table::new(data).to_string();
                writeln!(writer, "{}", table)?;
            }
        }
        OutputFormat::Raw => {
            for item in data {
                writeln!(writer, "{:?}", item)?;
            }
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct StatusInfo {
    pub component: String,
    pub status: String,
    pub uptime: String,
    pub health: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_mb: u64,
    pub status: String,
    pub security_flags: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub permissions: String,
    pub modified: String,
    pub hash: Option<String>,
}

pub fn print_success(message: &str, use_color: bool) {
    if use_color {
        println!("\x1b[32m✓\x1b[0m {}", message);
    } else {
        println!("✓ {}", message);
    }
}

pub fn print_warning(message: &str, use_color: bool) {
    if use_color {
        println!("\x1b[33m⚠\x1b[0m {}", message);
    } else {
        println!("⚠ {}", message);
    }
}

pub fn print_error(message: &str, use_color: bool) {
    if use_color {
        eprintln!("\x1b[31m✗\x1b[0m {}", message);
    } else {
        eprintln!("✗ {}", message);
    }
}

pub fn print_info(message: &str, use_color: bool) {
    if use_color {
        println!("\x1b[34mℹ\x1b[0m {}", message);
    } else {
        println!("ℹ {}", message);
    }
}
