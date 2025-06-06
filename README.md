# ccat - CLAUDE.md Context Analyzer

A comprehensive command-line tool for analyzing and managing Claude Code memory files (CLAUDE.md).

## Features

- 🔍 **Smart File Discovery**: Automatically finds CLAUDE.md files in user, project, and subdirectory locations
- 📊 **Multiple Output Formats**: Text, JSON, Tree view, and raw output
- 🔗 **Import Resolution**: Tracks and validates import chains between memory files
- 🏥 **Diagnostics**: Detects issues like circular imports, missing files, and large contexts
- 🔎 **Context Search**: Search within memory files with regex support
- 📦 **Export Capabilities**: Export contexts to various formats (Markdown, JSON, HTML, PDF)
- 🔧 **Claude Code Integration**: Direct integration with Claude Code for automated fixes

## Installation

### From crates.io (Recommended)

```bash
cargo install ccat
```

### From source

```bash
git clone https://github.com/nwiizo/ccat.git
cd ccat
cargo install --path .
```

### From GitHub releases

Download the latest binary from the [releases page](https://github.com/nwiizo/ccat/releases).

## Usage

### Basic Commands

```bash
# Show memory files in current directory
ccat

# Show memory files with tree view
ccat show -f tree

# Include subdirectories
ccat show -s

# Show only specific types
ccat show -t project,user

# Run diagnostics
ccat diagnose

# Search for patterns
ccat search "TODO"

# Initialize new CLAUDE.md
ccat init
```

### Advanced Usage

```bash
# Show with metadata
ccat show --show-metadata

# Export to JSON
ccat export -f json -o context.json

# Watch for changes
ccat watch --notify

# Validate with strict rules
ccat validate --strict
```

## Command Reference

### `show` - Display memory files (default)

```bash
ccat show [OPTIONS] [PATH]

Options:
  -f, --format <FORMAT>     Output format [text|json|tree|raw]
  -t, --type <TYPE>         Filter by type (project, user, local, subdir)
  -c, --content-only        Show content only
  -n, --no-imports          Don't expand imports
  -s, --include-subdirs     Include subdirectories
  -d, --max-depth <N>       Maximum directory depth
  --show-metadata           Show file metadata
```

### `diagnose` - Run diagnostics

```bash
ccat diagnose [OPTIONS] [PATH]

Options:
  --fix                Auto-fix issues
  --strict             Use strict rules
  --rules <RULES>      Custom rule files
  --ignore <PATTERN>   Ignore patterns
```

### `search` - Search within contexts

```bash
ccat search [OPTIONS] <QUERY>

Options:
  -r, --regex          Use regex search
  -i, --ignore-case    Case insensitive
  -w, --word           Word boundaries
  -A, --after <N>      Show N lines after match
  -B, --before <N>     Show N lines before match
```

## Memory File Types

- **Project Memory** (`./CLAUDE.md`): Project-specific context
- **User Memory** (`~/.claude/CLAUDE.md`): Global user settings
- **Local Memory** (`./CLAUDE.local.md`): Deprecated local overrides
- **Subdirectory Memory**: CLAUDE.md files in subdirectories

## Diagnostics

The tool can detect:
- ❌ Circular imports
- ❌ Missing import files
- ⚠️ Large files (>1MB)
- ⚠️ Deep import chains
- ⚠️ Duplicate imports
- 🔒 Security issues (API keys, passwords)

## Claude Code Integration

```bash
# Fix issues with Claude
ccat diagnose --fix-with-claude | claude -p "Fix these issues"

# Generate optimized CLAUDE.md
ccat init --analyze-project | claude -p "Generate CLAUDE.md"

# Analyze team patterns
ccat analyze-team --dir=/repos | claude -p "Standardize configs"
```

## Configuration

Configuration file at `~/.config/ccat/config.toml`:

```toml
[display]
format = "tree"
color = "auto"
show_metadata = false

[diagnostics]
strict = false
auto_fix = false
rules = ["default"]

[performance]
parallel = true
cache_size = "100MB"
```

## Development

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Lint
cargo clippy

# Format
cargo fmt
```

## Performance

- Parallel file scanning with rayon
- LRU caching for parsed files
- Incremental import resolution
- Optimized for large codebases

## License

MIT