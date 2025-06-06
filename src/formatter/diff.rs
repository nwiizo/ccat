use super::Formatter;
use crate::core::MemoryFile;
use anyhow::Result;

pub struct DiffFormatter;

impl Default for DiffFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for DiffFormatter {
    fn format(&self, _files: &[MemoryFile]) -> Result<()> {
        // TODO: Implement diff formatting
        Ok(())
    }
}
