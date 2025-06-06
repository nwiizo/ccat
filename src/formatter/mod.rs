pub mod text;
pub mod json;
pub mod tree;
pub mod diff;

use crate::core::MemoryFile;
use anyhow::Result;

pub trait Formatter {
    fn format(&self, files: &[MemoryFile]) -> Result<()>;
}

pub use text::TextFormatter;
pub use json::JsonFormatter;
pub use tree::TreeFormatter;
pub use diff::DiffFormatter;