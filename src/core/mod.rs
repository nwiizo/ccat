pub mod scanner;
pub mod parser;
pub mod resolver;
pub mod cache;

use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

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
    ProjectMemory,      // ./CLAUDE.md
    UserMemory,         // ~/.claude/CLAUDE.md
    LocalMemory,        // ./CLAUDE.local.md (deprecated)
    SubdirMemory,       // サブディレクトリのCLAUDE.md
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

pub use scanner::Scanner;
pub use parser::Parser;
pub use resolver::ImportResolver;
pub use cache::FileCache;