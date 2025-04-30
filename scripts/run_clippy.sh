#!/bin/bash

# Script to run Clippy with various lint levels incrementally
# Each run will fix issues automatically and allow dirty working directories

show_usage() {
    echo "Usage: $0 [--start-step STEP_NUMBER]"
    echo ""
    echo "Options:"
    echo "  --start-step STEP_NUMBER  Start from a specific step (1-4)"
    echo "                          1: Default clippy checks"
    echo "                          2: All clippy checks"
    echo "                          3: Pedantic clippy checks"
    echo "                          4: Nursery clippy checks"
    echo ""
    echo "Example:"
    echo "  $0               # Run all steps from the beginning"
    echo "  $0 --start-step 3  # Start from step 3 (pedantic checks)"
    exit 1
}

# Show help if requested
if [ "$1" == "-h" ] || [ "$1" == "--help" ]; then
    show_usage
fi

set -e # Exit on first error

ECHO_PREFIX="\033[1;36m[Clippy]\033[0m"

# Process command line arguments
START_STEP=1
if [ "$1" == "--start-step" ] && [ -n "$2" ]; then
    START_STEP=$2
    if ! [[ "$START_STEP" =~ ^[1-4]$ ]]; then
        echo "Error: Start step must be a number between 1 and 4"
        exit 1
    fi
fi

# Function to echo with prefix
echo_step() {
    echo -e "$ECHO_PREFIX $1"
}

# Function to run clippy with specified arguments
run_clippy() {
    local lint_level=$1
    local description=$2
    
    echo_step "Running clippy with $description..."
    cargo clippy --fix --allow-dirty -- $lint_level
    
    # Check if the previous command was successful
    if [ $? -eq 0 ]; then
        echo_step "✅ Successfully fixed issues for $description"
    else
        echo_step "⚠️ Some issues could not be automatically fixed for $description"
    fi
    
    echo ""
}

# Main execution
echo_step "Starting incremental Clippy checks with fixes from step $START_STEP"
echo ""

# Step 1: Run with default settings
if [ $START_STEP -le 1 ]; then
    echo_step "STEP 1/4: Default clippy checks"
    run_clippy "" "default settings"
else
    echo_step "Skipping step 1 (default clippy checks)"
fi

# Step 2: Run with --all
if [ $START_STEP -le 2 ]; then
    echo_step "STEP 2/4: All clippy checks"
    run_clippy "-W clippy::all" "all lints"
else
    echo_step "Skipping step 2 (all clippy checks)"
fi

# Step 3: Run with --pedantic
if [ $START_STEP -le 3 ]; then
    echo_step "STEP 3/4: Pedantic clippy checks"
    run_clippy "-W clippy::all -W clippy::pedantic" "pedantic lints"
else
    echo_step "Skipping step 3 (pedantic clippy checks)"
fi

# Step 4: Run with --nursery (optional)
if [ $START_STEP -le 4 ]; then
    echo_step "STEP 4/4: Nursery clippy checks (optional)"
    read -p "Do you want to run with nursery lints? (y/n) " run_nursery
    if [[ $run_nursery == "y" || $run_nursery == "Y" ]]; then
        run_clippy "-W clippy::all -W clippy::pedantic -W clippy::nursery" "nursery lints"
    else
        echo_step "Skipping nursery lints"
    fi
else
    echo_step "Skipping step 4 (nursery clippy checks)"
fi

echo_step "All Clippy checks completed!"