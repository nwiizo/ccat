pub mod diff;
pub mod json;
pub mod text;
pub mod tree;

use crate::core::MemoryFile;
use anyhow::Result;

pub trait Formatter {
    fn format(&self, files: &[MemoryFile]) -> Result<()>;
}

pub use diff::DiffFormatter;
pub use json::JsonFormatter;
pub use text::TextFormatter;
pub use tree::TreeFormatter;
