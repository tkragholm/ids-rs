#!/bin/bash
# Script to bump version across the project and create a release tag

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install it first."
    exit 1
fi

# Get the current version from Cargo.toml
CURRENT_VERSION=$(grep 'version = "[0-9.]*"' Cargo.toml | head -1 | sed 's/.*version = "\([0-9.]*\)".*/\1/')

# Default to patch version bump if no argument is provided
if [ -z "$1" ]; then
    echo "No version bump type specified. Using 'patch'."
    BUMP_TYPE="patch"
else
    BUMP_TYPE="$1"
fi

# Function to bump semantic version
bump_version() {
    local version=$1
    local type=$2
    
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
        *)
            echo "Invalid bump type. Use 'major', 'minor', or 'patch'."
            exit 1
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Calculate new version
NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$BUMP_TYPE")

echo "Bumping version from $CURRENT_VERSION to $NEW_VERSION"

# Update workspace version in Cargo.toml
sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Python packages versions
find . -name "pyproject.toml" -exec sed -i '' "s/version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" {} \;

# Update version in any __init__.py files
find . -name "__init__.py" -exec sed -i '' "s/__version__ = \"$CURRENT_VERSION\"/__version__ = \"$NEW_VERSION\"/" {} \;

# Run cargo update to update Cargo.lock with new version
cargo update

echo "Version bump complete. Files updated:"
grep -r "version.*$NEW_VERSION" --include="*.toml" .

# Commit the version bump
echo "Creating commit for version $NEW_VERSION"
git add Cargo.toml Cargo.lock $(find . -name "pyproject.toml") $(find . -name "__init__.py")
git commit -m "Bump version to $NEW_VERSION"

# Create a tag
echo "Creating tag v$NEW_VERSION"
git tag -a "v$NEW_VERSION" -m "Release version $NEW_VERSION"

echo "Version $NEW_VERSION is ready."
echo ""
echo "To push the changes and trigger a release, run:"
echo "  git push && git push origin v$NEW_VERSION"
echo ""
echo "Or run this command now? (y/n)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    git push && git push origin "v$NEW_VERSION"
    echo "Changes pushed! GitHub Actions workflow should start building the release."
else
    echo "Commit and tag created but not pushed. Push manually when ready."
fi