use super::Formatter;
use crate::core::{MemoryFile, MemoryType};
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

pub struct TextFormatter {
    content_only: bool,
    show_metadata: bool,
    expand_imports: bool,
}

impl TextFormatter {
    pub fn new() -> Self {
        Self {
            content_only: false,
            show_metadata: false,
            expand_imports: true,
        }
    }

    pub fn with_content_only(mut self, value: bool) -> Self {
        self.content_only = value;
        self
    }

    pub fn with_show_metadata(mut self, value: bool) -> Self {
        self.show_metadata = value;
        self
    }

    pub fn with_expand_imports(mut self, value: bool) -> Self {
        self.expand_imports = value;
        self
    }

    fn format_header(&self, file: &MemoryFile) -> String {
        let type_badge = match file.file_type {
            MemoryType::ProjectMemory => "[PROJECT]".green(),
            MemoryType::UserMemory => "[USER]".blue(),
            MemoryType::LocalMemory => "[LOCAL]".yellow(),
            MemoryType::SubdirMemory => "[SUBDIR]".cyan(),
        };

        format!("{} {}", type_badge, file.path.display().to_string().bold())
    }

    fn format_metadata(&self, file: &MemoryFile) -> String {
        let meta = &file.metadata;
        format!(
            "  Size: {} | Lines: {} | Modified: {} | Hash: {}",
            format_size(meta.size).dimmed(),
            meta.line_count.to_string().dimmed(),
            meta.modified
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .dimmed(),
            &meta.hash[..8].dimmed()
        )
    }

    fn format_imports(&self, file: &MemoryFile) -> String {
        if file.imports.is_empty() {
            return String::new();
        }

        let mut output = String::from("  Imports:\n");
        for import in &file.imports {
            let status = if import.resolved_path.is_some() {
                "✓".green()
            } else {
                "✗".red()
            };
            output.push_str(&format!(
                "    {} {} (line {})\n",
                status, import.path, import.line_number
            ));
        }
        output
    }
}

impl Formatter for TextFormatter {
    fn format(&self, files: &[MemoryFile]) -> Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for (i, file) in files.iter().enumerate() {
            if i > 0 {
                writeln!(handle, "\n{}", "─".repeat(80).dimmed())?;
            }

            // Header
            if !self.content_only {
                writeln!(handle, "{}", self.format_header(file))?;

                if self.show_metadata {
                    writeln!(handle, "{}", self.format_metadata(file))?;
                }

                if !file.imports.is_empty() {
                    write!(handle, "{}", self.format_imports(file))?;
                }

                writeln!(handle)?;
            }

            // Content
            if self.content_only {
                write!(handle, "{}", file.content)?;
            } else {
                // Add line numbers
                for (line_no, line) in file.content.lines().enumerate() {
                    writeln!(
                        handle,
                        "{:4} │ {}",
                        (line_no + 1).to_string().dimmed(),
                        line
                    )?;
                }
            }
        }

        Ok(())
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
