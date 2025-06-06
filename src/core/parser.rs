use super::{MemoryFile, Import};
use anyhow::Result;
use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static IMPORT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{import\s+([^\}]+)\}\}").unwrap()
});

static SECTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^#+\s+(.+)$").unwrap()
});

#[derive(Debug, Clone)]
pub struct ParsedContent {
    pub sections: Vec<Section>,
    pub imports: Vec<Import>,
    pub metadata: ContentMetadata,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub title: String,
    pub level: usize,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Clone)]
pub struct ContentMetadata {
    pub has_imports: bool,
    pub section_count: usize,
    pub code_block_count: usize,
    pub link_count: usize,
}

pub struct Parser {
    extract_sections: bool,
    #[allow(dead_code)]
    resolve_imports: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            extract_sections: true,
            resolve_imports: true,
        }
    }

    pub fn parse(&self, file: &MemoryFile) -> Result<ParsedContent> {
        let lines: Vec<&str> = file.content.lines().collect();
        let mut sections = Vec::new();
        let mut imports = Vec::new();
        let mut current_section: Option<(String, usize, usize, Vec<String>)> = None;
        let mut in_code_block = false;
        let mut code_block_count = 0;
        let mut link_count = 0;

        for (idx, line) in lines.iter().enumerate() {
            let line_number = idx + 1;

            // Track code blocks
            if line.trim().starts_with("```") {
                if in_code_block {
                    in_code_block = false;
                } else {
                    in_code_block = true;
                    code_block_count += 1;
                }
            }

            // Extract imports
            if let Some(captures) = IMPORT_REGEX.captures(line) {
                if let Some(import_path) = captures.get(1) {
                    imports.push(Import {
                        path: import_path.as_str().trim().to_string(),
                        line_number,
                        resolved_path: None,
                    });
                }
            }

            // Count links
            link_count += self.count_links(line);

            // Extract sections
            if self.extract_sections && !in_code_block {
                if let Some(captures) = SECTION_REGEX.captures(line) {
                    // Save previous section if exists
                    if let Some((title, level, start, content_lines)) = current_section.take() {
                        sections.push(Section {
                            title,
                            level,
                            content: content_lines.join("\n"),
                            line_start: start,
                            line_end: idx,
                        });
                    }

                    // Start new section
                    let title = captures.get(1).unwrap().as_str().to_string();
                    let level = line.chars().take_while(|&c| c == '#').count();
                    current_section = Some((title, level, line_number, Vec::new()));
                } else if let Some((_, _, _, ref mut content_lines)) = current_section {
                    content_lines.push(line.to_string());
                }
            }
        }

        // Save last section
        if let Some((title, level, start, content_lines)) = current_section {
            sections.push(Section {
                title,
                level,
                content: content_lines.join("\n"),
                line_start: start,
                line_end: lines.len(),
            });
        }

        let metadata = ContentMetadata {
            has_imports: !imports.is_empty(),
            section_count: sections.len(),
            code_block_count,
            link_count,
        };

        Ok(ParsedContent {
            sections,
            imports,
            metadata,
        })
    }

    fn count_links(&self, line: &str) -> usize {
        let markdown_link = Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();
        let url_link = Regex::new(r"https?://[^\s]+").unwrap();
        
        markdown_link.find_iter(line).count() + url_link.find_iter(line).count()
    }

    pub fn extract_frontmatter(&self, content: &str) -> Option<HashMap<String, String>> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() || !lines[0].trim().eq("---") {
            return None;
        }

        let mut frontmatter = HashMap::new();

        for line in lines.iter().skip(1) {
            if line.trim().eq("---") {
                break;
            }
            
            if let Some((key, value)) = line.split_once(':') {
                frontmatter.insert(
                    key.trim().to_string(),
                    value.trim().to_string()
                );
            }
        }

        if frontmatter.is_empty() {
            None
        } else {
            Some(frontmatter)
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}