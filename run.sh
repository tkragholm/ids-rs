#!/usr/bin/env bash

# Population command
./target/release/ids-rs population --bef ../../generated_data/parquet/bef --mfr ../../generated_data/parquet/mfr --output ../../TEST_OUT

# SCD command
./target/release/ids-rs population-scd --population ../../TEST_OUT/population.parquet --lpr ../../generated_data/parquet/ --output ../../TEST_OUT/

# Full pipeline command
./target/release/ids-rs study-design --bef ../../generated_data/parquet/bef --mfr ../../generated_data/parquet/mfr --lpr ../../generated_data/parquet/ --output ../../TEST_OUT -vvv
