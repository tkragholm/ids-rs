#!/usr/bin/env bash

# Population command
./target/debug/ids-rs population --bef ../../generated_data/parquet/bef --mfr ../../generated_data/parquet/mfr --output ../../TEST_OUT -vvv

# SCD command
./target/debug/ids-rs population-scd --population ../../TEST_OUT/population.parquet --lpr ../../generated_data/parquet/ --output ../../TEST_OUT/ -vvv

# Full pipeline command
./target/debug/ids-rs study-design --bef ../../generated_data/parquet/bef --mfr ../../generated_data/parquet/mfr --lpr ../../generated_data/parquet/ --output ../../TEST_OUT -vvv
