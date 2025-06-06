pub mod args;
pub mod commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "ccat")]
#[command(about = "CLAUDE.md Context Analyzer", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long)]
    pub quiet: bool,

    #[arg(long, value_enum, default_value = "auto")]
    pub color: ColorWhen,

    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Display memory files (default)")]
    Show(args::ShowArgs),
    
    #[command(about = "Run diagnostics on configuration")]
    Diagnose(args::DiagnoseArgs),
    
    #[command(about = "Search within context")]
    Search(args::SearchArgs),
    
    #[command(about = "Show differences between files")]
    Diff(args::DiffArgs),
    
    #[command(about = "Watch for changes")]
    Watch(args::WatchArgs),
    
    #[command(about = "Export context to various formats")]
    Export(args::ExportArgs),
    
    #[command(about = "Validate configuration")]
    Validate(args::ValidateArgs),
    
    #[command(about = "Initialize a new CLAUDE.md file")]
    Init(args::InitArgs),
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ColorWhen {
    Always,
    Never,
    Auto,
}

impl ColorWhen {
    pub fn should_colorize(&self) -> bool {
        match self {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => atty::is(atty::Stream::Stdout),
        }
    }
}