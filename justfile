# Define default variables
ids_binary := "target/release/ids"  # Changed from release to debug
default_rust_log := "info"
default_matched_pairs := "output/matched_pairs.csv"
default_family_file := "data/registers/family.parquet"
default_akm_dir := "data/registers/akm/"
default_bef_dir := "data/registers/bef/"
default_ind_dir := "data/registers/ind/"
default_uddf_dir := "data/registers/uddf/"

# Data generation parameters
default_output_dir := "data/registers"
default_start_year := "2000"
default_end_year := "2023"
default_year_range := default_start_year + "-" + default_end_year

# Default command
default:
    @just --list

# Build the release version (now the primary build target)
build:
    cargo build --release

# Build the debug version (kept for when needed)
build-debug:
    cargo build

# Run the check-balance command with default settings
check-balance: build
    RUST_LOG={{default_rust_log}} {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with custom paths
check-balance-custom matched_pairs family_file akm_dir bef_dir ind_dir uddf_dir log_level="debug": build
    RUST_LOG={{log_level}} {{ids_binary}} check-balance \
        -m {{matched_pairs}} \
        --family-file {{family_file}} \
        --akm-dir {{akm_dir}} \
        --bef-dir {{bef_dir}} \
        --ind-dir {{ind_dir}} \
        --uddf-dir {{uddf_dir}}

# Run check-balance with verbose logging
check-balance-verbose: build
    RUST_LOG=trace {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with minimal logging
check-balance-quiet: build
    RUST_LOG=info {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with LRU cache for better memory management
check-balance-lru: build
    RUST_LOG=info IDS_CACHE_TYPE=lru {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with adaptive cache
check-balance-adaptive: build
    RUST_LOG=info IDS_CACHE_TYPE=adaptive {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with legacy cache
check-balance-legacy: build
    RUST_LOG=info IDS_CACHE_TYPE=legacy {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}
        
# Run check-balance with LRU cache but skip matched pairs processing (for debugging)
check-balance-lru-skip-pairs: build
    RUST_LOG=info IDS_CACHE_TYPE=lru IDS_SKIP_PAIRS=true {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with adaptive cache but skip matched pairs processing (for debugging)
check-balance-adaptive-skip-pairs: build
    RUST_LOG=info IDS_CACHE_TYPE=adaptive IDS_SKIP_PAIRS=true {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with legacy cache but skip matched pairs processing (for debugging)
check-balance-legacy-skip-pairs: build
    RUST_LOG=info IDS_CACHE_TYPE=legacy IDS_SKIP_PAIRS=true {{ids_binary}} check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Generate synthetic registers with standard size
generate-registers: build
    RUST_LOG=info {{ids_binary}} generate-registers \
        -o {{default_output_dir}} \
        -t 1000000 \
        -c 50000 \
        -s {{default_start_year}} \
        -e {{default_end_year}}

# Generate synthetic registers with small size (useful for testing)
generate-registers-small: build
    RUST_LOG=info {{ids_binary}} generate-registers \
        -o {{default_output_dir}}_small \
        -t 100000 \
        -c 5000 \
        -s {{default_start_year}} \
        -e {{default_end_year}}

# Generate synthetic registers with tiny size (for quick tests)
generate-registers-tiny: build
    RUST_LOG=info {{ids_binary}} generate-registers \
        -o {{default_output_dir}}_tiny \
        -t 10000 \
        -c 500 \
        -s {{default_start_year}} \
        -e {{default_end_year}}

# Generate synthetic registers with a custom size
generate-registers-custom total_records cases output_dir="data/registers_custom" start_year="2000" end_year="2023" seed="": build
    RUST_LOG=info {{ids_binary}} generate-registers \
        -o {{output_dir}} \
        -t {{total_records}} \
        -c {{cases}} \
        -s {{start_year}} \
        -e {{end_year}}

# Generate registers and run sampling in one command (standard size)
generate-and-sample: generate-registers
    RUST_LOG=info {{ids_binary}} sample \
        -i {{default_output_dir}}/pediatric.csv \
        -n 4 \
        -b 30 \
        -p 365

# Generate registers and run sampling in one command (small size)
generate-and-sample-small: generate-registers-small
    RUST_LOG=info {{ids_binary}} sample \
        -i {{default_output_dir}}_small/pediatric.csv \
        -n 4 \
        -b 30 \
        -p 365

# Generate registers and run sampling in one command (tiny size)
generate-and-sample-tiny: generate-registers-tiny
    RUST_LOG=info {{ids_binary}} sample \
        -i {{default_output_dir}}_tiny/pediatric.csv \
        -n 4 \
        -b 30 \
        -p 365

# Run the complete pipeline (generate, sample, and check balance) with tiny datasets
complete-pipeline-tiny: generate-and-sample-tiny
    RUST_LOG=warn {{ids_binary}} check-balance \
        -m output/matched_pairs.csv \
        --family-file {{default_output_dir}}_tiny/family.parquet \
        --akm-dir {{default_output_dir}}_tiny/akm/ \
        --bef-dir {{default_output_dir}}_tiny/bef/ \
        --ind-dir {{default_output_dir}}_tiny/ind/ \
        --uddf-dir {{default_output_dir}}_tiny/uddf/

# Run the complete pipeline with small datasets
complete-pipeline-small: generate-and-sample-small
    RUST_LOG=warn {{ids_binary}} check-balance \
        -m output/matched_pairs.csv \
        --family-file {{default_output_dir}}_small/family.parquet \
        --akm-dir {{default_output_dir}}_small/akm/ \
        --bef-dir {{default_output_dir}}_small/bef/ \
        --ind-dir {{default_output_dir}}_small/ind/ \
        --uddf-dir {{default_output_dir}}_small/uddf/ \
        --structured

# Run the complete pipeline with small datasets using legacy cache (working version from TROUBLE.md)
complete-pipeline-small-legacy: generate-and-sample-small
    RUST_LOG=warn IDS_CACHE_TYPE=legacy {{ids_binary}} check-balance \
        -m output/matched_pairs.csv \
        --family-file {{default_output_dir}}_small/family.parquet \
        --akm-dir {{default_output_dir}}_small/akm/ \
        --bef-dir {{default_output_dir}}_small/bef/ \
        --ind-dir {{default_output_dir}}_small/ind/ \
        --uddf-dir {{default_output_dir}}_small/uddf/ \
        --structured

# Run the complete pipeline with small datasets using legacy cache but skip matched pairs
complete-pipeline-small-legacy-skip-pairs: generate-and-sample-small
    RUST_LOG=warn IDS_CACHE_TYPE=legacy IDS_SKIP_PAIRS=true {{ids_binary}} check-balance \
        -m output/matched_pairs.csv \
        --family-file {{default_output_dir}}_small/family.parquet \
        --akm-dir {{default_output_dir}}_small/akm/ \
        --bef-dir {{default_output_dir}}_small/bef/ \
        --ind-dir {{default_output_dir}}_small/ind/ \
        --uddf-dir {{default_output_dir}}_small/uddf/ \
        --structured

# Clean up generated data directories
clean-data:
    rm -rf {{default_output_dir}}_tiny
    rm -rf {{default_output_dir}}_small
    rm -rf data/registers_custom

# Reset output directory (for fresh runs)
clean-output:
    rm -rf output/*
