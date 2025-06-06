pub mod analyzer;
pub mod claude;
pub mod cli;
pub mod core;
pub mod formatter;

pub use analyzer::{DiagnosticResult, Diagnostics};
pub use core::{MemoryFile, MemoryType, Parser, Scanner};
pub use formatter::Formatter;

use anyhow::Result;
use std::path::Path;

pub fn analyze<P: AsRef<Path>>(path: P) -> Result<Vec<MemoryFile>> {
    let scanner = Scanner::new();
    let files = scanner.scan(path)?;
    Ok(files)
}

pub fn diagnose<P: AsRef<Path>>(path: P) -> Result<DiagnosticResult> {
    let files = analyze(path)?;
    let diagnostics = Diagnostics::new();
    let result = diagnostics.check(&files)?;
    Ok(result)
}
