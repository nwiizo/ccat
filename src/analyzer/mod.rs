pub mod diagnostics;
pub mod profiler;
pub mod security;
pub mod validator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticResult {
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub suggestions: Vec<Suggestion>,
    pub metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub message: String,
    pub fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_files: usize,
    pub total_size: u64,
    pub import_depth: usize,
    pub circular_imports: usize,
}

pub use diagnostics::Diagnostics;
