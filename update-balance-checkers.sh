#!/bin/bash
# Script to update BalanceChecker.get_covariate calls to BalanceChecker.covariate

set -euo pipefail

echo "Updating BalanceChecker.get_covariate calls"

# Find all Rust files in the covariates directory
rust_files=$(find ./crates/covariates -name "*.rs")

# Replace get_covariate with covariate
for file in $rust_files; do
  if grep -q ".get_covariate(" "$file"; then
    echo "  - Updating $file"
    gsed -i "s/.get_covariate(/.covariate(/g" "$file"
  fi
done

echo "Update complete!"