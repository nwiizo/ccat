use anyhow::Result;
use clap::Parser;
use ccat::cli::{Cli, Commands};
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Show(args)) => ccat::cli::commands::show::execute(args),
        Some(Commands::Diagnose(args)) => ccat::cli::commands::diagnose::execute(args),
        Some(Commands::Search(args)) => ccat::cli::commands::search::execute(args),
        Some(Commands::Diff(args)) => ccat::cli::commands::diff::execute(args),
        Some(Commands::Watch(args)) => ccat::cli::commands::watch::execute(args),
        Some(Commands::Export(args)) => ccat::cli::commands::export::execute(args),
        Some(Commands::Validate(args)) => ccat::cli::commands::validate::execute(args),
        Some(Commands::Init(args)) => ccat::cli::commands::init::execute(args),
        None => {
            // Default to show command with current directory
            let args = ccat::cli::args::ShowArgs {
                path: std::path::PathBuf::from("."),
                format: ccat::cli::args::ShowFormat::Text,
                r#type: None,
                content_only: false,
                no_imports: false,
                include_subdirs: false,
                max_depth: None,
                show_metadata: false,
            };
            ccat::cli::commands::show::execute(args)
        }
    }
}