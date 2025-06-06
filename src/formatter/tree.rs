use super::Formatter;
use crate::core::{MemoryFile, MemoryType};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use termtree::Tree;

pub struct TreeFormatter;

impl Default for TreeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for TreeFormatter {
    fn format(&self, files: &[MemoryFile]) -> Result<()> {
        let mut root = Tree::new("CLAUDE.md Files".bold().to_string());

        // Group files by type
        let mut user_files = Vec::new();
        let mut project_files = Vec::new();
        let mut subdir_files = Vec::new();
        let mut local_files = Vec::new();

        for file in files {
            match file.file_type {
                MemoryType::UserMemory => user_files.push(file),
                MemoryType::ProjectMemory => project_files.push(file),
                MemoryType::SubdirMemory => subdir_files.push(file),
                MemoryType::LocalMemory => local_files.push(file),
            }
        }

        // Add user memory
        if !user_files.is_empty() {
            let mut user_node = Tree::new("User Memory".blue().to_string());
            for file in user_files {
                user_node.push(format_file_node(file));
            }
            root.push(user_node);
        }

        // Add project memory
        if !project_files.is_empty() {
            let mut project_node = Tree::new("Project Memory".green().to_string());
            for file in project_files {
                project_node.push(format_file_node(file));
            }
            root.push(project_node);
        }

        // Add subdir memories
        if !subdir_files.is_empty() {
            let mut subdir_node = Tree::new("Subdirectory Memories".cyan().to_string());

            // Build directory tree
            for file in subdir_files {
                let relative_path = file.path.strip_prefix(".").unwrap_or(&file.path);
                add_to_tree(&mut subdir_node, relative_path, file);
            }

            root.push(subdir_node);
        }

        // Add local memory (deprecated)
        if !local_files.is_empty() {
            let mut local_node = Tree::new("Local Memory (Deprecated)".yellow().to_string());
            for file in local_files {
                local_node.push(format_file_node(file));
            }
            root.push(local_node);
        }

        println!("{}", root);
        Ok(())
    }
}

fn format_file_node(file: &MemoryFile) -> Tree<String> {
    let meta = &file.metadata;
    let imports_info = if file.imports.is_empty() {
        String::new()
    } else {
        format!(" [{} imports]", file.imports.len())
            .dimmed()
            .to_string()
    };

    let info = format!(
        "{} ({} lines, {}{})",
        file.path.file_name().unwrap_or_default().to_string_lossy(),
        meta.line_count,
        format_size(meta.size),
        imports_info
    );

    Tree::new(info)
}

fn add_to_tree(tree: &mut Tree<String>, path: &Path, file: &MemoryFile) {
    let components: Vec<_> = path
        .parent()
        .unwrap_or(Path::new(""))
        .components()
        .collect();

    if components.is_empty() {
        tree.push(format_file_node(file));
        return;
    }

    // Build path to the file
    let mut path_components = Vec::new();
    for component in components {
        path_components.push(component.as_os_str().to_string_lossy().to_string());
    }

    // Navigate or create the path
    fn ensure_path<'a>(
        tree: &'a mut Tree<String>,
        path: &[String],
        depth: usize,
    ) -> &'a mut Tree<String> {
        if depth >= path.len() {
            return tree;
        }

        let name = &path[depth];

        // Find or create child
        let child_idx = tree.leaves.iter().position(|child| &child.root == name);

        if child_idx.is_none() {
            tree.push(Tree::new(name.clone()));
        }

        let child_idx = tree
            .leaves
            .iter()
            .position(|child| &child.root == name)
            .unwrap();
        ensure_path(&mut tree.leaves[child_idx], path, depth + 1)
    }

    let target = ensure_path(tree, &path_components, 0);
    target.push(format_file_node(file));
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
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
