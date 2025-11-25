use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Quiet mode - suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Output format
    #[arg(long, global = true, value_enum, default_value = "table")]
    pub format: crate::cli::output::OutputFormat,

    /// Output file (default: stdout)
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Configuration file path
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Enable dry-run mode (show what would be done without executing)
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Force operation without confirmation prompts
    #[arg(long, global = true)]
    pub force: bool,

    /// Enable JSON output for machine parsing
    #[arg(long, global = true)]
    pub json: bool,

    /// Show timestamps in output
    #[arg(long, global = true)]
    pub timestamps: bool,

    /// Enable colored output
    #[arg(long, global = true, default_value = "auto")]
    pub color: ColorMode,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

impl GlobalArgs {
    pub fn should_use_color(&self) -> bool {
        match self.color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => atty::is(atty::Stream::Stdout),
        }
    }

    pub fn is_machine_readable(&self) -> bool {
        self.json || matches!(self.format, crate::cli::output::OutputFormat::Json)
    }
}
