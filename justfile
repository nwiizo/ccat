# Default recipe to display help information
default:
    @just --list

# Run all quality checks
check:
    @echo "🔍 Running all quality checks..."
    just fmt-check
    just clippy
    just test
    just build

# Auto-fix formatting and clippy issues
fix:
    @echo "🔧 Auto-fixing issues..."
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

# Check code formatting
fmt:
    cargo fmt

# Check code formatting (CI mode)
fmt-check:
    @echo "📝 Checking code formatting..."
    cargo fmt --all -- --check

# Run clippy lints
clippy:
    @echo "📎 Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    @echo "🧪 Running tests..."
    cargo test

# Build release binary
build:
    @echo "🏗️  Building release..."
    cargo build --release

# Install the binary locally
install:
    @echo "📦 Installing ccat..."
    cargo install --path .

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Run security audit
audit:
    @echo "🔐 Running security audit..."
    cargo audit

# Check for outdated dependencies
outdated:
    @echo "📅 Checking for outdated dependencies..."
    cargo outdated

# Update dependencies
update:
    @echo "⬆️  Updating dependencies..."
    cargo update

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cargo doc --open

# Run benchmarks
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench

# Profile the application
profile:
    @echo "📊 Profiling application..."
    cargo build --release
    perf record target/release/ccat --help
    perf report

# Release patch version (0.1.0 -> 0.1.1)
release-patch:
    @echo "🚀 Releasing patch version..."
    ./quick-release.sh patch

# Release minor version (0.1.0 -> 0.2.0)
release-minor:
    @echo "🚀 Releasing minor version..."
    ./quick-release.sh minor

# Release major version (0.1.0 -> 1.0.0)
release-major:
    @echo "🚀 Releasing major version..."
    ./quick-release.sh major

# Setup development environment
setup:
    @echo "🛠️  Setting up development environment..."
    rustup component add clippy rustfmt
    cargo install cargo-audit cargo-outdated

# Show project statistics
stats:
    @echo "📈 Project statistics:"
    @echo "Lines of code:"
    find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo "Dependencies:"
    cargo tree --depth 1 | wc -l
    @echo "Test coverage:"
    cargo test 2>&1 | grep "test result" || echo "No tests found"

# Lint all files
lint:
    @echo "🔍 Linting all files..."
    just clippy
    just fmt-check