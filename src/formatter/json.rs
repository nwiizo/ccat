use super::Formatter;
use crate::core::MemoryFile;
use anyhow::Result;
use serde_json;

pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self { pretty: true }
    }

    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, files: &[MemoryFile]) -> Result<()> {
        if self.pretty {
            println!("{}", serde_json::to_string_pretty(files)?);
        } else {
            println!("{}", serde_json::to_string(files)?);
        }
        Ok(())
    }
}
