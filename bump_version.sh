#!/bin/bash
# Enhanced script to bump version across the project and create a release tag
# Ensures synchronization between all versioned files and git tags

set -e  # Exit on error

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install it first."
    exit 1
fi

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep 'version = "[0-9.]*"' Cargo.toml | head -1 | sed 's/.*version = "\([0-9.]*\)".*/\1/')
if [ -z "$CURRENT_VERSION" ]; then
    echo -e "${RED}Error: Could not determine current version from Cargo.toml${NC}"
    exit 1
fi

# Default to patch version bump if no argument is provided
if [ -z "$1" ]; then
    echo -e "${YELLOW}No version bump type specified. Using 'patch'.${NC}"
    BUMP_TYPE="patch"
elif [ "$1" == "major" ] || [ "$1" == "minor" ] || [ "$1" == "patch" ]; then
    BUMP_TYPE="$1"
elif [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    # Direct version specification
    NEW_VERSION="$1"
    BUMP_TYPE="direct"
    echo -e "${BLUE}Using directly specified version: $NEW_VERSION${NC}"
else
    echo -e "${RED}Invalid bump type. Use 'major', 'minor', 'patch', or a specific version (e.g., '1.2.3').${NC}"
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
    
    IFS='.' read -r -a parts <<< "$version"
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

echo -e "${GREEN}Bumping version from $CURRENT_VERSION to $NEW_VERSION${NC}"

echo -e "${BLUE}Updating version in all project files...${NC}"

# Update workspace version in Cargo.toml
sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Python packages versions in all pyproject.toml files (excluding venv directories)
find . -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" {} \;
find . -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec sed -i '' "s/version = \".*\"/version = \"$NEW_VERSION\"/" {} \;

# Update version in any project __init__.py files (excluding venv directories)
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec sed -i '' "s/__version__ = \"$CURRENT_VERSION\"/__version__ = \"$NEW_VERSION\"/" {} \;
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec sed -i '' "s/__version__ = \".*\"/__version__ = \"$NEW_VERSION\"/" {} \;

# Run cargo update to update Cargo.lock with new version
echo -e "${BLUE}Updating Cargo.lock...${NC}"
cargo update

# Verify all files were updated correctly
echo -e "${GREEN}Version bump complete. Files updated:${NC}"
echo -e "${BLUE}Cargo.toml:${NC}"
grep "version" Cargo.toml | head -n 5
echo -e "${BLUE}pyproject.toml files:${NC}"
find ./crates -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec grep "version" {} \;
echo -e "${BLUE}__init__.py files:${NC}"
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" -exec grep "__version__" {} \;

# Delete any existing tags with the same version to avoid conflicts
if git tag | grep -q "v$NEW_VERSION"; then
    echo -e "${YELLOW}Tag v$NEW_VERSION already exists. Deleting...${NC}"
    git tag -d "v$NEW_VERSION"
    
    if git ls-remote --tags origin | grep -q "refs/tags/v$NEW_VERSION"; then
        echo -e "${YELLOW}Remote tag v$NEW_VERSION exists. Will be overwritten on push.${NC}"
    fi
fi

# Add files for the version bump
echo -e "${BLUE}Adding files for version $NEW_VERSION${NC}"
git add Cargo.toml Cargo.lock 

# Add pyproject.toml files
find ./crates -name "pyproject.toml" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" | while read file; do
    if [ -f "$file" ]; then
        git add "$file"
    fi
done

# Add __init__.py files
find ./crates -name "__init__.py" -not -path "*/\.*" -not -path "*/venv/*" -not -path "*/.venv/*" | while read file; do
    if [ -f "$file" ]; then
        git add "$file"
    fi
done

# Check if there are changes to commit
CHANGES_COMMITTED=false
if git diff --cached --quiet; then
    echo -e "${YELLOW}No changes to commit. Version might already be up to date.${NC}"
else
    # Commit the version bump
    echo -e "${BLUE}Creating commit for version $NEW_VERSION${NC}"
    git commit -m "Bump version to $NEW_VERSION"
    CHANGES_COMMITTED=true
fi

# Create a tag if we had changes or if the tag doesn't exist yet
if $CHANGES_COMMITTED || ! git tag | grep -q "v$NEW_VERSION"; then
    echo -e "${BLUE}Creating tag v$NEW_VERSION${NC}"
    git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"
else
    echo -e "${YELLOW}No changes to commit and tag v$NEW_VERSION already exists. Skipping tag creation.${NC}"
fi

echo -e "${GREEN}Version $NEW_VERSION is ready.${NC}"

if $CHANGES_COMMITTED || ! git tag | grep -q "v$NEW_VERSION"; then
    echo ""
    echo -e "${BLUE}To push the changes and trigger a release, run:${NC}"
    echo "  git push && git push origin v$NEW_VERSION"
    echo ""
    echo -e "${YELLOW}Or run this command now? (y/n)${NC}"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        git push && git push origin "v$NEW_VERSION"
        echo -e "${GREEN}Changes pushed! GitHub Actions workflow should start building the release.${NC}"
    else
        echo -e "${YELLOW}Commit and/or tag created but not pushed. Push manually when ready.${NC}"
    fi
else
    echo -e "${YELLOW}No changes were made. Nothing to push.${NC}"
fi