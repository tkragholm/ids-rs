#!/bin/bash
set -e

# Configuration - CHANGE THESE VALUES
NEW_PACKAGE_NAME="ids-toolkit"  # Your desired PyPI package name
NEW_MODULE_NAME="ids_toolkit"   # Python module name (usually package name with underscores)

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMP_DIR="$SCRIPT_DIR/publication_temp"

# Update configuration in scripts
sed -i '' "s/NEW_PACKAGE_NAME=\".*\"/NEW_PACKAGE_NAME=\"$NEW_PACKAGE_NAME\"/" "$TEMP_DIR/republish.sh" "$TEMP_DIR/update_readme.sh"
sed -i '' "s/NEW_MODULE_NAME=\".*\"/NEW_MODULE_NAME=\"$NEW_MODULE_NAME\"/" "$TEMP_DIR/rename_module.sh" "$TEMP_DIR/republish.sh"

echo "===== PUBLISHING NEW PACKAGE: $NEW_PACKAGE_NAME ====="
echo "Module name will be: $NEW_MODULE_NAME"
echo

echo "Step 1: Updating README.md..."
"$TEMP_DIR/update_readme.sh"
echo

echo "Step 2: Renaming Python module..."
"$TEMP_DIR/rename_module.sh"
echo

echo "Step 3: Updating package configuration and building..."
"$TEMP_DIR/republish.sh"
echo

echo "===== PACKAGE PREPARATION COMPLETE ====="
echo
echo "To publish to PyPI, run:"
echo "cd $SCRIPT_DIR && $SCRIPT_DIR/test_install/venv/bin/python -m twine upload target/wheels/ids_toolkit-*"
echo
echo "You may need to authenticate with your PyPI credentials."
echo "If you haven't set up PyPI credentials yet, you can create a .pypirc file or use the prompts."
echo
echo "To test the package locally first:"
echo "$SCRIPT_DIR/test_install/venv/bin/pip install --force-reinstall $SCRIPT_DIR/target/wheels/ids_toolkit-*.whl"