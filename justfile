# Define default variables
default_rust_log := "info"
default_matched_pairs := "output/matched_pairs.csv"
default_family_file := "data/registers/family.parquet"
default_akm_dir := "data/registers/akm/"
default_bef_dir := "data/registers/bef/"
default_ind_dir := "data/registers/ind/"
default_uddf_dir := "data/registers/uddf/"

# Default command
default:
    @just --list

# Build the release version
build-release:
    cargo build --release

# Run the check-balance command with default settings
check-balance: build-release
    RUST_LOG={{default_rust_log}} ./target/release/ids check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with custom paths
check-balance-custom matched_pairs family_file akm_dir bef_dir ind_dir uddf_dir log_level="debug": build-release
    RUST_LOG={{log_level}} ./target/release/ids check-balance \
        -m {{matched_pairs}} \
        --family-file {{family_file}} \
        --akm-dir {{akm_dir}} \
        --bef-dir {{bef_dir}} \
        --ind-dir {{ind_dir}} \
        --uddf-dir {{uddf_dir}}

# Run check-balance with verbose logging
check-balance-verbose: build-release
    RUST_LOG=trace ./target/release/ids check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}

# Run check-balance with minimal logging
check-balance-quiet: build-release
    RUST_LOG=info ./target/release/ids check-balance \
        -m {{default_matched_pairs}} \
        --family-file {{default_family_file}} \
        --akm-dir {{default_akm_dir}} \
        --bef-dir {{default_bef_dir}} \
        --ind-dir {{default_ind_dir}} \
        --uddf-dir {{default_uddf_dir}}
