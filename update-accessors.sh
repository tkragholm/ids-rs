#!/bin/bash
# Script to update all accessor method calls from the old get_* prefix to the new names

set -euo pipefail

REPO_ROOT="$(pwd)"
echo "Updating accessor method calls in $REPO_ROOT"

# The data_store.rs, time_varying_backend.rs, and values.rs files already have the specialized implementation
# with both old and new methods, so we'll skip them
EXCLUDE_PATTERN="models/covariate/values.rs\|store/data_store.rs\|store/time_varying_backend.rs"

# Find all Rust files excluding the ones we want to skip
rust_files=$(find . -name "*.rs" -not -path "*/target/*" | grep -v "$EXCLUDE_PATTERN")

# Define the replacements as separate variables
replace_type() {
  echo "Replacing .get_type() with .type_()"
  for file in $rust_files; do
    if grep -q ".get_type(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_type(/.type_(/g" "$file"
    fi
  done
}

replace_family_size() {
  echo "Replacing .get_family_size() with .family_size()"
  for file in $rust_files; do
    if grep -q ".get_family_size(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_family_size(/.family_size(/g" "$file"
    fi
  done
}

replace_municipality() {
  echo "Replacing .get_municipality() with .municipality()"
  for file in $rust_files; do
    if grep -q ".get_municipality(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_municipality(/.municipality(/g" "$file"
    fi
  done
}

replace_family_type() {
  echo "Replacing .get_family_type() with .family_type()"
  for file in $rust_files; do
    if grep -q ".get_family_type(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_family_type(/.family_type(/g" "$file"
    fi
  done
}

replace_civil_status() {
  echo "Replacing .get_civil_status() with .civil_status()"
  for file in $rust_files; do
    if grep -q ".get_civil_status(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_civil_status(/.civil_status(/g" "$file"
    fi
  done
}

replace_gender() {
  echo "Replacing .get_gender() with .gender()"
  for file in $rust_files; do
    if grep -q ".get_gender(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_gender(/.gender(/g" "$file"
    fi
  done
}

replace_citizenship() {
  echo "Replacing .get_citizenship() with .citizenship()"
  for file in $rust_files; do
    if grep -q ".get_citizenship(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_citizenship(/.citizenship(/g" "$file"
    fi
  done
}

replace_age() {
  echo "Replacing .get_age() with .age()"
  for file in $rust_files; do
    if grep -q ".get_age(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_age(/.age(/g" "$file"
    fi
  done
}

replace_children_count() {
  echo "Replacing .get_children_count() with .children_count()"
  for file in $rust_files; do
    if grep -q ".get_children_count(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_children_count(/.children_count(/g" "$file"
    fi
  done
}

replace_income_amount() {
  echo "Replacing .get_income_amount() with .income_amount()"
  for file in $rust_files; do
    if grep -q ".get_income_amount(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_income_amount(/.income_amount(/g" "$file"
    fi
  done
}

replace_wage_income() {
  echo "Replacing .get_wage_income() with .wage_income()"
  for file in $rust_files; do
    if grep -q ".get_wage_income(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_wage_income(/.wage_income(/g" "$file"
    fi
  done
}

replace_employment_status() {
  echo "Replacing .get_employment_status() with .employment_status()"
  for file in $rust_files; do
    if grep -q ".get_employment_status(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_employment_status(/.employment_status(/g" "$file"
    fi
  done
}

replace_education_level() {
  echo "Replacing .get_education_level() with .education_level()"
  for file in $rust_files; do
    if grep -q ".get_education_level(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_education_level(/.education_level(/g" "$file"
    fi
  done
}

replace_isced_code() {
  echo "Replacing .get_isced_code() with .isced_code()"
  for file in $rust_files; do
    if grep -q ".get_isced_code(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_isced_code(/.isced_code(/g" "$file"
    fi
  done
}

replace_education_years() {
  echo "Replacing .get_education_years() with .education_years()"
  for file in $rust_files; do
    if grep -q ".get_education_years(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_education_years(/.education_years(/g" "$file"
    fi
  done
}

replace_occupation_code() {
  echo "Replacing .get_occupation_code() with .occupation_code()"
  for file in $rust_files; do
    if grep -q ".get_occupation_code(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_occupation_code(/.occupation_code(/g" "$file"
    fi
  done
}

replace_classification() {
  echo "Replacing .get_classification() with .classification()"
  for file in $rust_files; do
    if grep -q ".get_classification(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_classification(/.classification(/g" "$file"
    fi
  done
}

replace_socio() {
  echo "Replacing .get_socio() with .socio()"
  for file in $rust_files; do
    if grep -q ".get_socio(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_socio(/.socio(/g" "$file"
    fi
  done
}

replace_socio02() {
  echo "Replacing .get_socio02() with .socio02()"
  for file in $rust_files; do
    if grep -q ".get_socio02(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_socio02(/.socio02(/g" "$file"
    fi
  done
}

replace_pre_socio() {
  echo "Replacing .get_pre_socio() with .pre_socio()"
  for file in $rust_files; do
    if grep -q ".get_pre_socio(" "$file"; then
      echo "  - Updating $file"
      gsed -i "s/.get_pre_socio(/.pre_socio(/g" "$file"
    fi
  done
}

# Run all replacements
replace_type
replace_family_size
replace_municipality
replace_family_type
replace_civil_status
replace_gender
replace_citizenship
replace_age
replace_children_count
replace_income_amount
replace_wage_income
replace_employment_status
replace_education_level
replace_isced_code
replace_education_years
replace_occupation_code
replace_classification
replace_socio
replace_socio02
replace_pre_socio

echo "Update complete!"