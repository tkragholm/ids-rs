#!/bin/bash
set -e

# Configuration
NEW_PACKAGE_NAME="ids-toolkit"  # Change this to your desired name
PYTHON_MODULE_NAME="ids_toolkit"  # Python module name (usually package name with underscore)
CURRENT_DIR=$(pwd)
REPO_ROOT="$CURRENT_DIR"
IDS_CRATE_DIR="$REPO_ROOT/crates/ids"

echo "Preparing to republish package as $NEW_PACKAGE_NAME..."

# 1. Update pyproject.toml
echo "Updating pyproject.toml..."
cat > "$IDS_CRATE_DIR/pyproject.toml" << EOF
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "$NEW_PACKAGE_NAME"
version = "1.2.1"
description = "Incidence Density Sampling CLI Tool"
authors = [
    {name = "Tobias Kragholm", email = "tkragholm@gmail.com"}
]
readme = "README.md"
requires-python = ">=3.7"
license = {text = "MIT"}
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.7",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Environment :: Console",
    "Intended Audience :: Science/Research",
]

[tool.maturin]
# Configure maturin to build and package the binary
bindings = "bin"  # We're creating a binary package
module-name = "$PYTHON_MODULE_NAME"
manifest-path = "Cargo.toml"
python-source = "python"  # Directory with Python code
EOF

# 2. Update version in __init__.py
echo "Updating version in __init__.py..."
VERSION=$(grep 'version = ' $REPO_ROOT/Cargo.toml | head -1 | cut -d '"' -f 2)
sed -i '' "s/__version__ = \".*\"/__version__ = \"$VERSION\"/" "$IDS_CRATE_DIR/python/ids_tk/__init__.py"

# 3. Build the package
echo "Building package with maturin..."
cd "$IDS_CRATE_DIR"

# Use the project's virtual environment
VENV_PATH="$REPO_ROOT/test_install/venv"
PYTHON_PATH="$VENV_PATH/bin/python"
PIP_PATH="$VENV_PATH/bin/pip"

echo "Using Python from virtual environment: $PYTHON_PATH"
$PIP_PATH install -U maturin twine

# Build for current platform
source "$VENV_PATH/bin/activate"
maturin build --release
deactivate

echo ""
echo "Package built successfully!"
echo ""
echo "To publish to PyPI, run the following command:"
echo "cd $IDS_CRATE_DIR && python -m twine upload target/wheels/*"
echo ""
echo "To install locally for testing:"
echo "pip install --force-reinstall $IDS_CRATE_DIR/target/wheels/*.whl"