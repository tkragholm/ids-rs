#!/bin/bash
set -e

# Configuration
NEW_MODULE_NAME="ids_toolkit"  # Change this to your desired module name
CURRENT_DIR=$(pwd)
REPO_ROOT="$CURRENT_DIR"
IDS_CRATE_DIR="$REPO_ROOT/crates/ids"
OLD_MODULE_PATH="$IDS_CRATE_DIR/python/ids_tk"
NEW_MODULE_PATH="$IDS_CRATE_DIR/python/$NEW_MODULE_NAME"

echo "Renaming Python module from ids_tk to $NEW_MODULE_NAME..."

# Create new directory
mkdir -p "$NEW_MODULE_PATH"

# Copy all files from old to new directory
cp -r "$OLD_MODULE_PATH"/* "$NEW_MODULE_PATH/"

# Keep the old directory for now (to avoid build issues)
# We'll update the configuration to use the new directory

echo "Module renamed successfully!"
echo "Remember to run the republish.sh script after this."