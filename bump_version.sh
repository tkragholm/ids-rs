#!/bin/bash
# =========================================================================
# bump_version.sh - Comprehensive Version Management Script for ids-rs
# =========================================================================
# This script handles version bumping across the entire project, including:
# - Cargo.toml workspace and member crates
# - pyproject.toml files for Python packages
# - __init__.py files with version declarations
# - Creating Git tags and commits
#
# It supports major/minor/patch increments or direct version specification
# and ensures consistent versioning across the entire project.

set -eo pipefail # Exit on error, pipe failures

# =========================================================================
# Configuration and utilities
# =========================================================================
# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
log_info() { echo -e "${BLUE}INFO:${NC} $1"; }
log_success() { echo -e "${GREEN}SUCCESS:${NC} $1"; }
log_warning() { echo -e "${YELLOW}WARNING:${NC} $1"; }
log_error() { echo -e "${RED}ERROR:${NC} $1"; }
log_step() { echo -e "\n${CYAN}⚡ $1${NC}"; }

# Detect OS for sed compatibility
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS requires an empty string after -i
    SED_INPLACE="sed -i ''"
else
    # Linux and others don't
    SED_INPLACE="sed -i"
fi

# =========================================================================
# Dependency checks
# =========================================================================
log_step "Checking dependencies..."

# Check if jq is installed
if ! command -v jq &>/dev/null; then
    log_error "jq is required but not installed. Please install it first."
    exit 1
fi

# Check for git
if ! command -v git &>/dev/null; then
    log_error "git is required but not installed. Please install it first."
    exit 1
fi

# =========================================================================
# Version detection and validation
# =========================================================================
log_step "Detecting current version..."

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep 'version = "[0-9.]*"' Cargo.toml | head -1 | sed 's/.*version = "\([0-9.]*\)".*/\1/')
if [ -z "$CURRENT_VERSION" ]; then
    log_error "Could not determine current version from Cargo.toml"
    exit 1
fi
log_info "Current version is: $CURRENT_VERSION"

# Parse command-line arguments
if [ -z "$1" ]; then
    log_warning "No version bump type specified. Using 'patch'."
    BUMP_TYPE="patch"
elif [ "$1" == "major" ] || [ "$1" == "minor" ] || [ "$1" == "patch" ]; then
    BUMP_TYPE="$1"
    log_info "Bumping $BUMP_TYPE version"
elif [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    # Direct version specification
    NEW_VERSION="$1"
    BUMP_TYPE="direct"
    log_info "Using directly specified version: $NEW_VERSION"
else
    log_error "Invalid bump type. Use 'major', 'minor', 'patch', or a specific version (e.g., '1.2.3')."
    echo -e "\nUsage: $0 [major|minor|patch|X.Y.Z]"
    echo -e "  major  - Increment major version (X.0.0)"
    echo -e "  minor  - Increment minor version (x.Y.0)"
    echo -e "  patch  - Increment patch version (x.y.Z)"
    echo -e "  X.Y.Z  - Set a specific version number"
    exit 1
fi

# Function to bump semantic version
bump_version() {
    local version=$1
    local type=$2

    if [ "$type" == "direct" ]; then
        echo "$NEW_VERSION"
        return
    fi

    IFS='.' read -r -a parts <<<"$version"
    local major="${parts[0]}"
    local minor="${parts[1]}"
    local patch="${parts[2]}"

    case "$type" in
    major)
        major=$((major + 1))
        minor=0
        patch=0
        ;;
    minor)
        minor=$((minor + 1))
        patch=0
        ;;
    patch)
        patch=$((patch + 1))
        ;;
    esac

    echo "$major.$minor.$patch"
}

# Calculate new version (if not directly specified)
if [ "$BUMP_TYPE" != "direct" ]; then
    NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$BUMP_TYPE")
fi

log_success "Will bump version from $CURRENT_VERSION to $NEW_VERSION"

# Ask for confirmation before proceeding
echo ""
read -p "Continue with version update? (y/n): " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log_info "Version update cancelled."
    exit 0
fi

# =========================================================================
# Update version in all files
# =========================================================================
log_step "Updating version in project files..."

# Update workspace version in Cargo.toml
$SED_INPLACE "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
log_info "Updated Cargo.toml workspace version"

# Update all Cargo.toml files in member crates
find ./crates -name "Cargo.toml" -not -path "*/\.*" -not -path "*/target/*" | while read -r cargo_file; do
    if grep -q "^\[package\]" "$cargo_file"; then
        # Only update version in actual package Cargo.toml files, not workspace files
        $SED_INPLACE "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$cargo_file"
        log_info "Updated version in $cargo_file"
    fi
done

# Update Python packages versions in all pyproject.toml files (excluding venv directories)
find . -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" | while read -r pyproject_file; do
    $SED_INPLACE "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$pyproject_file"
    $SED_INPLACE "s/version = \"[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\"/version = \"$NEW_VERSION\"/" "$pyproject_file"
    log_info "Updated version in $pyproject_file"
done

# Update version in any project __init__.py files (excluding venv directories)
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" | while read -r init_file; do
    $SED_INPLACE "s/__version__ = \"$CURRENT_VERSION\"/__version__ = \"$NEW_VERSION\"/" "$init_file"
    $SED_INPLACE "s/__version__ = \"[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\"/__version__ = \"$NEW_VERSION\"/" "$init_file"
    log_info "Updated version in $init_file"
done

# Run cargo update to update Cargo.lock with new version
log_step "Updating Cargo.lock..."
cargo update
log_success "Cargo.lock updated"

# =========================================================================
# Verify updates
# =========================================================================
log_step "Verifying updates..."

# Verify Cargo.toml update
if grep -q "version = \"$NEW_VERSION\"" Cargo.toml; then
    log_success "Cargo.toml correctly updated to $NEW_VERSION"
else
    log_error "Failed to update version in Cargo.toml"
    exit 1
fi

# Show summary of updated files
echo -e "\n${CYAN}Updated Files Summary:${NC}"
echo -e "${BLUE}Cargo.toml:${NC}"
grep -n "version" Cargo.toml | head -n 5
echo -e "\n${BLUE}pyproject.toml files:${NC}"
find ./crates -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" -exec grep -n "version" {} \;
echo -e "\n${BLUE}__init__.py files:${NC}"
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" -exec grep -n "__version__" {} \;

# =========================================================================
# Git operations
# =========================================================================
log_step "Performing Git operations..."

# Check if we're in a clean git state
if ! git diff --quiet; then
    log_warning "You have uncommitted changes in your working directory."
    read -p "Continue anyway? (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Version update cancelled. Please commit or stash your changes first."
        exit 0
    fi
fi

# Delete any existing tags with the same version to avoid conflicts
if git tag | grep -q "v$NEW_VERSION"; then
    log_warning "Tag v$NEW_VERSION already exists. Deleting..."
    git tag -d "v$NEW_VERSION"

    if git ls-remote --tags origin | grep -q "refs/tags/v$NEW_VERSION"; then
        log_warning "Remote tag v$NEW_VERSION exists. Will be overwritten on push."
    fi
fi

# Add files for the version bump
log_info "Adding files for version $NEW_VERSION"
git add Cargo.toml Cargo.lock

# Add all Cargo.toml files in member crates
find ./crates -name "Cargo.toml" -not -path "*/\.*" -not -path "*/target/*" | while read -r file; do
    if [ -f "$file" ]; then
        git add "$file"
    fi
done

# Add pyproject.toml files
find ./crates -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" | while read -r file; do
    if [ -f "$file" ]; then
        git add "$file"
    fi
done

# Add __init__.py files
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -not -path "*/target/*" | while read -r file; do
    if [ -f "$file" ]; then
        git add "$file"
    fi
done

# Check if there are changes to commit
CHANGES_COMMITTED=false
if git diff --cached --quiet; then
    log_warning "No changes to commit. Version might already be up to date."
else
    # Commit the version bump
    log_info "Creating commit for version $NEW_VERSION"
    git commit -m "Bump version to $NEW_VERSION"
    CHANGES_COMMITTED=true
fi

# Create a tag if we had changes or if the tag doesn't exist yet
if $CHANGES_COMMITTED || ! git tag | grep -q "v$NEW_VERSION"; then
    log_info "Creating tag v$NEW_VERSION"
    git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"
else
    log_warning "No changes to commit and tag v$NEW_VERSION already exists. Skipping tag creation."
fi

log_success "Version $NEW_VERSION is ready."

# =========================================================================
# Push changes
# =========================================================================
if $CHANGES_COMMITTED || ! git tag | grep -q "v$NEW_VERSION"; then
    echo ""
    log_step "Final Steps"
    echo -e "${BLUE}To push the changes and trigger a release, run:${NC}"
    echo "  git push && git push origin v$NEW_VERSION"
    echo ""
    read -p "Push changes now? (y/n): " -r
    if [[ "$REPLY" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        # First push the commit
        if git push; then
            log_info "Commit pushed successfully"
        else
            log_error "Failed to push commit"
            exit 1
        fi

        # Then push the tag
        if git push origin "v$NEW_VERSION"; then
            log_success "Tag pushed successfully"
        else
            log_error "Failed to push tag"
            exit 1
        fi

        log_success "Changes pushed! GitHub Actions workflow should start building the release."
    else
        log_warning "Commit and/or tag created but not pushed. Push manually when ready."
    fi
else
    log_warning "No changes were made. Nothing to push."
fi

log_step "Version bump complete!"
echo "Current version: $CURRENT_VERSION → New version: $NEW_VERSION"
