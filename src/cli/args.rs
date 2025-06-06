use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct ShowArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(short, long, value_enum, default_value = "text")]
    pub format: ShowFormat,

    #[arg(short = 't', long)]
    pub r#type: Option<Vec<String>>,

    #[arg(short = 'c', long)]
    pub content_only: bool,

    #[arg(short = 'n', long)]
    pub no_imports: bool,

    #[arg(short = 's', long)]
    pub include_subdirs: bool,

    #[arg(short = 'd', long)]
    pub max_depth: Option<usize>,

    #[arg(long)]
    pub show_metadata: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ShowFormat {
    Text,
    Json,
    Tree,
    Raw,
}

#[derive(Debug, Args)]
pub struct DiagnoseArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub fix: bool,

    #[arg(long)]
    pub strict: bool,

    #[arg(long)]
    pub rules: Vec<String>,

    #[arg(long)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    pub query: String,

    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(short = 'r', long)]
    pub regex: bool,

    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    #[arg(short = 'w', long)]
    pub word: bool,

    #[arg(short = 'A', long, value_name = "N")]
    pub after: Option<usize>,

    #[arg(short = 'B', long, value_name = "N")]
    pub before: Option<usize>,

    #[arg(long)]
    pub r#type: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct DiffArgs {
    pub file1: PathBuf,
    pub file2: PathBuf,

    #[arg(short = 'u', long, default_value = "3")]
    pub unified: usize,

    #[arg(long)]
    pub color: bool,

    #[arg(long)]
    pub side_by_side: bool,
}

#[derive(Debug, Args)]
pub struct WatchArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long, default_value = "1000")]
    pub interval: u64,

    #[arg(long, default_value = "300")]
    pub debounce: u64,

    #[arg(long)]
    pub notify: bool,

    #[arg(long)]
    pub exec: Option<String>,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(short, long, value_enum, default_value = "markdown")]
    pub format: ExportFormat,

    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    #[arg(long)]
    pub include_metadata: bool,

    #[arg(long)]
    pub expand_imports: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ExportFormat {
    Markdown,
    Json,
    Html,
    Pdf,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub strict: bool,

    #[arg(long)]
    pub fix: bool,
}

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub analyze_project: bool,

    #[arg(long)]
    pub template: Option<String>,

    #[arg(long)]
    pub force: bool,
}
