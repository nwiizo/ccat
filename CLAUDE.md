# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is `ccat`, a command-line tool for analyzing and managing Claude Code memory files (CLAUDE.md). It's built in Rust and provides comprehensive analysis, diagnostics, and export capabilities for CLAUDE.md files across different scopes (user, project, subdirectory).

## Development Commands

### Essential Commands
```bash
# Build and run quality checks (recommended before commits)
just check

# Auto-fix formatting and clippy issues
just fix

# Build release binary
just build

# Run tests
just test

# Install locally
just install
```

### Development Workflow
```bash
# Format code
cargo fmt

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Run specific tests
cargo test [test_name]

# Run benchmarks
cargo bench

# Generate and open documentation
cargo doc --open
```

## Architecture Overview

The codebase is organized into five main modules:

### Core Module (`src/core/`)
- **Scanner**: Discovers CLAUDE.md files in filesystem hierarchy
- **Parser**: Parses memory files and extracts imports/metadata
- **Resolver**: Resolves import chains and validates references
- **Cache**: LRU caching system for parsed files

### CLI Module (`src/cli/`)
- **Commands**: Eight main commands (show, diagnose, search, diff, watch, export, validate, init)
- **Args**: Command-line argument definitions and parsing
- Entry point that routes to appropriate command handlers

### Analyzer Module (`src/analyzer/`)
- **Diagnostics**: Detects issues like circular imports, missing files, large contexts
- **Security**: Scans for exposed secrets, API keys, passwords
- **Validator**: Validates memory file structure and content
- **Profiler**: Performance metrics and analysis

### Formatter Module (`src/formatter/`)
- **Text/Tree/JSON**: Multiple output format implementations
- **Diff**: Comparison and difference highlighting
- Unified `Formatter` trait for consistent output handling

### Claude Module (`src/claude/`)
- **Integration**: Direct integration with Claude Code for automated fixes
- **Protocol**: Communication protocols for Claude Code interaction

## Memory File Types

The tool recognizes four types of memory files:
- **ProjectMemory** (`./CLAUDE.md`): Project-specific context
- **UserMemory** (`~/.claude/CLAUDE.md`): Global user settings  
- **LocalMemory** (`./CLAUDE.local.md`): Deprecated local overrides
- **SubdirMemory**: CLAUDE.md files in subdirectories

## Key Data Structures

- `MemoryFile`: Core structure containing path, type, content, imports, and metadata
- `Import`: Represents import statements with line numbers and resolved paths
- `DiagnosticResult`: Contains errors, warnings, suggestions, and performance metrics

## Performance Considerations

- Uses `rayon` for parallel file scanning and processing
- LRU cache for parsed files to avoid re-parsing
- Incremental import resolution
- Optimized for large codebases with many memory files

## Testing Strategy

- Unit tests for core parsing and analysis logic
- Integration tests using `assert_cmd` for CLI commands
- Benchmark tests for performance-critical operations
- Property tests with `proptest` for edge cases
- Snapshot tests with `insta` for output validation

## Configuration

The tool supports configuration via `~/.config/ccat/config.toml` with sections for display, diagnostics, and performance settings.