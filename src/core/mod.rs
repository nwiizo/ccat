pub mod cache;
pub mod parser;
pub mod resolver;
pub mod scanner;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFile {
    pub path: PathBuf,
    pub file_type: MemoryType,
    pub content: String,
    pub imports: Vec<Import>,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryType {
    ProjectMemory, // ./CLAUDE.md
    UserMemory,    // ~/.claude/CLAUDE.md
    LocalMemory,   // ./CLAUDE.local.md (deprecated)
    SubdirMemory,  // サブディレクトリのCLAUDE.md
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub line_number: usize,
    pub resolved_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub hash: String,
    pub line_count: usize,
}

pub use cache::FileCache;
pub use parser::Parser;
pub use resolver::ImportResolver;
pub use scanner::Scanner;
