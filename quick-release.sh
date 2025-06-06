#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
usage() {
    echo "Usage: $0 [patch|minor|major]"
    echo ""
    echo "  patch   Increment patch version (0.1.0 -> 0.1.1)"
    echo "  minor   Increment minor version (0.1.0 -> 0.2.0)"
    echo "  major   Increment major version (0.1.0 -> 1.0.0)"
    echo ""
    echo "If no argument is provided, defaults to 'patch'"
    exit 1
}

# Function to check if we're on the main branch
check_branch() {
    local current_branch=$(git branch --show-current)
    if [ "$current_branch" != "main" ]; then
        print_error "Must be on 'main' branch to release. Current branch: $current_branch"
        exit 1
    fi
}

# Function to check for uncommitted changes
check_git_status() {
    if ! git diff-index --quiet HEAD --; then
        print_error "There are uncommitted changes. Please commit or stash them before releasing."
        git status --short
        exit 1
    fi
}

# Function to run quality checks
run_checks() {
    print_status "Running quality checks..."
    
    print_status "Checking formatting..."
    if ! cargo fmt --all -- --check; then
        print_error "Code formatting check failed. Run 'cargo fmt' to fix."
        exit 1
    fi
    
    print_status "Running clippy..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        print_error "Clippy check failed. Fix all warnings before releasing."
        exit 1
    fi
    
    print_status "Running tests..."
    if ! cargo test; then
        print_error "Tests failed. Fix all test failures before releasing."
        exit 1
    fi
    
    print_status "Building release..."
    if ! cargo build --release; then
        print_error "Release build failed."
        exit 1
    fi
    
    print_success "All quality checks passed!"
}

# Function to get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "//; s/"//'
}

# Function to increment version
increment_version() {
    local version=$1
    local increment_type=$2
    
    IFS='.' read -r major minor patch <<< "$version"
    
    case $increment_type in
        "patch")
            patch=$((patch + 1))
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        *)
            print_error "Invalid increment type: $increment_type"
            usage
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Function to update version in Cargo.toml
update_version() {
    local new_version=$1
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = .*/version = \"$new_version\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = .*/version = \"$new_version\"/" Cargo.toml
    fi
}

# Function to create git tag and push
create_and_push_tag() {
    local version=$1
    local tag="v$version"
    
    print_status "Creating git commit for version $version..."
    git add Cargo.toml Cargo.lock
    git commit -m "Release $tag"
    
    print_status "Creating git tag $tag..."
    git tag "$tag"
    
    print_status "Pushing to origin..."
    git push origin main
    git push origin "$tag"
    
    print_success "Tag $tag has been created and pushed!"
}

# Main function
main() {
    local increment_type=${1:-patch}
    
    # Validate increment type
    if [[ ! "$increment_type" =~ ^(patch|minor|major)$ ]]; then
        print_error "Invalid increment type: $increment_type"
        usage
    fi
    
    print_status "Starting release process with $increment_type increment..."
    
    # Pre-flight checks
    check_branch
    check_git_status
    
    # Get current version
    local current_version=$(get_current_version)
    print_status "Current version: $current_version"
    
    # Calculate new version
    local new_version=$(increment_version "$current_version" "$increment_type")
    print_status "New version: $new_version"
    
    # Run quality checks
    run_checks
    
    # Confirm with user
    echo ""
    print_warning "About to release version $new_version"
    print_warning "This will:"
    echo "  1. Update Cargo.toml to version $new_version"
    echo "  2. Create a git commit"
    echo "  3. Create and push git tag v$new_version"
    echo "  4. Trigger GitHub Actions for release automation"
    echo ""
    read -p "Continue? [y/N] " -n 1 -r
    echo ""
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Release cancelled."
        exit 0
    fi
    
    # Update version
    print_status "Updating version in Cargo.toml..."
    update_version "$new_version"
    
    # Update Cargo.lock
    print_status "Updating Cargo.lock..."
    cargo check
    
    # Create tag and push
    create_and_push_tag "$new_version"
    
    echo ""
    print_success "Release $new_version completed successfully!"
    print_status "GitHub Actions will now build and publish the release."
    print_status "Check progress at: https://github.com/nwiizo/ccat/actions"
}

# Check if script is being sourced or executed
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi