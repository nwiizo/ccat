use crate::cli::args::{ShowArgs, ShowFormat};
use crate::core::{MemoryType, Parser, Scanner};
use crate::formatter::{Formatter, JsonFormatter, TextFormatter, TreeFormatter};
use anyhow::{Context, Result};
use colored::Colorize;

pub fn execute(args: ShowArgs) -> Result<()> {
    let scanner = Scanner::new()
        .with_subdirs(args.include_subdirs)
        .with_max_depth(args.max_depth);

    let mut files = scanner
        .scan(&args.path)
        .context("Failed to scan for CLAUDE.md files")?;

    // Filter by type if specified
    if let Some(ref types) = args.r#type {
        files.retain(|f| {
            let type_str = match f.file_type {
                MemoryType::ProjectMemory => "project",
                MemoryType::UserMemory => "user",
                MemoryType::LocalMemory => "local",
                MemoryType::SubdirMemory => "subdir",
            };
            types.contains(&type_str.to_string())
        });
    }

    if files.is_empty() {
        eprintln!("{}", "No CLAUDE.md files found".yellow());
        return Ok(());
    }

    // Parse files if needed
    let parser = Parser::new();
    let mut parsed_files = Vec::new();

    for file in &files {
        let parsed = parser.parse(file)?;
        parsed_files.push((file, parsed));
    }

    // Format and display
    match args.format {
        ShowFormat::Text => {
            let formatter = TextFormatter::new()
                .with_content_only(args.content_only)
                .with_show_metadata(args.show_metadata)
                .with_expand_imports(!args.no_imports);
            formatter.format(&files)?;
        }
        ShowFormat::Json => {
            let formatter = JsonFormatter::new();
            formatter.format(&files)?;
        }
        ShowFormat::Tree => {
            let formatter = TreeFormatter::new();
            formatter.format(&files)?;
        }
        ShowFormat::Raw => {
            for file in &files {
                println!("{}", file.content);
            }
        }
    }

    Ok(())
}
