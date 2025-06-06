use super::{MemoryFile, MemoryType, FileMetadata};
use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use rayon::prelude::*;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use std::io::Read;

pub struct Scanner {
    include_subdirs: bool,
    max_depth: Option<usize>,
    follow_symlinks: bool,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            include_subdirs: false,
            max_depth: None,
            follow_symlinks: false,
        }
    }

    pub fn with_subdirs(mut self, include: bool) -> Self {
        self.include_subdirs = include;
        self
    }

    pub fn with_max_depth(mut self, depth: Option<usize>) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn with_follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }

    pub fn scan<P: AsRef<Path>>(&self, path: P) -> Result<Vec<MemoryFile>> {
        let path = path.as_ref();
        let mut files = Vec::new();

        // Check for user memory file
        if let Some(home) = dirs::home_dir() {
            let user_memory = home.join(".claude").join("CLAUDE.md");
            if user_memory.exists() {
                if let Ok(file) = self.scan_file(&user_memory, MemoryType::UserMemory) {
                    files.push(file);
                }
            }
        }

        // Check for project memory files
        let project_memory = path.join("CLAUDE.md");
        if project_memory.exists() {
            if let Ok(file) = self.scan_file(&project_memory, MemoryType::ProjectMemory) {
                files.push(file);
            }
        }

        // Check for local memory (deprecated)
        let local_memory = path.join("CLAUDE.local.md");
        if local_memory.exists() {
            if let Ok(file) = self.scan_file(&local_memory, MemoryType::LocalMemory) {
                files.push(file);
            }
        }

        // Scan subdirectories if requested
        if self.include_subdirs {
            let subdir_files = self.scan_subdirs(path)?;
            files.extend(subdir_files);
        }

        Ok(files)
    }

    fn scan_subdirs<P: AsRef<Path>>(&self, path: P) -> Result<Vec<MemoryFile>> {
        let path = path.as_ref();
        let mut walker = WalkDir::new(path)
            .follow_links(self.follow_symlinks);

        if let Some(depth) = self.max_depth {
            walker = walker.max_depth(depth);
        }

        let entries: Vec<_> = walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name() == "CLAUDE.md" 
                && e.path() != path.join("CLAUDE.md")
                && e.path() != path.join("CLAUDE.local.md")
            })
            .collect();

        let files: Vec<_> = entries
            .par_iter()
            .filter_map(|entry| {
                self.scan_file(entry.path(), MemoryType::SubdirMemory).ok()
            })
            .collect();

        Ok(files)
    }

    fn scan_file(&self, path: &Path, file_type: MemoryType) -> Result<MemoryFile> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let metadata = self.get_file_metadata(path)?;
        
        // Parse imports (basic implementation - will be enhanced)
        let imports = self.extract_imports(&content);

        Ok(MemoryFile {
            path: path.to_path_buf(),
            file_type,
            content,
            imports,
            metadata,
        })
    }

    fn get_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let size = metadata.len();
        
        // Calculate file hash
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];
        
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        
        let hash = format!("{:x}", hasher.finalize());
        
        // Count lines
        let content = fs::read_to_string(path)?;
        let line_count = content.lines().count();

        Ok(FileMetadata {
            size,
            modified: DateTime::<Utc>::from(modified),
            hash,
            line_count,
        })
    }

    fn extract_imports(&self, content: &str) -> Vec<super::Import> {
        let mut imports = Vec::new();
        
        for (line_number, line) in content.lines().enumerate() {
            if line.trim_start().starts_with("{{import") {
                if let Some(path) = self.parse_import_line(line) {
                    imports.push(super::Import {
                        path,
                        line_number: line_number + 1,
                        resolved_path: None,
                    });
                }
            }
        }
        
        imports
    }

    fn parse_import_line(&self, line: &str) -> Option<String> {
        // Basic import parsing - will be enhanced
        let trimmed = line.trim();
        if trimmed.starts_with("{{import") && trimmed.ends_with("}}") {
            let content = &trimmed[8..trimmed.len()-2].trim();
            Some(content.to_string())
        } else {
            None
        }
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}