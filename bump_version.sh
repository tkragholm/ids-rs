#!/bin/bash
# Script to bump version across the project

OLD_VERSION="0.2.0"
NEW_VERSION="0.2.1"

echo "Bumping version from $OLD_VERSION to $NEW_VERSION"

# Update workspace version
sed -i '' "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update Python package version
sed -i '' "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" crates/py/pyproject.toml

# Run cargo update to update Cargo.lock with new version
cargo update -p py

echo "Version bump complete. Files updated:"
grep -r "version.*$NEW_VERSION" --include="*.toml" .