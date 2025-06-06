use super::MemoryFile;
use anyhow::{bail, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ImportResolver {
    base_path: PathBuf,
    resolved_cache: HashMap<String, PathBuf>,
    #[allow(dead_code)]
    visited: HashSet<PathBuf>,
}

impl ImportResolver {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            resolved_cache: HashMap::new(),
            visited: HashSet::new(),
        }
    }

    pub fn resolve_imports(&mut self, file: &mut MemoryFile) -> Result<()> {
        let file_dir = file
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        for import in &mut file.imports {
            if let Some(resolved) = self.resolve_import_path(&import.path, file_dir)? {
                import.resolved_path = Some(resolved);
            }
        }

        Ok(())
    }

    pub fn resolve_all(&mut self, files: &mut [MemoryFile]) -> Result<()> {
        for file in files {
            self.resolve_imports(file)?;
        }
        Ok(())
    }

    pub fn check_circular_imports(&self, files: &[MemoryFile]) -> Result<Vec<CircularImport>> {
        let mut circular_imports = Vec::new();
        let mut import_graph: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

        // Build import graph
        for file in files {
            let mut deps = Vec::new();
            for import in &file.imports {
                if let Some(resolved) = &import.resolved_path {
                    deps.push(resolved.clone());
                }
            }
            import_graph.insert(file.path.clone(), deps);
        }

        // Check for cycles using DFS
        for path in import_graph.keys() {
            let mut visited = HashSet::new();
            let mut stack = Vec::new();

            if let Some(cycle) = Self::find_cycle(path, &import_graph, &mut visited, &mut stack) {
                circular_imports.push(CircularImport {
                    cycle: cycle.clone(),
                });
            }
        }

        Ok(circular_imports)
    }

    fn resolve_import_path(
        &mut self,
        import_path: &str,
        base_dir: &Path,
    ) -> Result<Option<PathBuf>> {
        // Check cache
        if let Some(cached) = self.resolved_cache.get(import_path) {
            return Ok(Some(cached.clone()));
        }

        // Try different resolution strategies
        let candidates = vec![
            // Relative to current file
            base_dir.join(import_path),
            // Relative to project root
            self.base_path.join(import_path),
            // Absolute path
            PathBuf::from(import_path),
            // With .md extension
            base_dir.join(format!("{}.md", import_path)),
            self.base_path.join(format!("{}.md", import_path)),
        ];

        for candidate in candidates {
            if candidate.exists() && candidate.is_file() {
                let canonical = candidate.canonicalize()?;
                self.resolved_cache
                    .insert(import_path.to_string(), canonical.clone());
                return Ok(Some(canonical));
            }
        }

        Ok(None)
    }

    fn find_cycle(
        current: &Path,
        graph: &HashMap<PathBuf, Vec<PathBuf>>,
        visited: &mut HashSet<PathBuf>,
        stack: &mut Vec<PathBuf>,
    ) -> Option<Vec<PathBuf>> {
        if stack.contains(&current.to_path_buf()) {
            // Found a cycle
            let pos = stack.iter().position(|p| p == current).unwrap();
            return Some(stack[pos..].to_vec());
        }

        if visited.contains(current) {
            return None;
        }

        visited.insert(current.to_path_buf());
        stack.push(current.to_path_buf());

        if let Some(deps) = graph.get(current) {
            for dep in deps {
                if let Some(cycle) = Self::find_cycle(dep, graph, visited, stack) {
                    return Some(cycle);
                }
            }
        }

        stack.pop();
        None
    }

    pub fn expand_imports(&self, file: &MemoryFile, max_depth: usize) -> Result<String> {
        self.expand_imports_recursive(file, 0, max_depth, &mut HashSet::new())
    }

    fn expand_imports_recursive(
        &self,
        file: &MemoryFile,
        depth: usize,
        max_depth: usize,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<String> {
        if depth >= max_depth {
            return Ok(file.content.clone());
        }

        if !visited.insert(file.path.clone()) {
            bail!("Circular import detected: {}", file.path.display());
        }

        let mut expanded = file.content.clone();

        for import in &file.imports {
            if let Some(resolved_path) = &import.resolved_path {
                if let Ok(imported_content) = fs::read_to_string(resolved_path) {
                    let import_line = format!("{{{{import {}}}}}", import.path);
                    let replacement = format!(
                        "<!-- Import from: {} -->\n{}\n<!-- End import -->",
                        resolved_path.display(),
                        imported_content
                    );
                    expanded = expanded.replace(&import_line, &replacement);
                }
            }
        }

        Ok(expanded)
    }
}

#[derive(Debug, Clone)]
pub struct CircularImport {
    pub cycle: Vec<PathBuf>,
}

impl std::fmt::Display for CircularImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circular import: ")?;
        for (i, path) in self.cycle.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}", path.display())?;
        }
        Ok(())
    }
}
