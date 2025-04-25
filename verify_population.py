#!/usr/bin/env python3
"""
Verification script to confirm population data from Rust IDS toolkit.
Loads the BEF and MFR data and verifies the counts match the Rust implementation.
"""

import os
import pandas as pd
import pyarrow.parquet as pq
import pyarrow as pa
from datetime import datetime

# Paths to data (same as used in the Rust command)
BEF_PATH = "/home/tkragholm/generated_data/parquet/bef/"
MFR_PATH = "/home/tkragholm/generated_data/parquet/mfr/"
OUTPUT_PATH = "/home/tkragholm/TEST_OUT/population.parquet"

# Birth year range filter
START_YEAR = 1995
END_YEAR = 2018

def load_bef_data(path):
    """Load BEF data and apply basic filtering."""
    # Get list of parquet files in the directory
    bef_files = [os.path.join(path, f) for f in os.listdir(path) if f.endswith('.parquet')]
    
    if not bef_files:
        raise FileNotFoundError(f"No parquet files found in {path}")
    
    # Load data from all files
    dfs = []
    for file in bef_files:
        try:
            # Read with pyarrow to handle potential date type issues
            table = pq.read_table(file)
            df = table.to_pandas()
            dfs.append(df)
        except Exception as e:
            print(f"Error loading {file}: {e}")
    
    # Combine all dataframes
    if not dfs:
        raise ValueError("No valid BEF data loaded")
    
    bef_data = pd.concat(dfs, ignore_index=True)
    print(f"Loaded {len(bef_data)} rows from BEF data")
    
    # Column mapping based on Rust implementation
    column_mapping = {}
    
    # Check and rename columns if necessary
    if "PNR" not in bef_data.columns and any("PNR" in col.upper() for col in bef_data.columns):
        # Find column that contains PNR
        for col in bef_data.columns:
            if "PNR" in col.upper():
                column_mapping[col] = "PNR"
                break
    
    if "FOED_DAG" not in bef_data.columns and any("FOED" in col.upper() for col in bef_data.columns):
        # Find column that contains birth date
        for col in bef_data.columns:
            if "FOED" in col.upper() or "BIRTH" in col.upper() or "DAG" in col.upper():
                column_mapping[col] = "FOED_DAG"
                break
    
    if "FAR_ID" not in bef_data.columns and any("FAR" in col.upper() or "FATHER" in col.upper() for col in bef_data.columns):
        # Find column that contains father ID
        for col in bef_data.columns:
            if "FAR" in col.upper() or "FATHER" in col.upper():
                column_mapping[col] = "FAR_ID"
                break
    
    if "MOR_ID" not in bef_data.columns and any("MOR" in col.upper() or "MOTHER" in col.upper() for col in bef_data.columns):
        # Find column that contains mother ID
        for col in bef_data.columns:
            if "MOR" in col.upper() or "MOTHER" in col.upper():
                column_mapping[col] = "MOR_ID"
                break
    
    if "FAMILIE_ID" not in bef_data.columns and any("FAM" in col.upper() or "FAMILY" in col.upper() for col in bef_data.columns):
        # Find column that contains family ID
        for col in bef_data.columns:
            if "FAM" in col.upper() or "FAMILY" in col.upper():
                column_mapping[col] = "FAMILIE_ID"
                break
    
    # Rename columns
    if column_mapping:
        bef_data = bef_data.rename(columns=column_mapping)
    
    # Print column names for diagnostics
    print("BEF columns:", bef_data.columns.tolist())
    
    return bef_data

def load_mfr_data(path):
    """Load MFR data and apply basic filtering."""
    # Get list of parquet files in the directory
    mfr_files = [os.path.join(path, f) for f in os.listdir(path) if f.endswith('.parquet')]
    
    if not mfr_files:
        raise FileNotFoundError(f"No parquet files found in {path}")
    
    # Load data from all files
    dfs = []
    for file in mfr_files:
        try:
            # Read with pyarrow to handle potential date type issues
            table = pq.read_table(file)
            df = table.to_pandas()
            dfs.append(df)
        except Exception as e:
            print(f"Error loading {file}: {e}")
    
    # Combine all dataframes
    if not dfs:
        raise ValueError("No valid MFR data loaded")
    
    mfr_data = pd.concat(dfs, ignore_index=True)
    print(f"Loaded {len(mfr_data)} rows from MFR data")
    
    # Column mapping based on Rust implementation
    column_mapping = {}
    
    # Check and rename columns if necessary
    if "CPR_BARN" not in mfr_data.columns and any("CPR" in col.upper() and ("BARN" in col.upper() or "CHILD" in col.upper()) for col in mfr_data.columns):
        # Find column that contains child CPR
        for col in mfr_data.columns:
            if "CPR" in col.upper() and ("BARN" in col.upper() or "CHILD" in col.upper()):
                column_mapping[col] = "CPR_BARN"
                break
    
    if "FOEDSELSDATO" not in mfr_data.columns and any(("FOED" in col.upper() or "BIRTH" in col.upper()) for col in mfr_data.columns):
        # Find column that contains birth date
        for col in mfr_data.columns:
            if "FOED" in col.upper() or "BIRTH" in col.upper() or "DATO" in col.upper():
                column_mapping[col] = "FOEDSELSDATO"
                break
    
    if "CPR_FADER" not in mfr_data.columns and any("CPR" in col.upper() and ("FAD" in col.upper() or "FATHER" in col.upper()) for col in mfr_data.columns):
        # Find column that contains father CPR
        for col in mfr_data.columns:
            if "CPR" in col.upper() and ("FAD" in col.upper() or "FATHER" in col.upper()):
                column_mapping[col] = "CPR_FADER"
                break
    
    if "CPR_MODER" not in mfr_data.columns and any("CPR" in col.upper() and ("MOD" in col.upper() or "MOTHER" in col.upper()) for col in mfr_data.columns):
        # Find column that contains mother CPR
        for col in mfr_data.columns:
            if "CPR" in col.upper() and ("MOD" in col.upper() or "MOTHER" in col.upper()):
                column_mapping[col] = "CPR_MODER"
                break
    
    # Rename columns
    if column_mapping:
        mfr_data = mfr_data.rename(columns=column_mapping)
    
    # Print column names for diagnostics
    print("MFR columns:", mfr_data.columns.tolist())
    
    # Create standardized column names to match BEF
    mfr_data["PNR"] = mfr_data["CPR_BARN"]
    if "FOEDSELSDATO" in mfr_data.columns:
        mfr_data["FOED_DAG"] = mfr_data["FOEDSELSDATO"]
    mfr_data["FAR_ID"] = mfr_data["CPR_FADER"] if "CPR_FADER" in mfr_data.columns else None
    mfr_data["MOR_ID"] = mfr_data["CPR_MODER"] if "CPR_MODER" in mfr_data.columns else None
    mfr_data["FAMILIE_ID"] = None  # MFR doesn't have FAMILIE_ID
    
    return mfr_data

def filter_by_birth_year(df, date_column, start_year, end_year):
    """Filter dataframe by birth year range."""
    # Handle different date formats
    if date_column in df.columns:
        # Check column type and convert if needed
        if pd.api.types.is_datetime64_dtype(df[date_column]):
            # Already datetime, just extract year
            mask = (df[date_column].dt.year >= start_year) & (df[date_column].dt.year <= end_year)
        else:
            # Try to convert to datetime
            try:
                dates = pd.to_datetime(df[date_column], errors='coerce')
                mask = (dates.dt.year >= start_year) & (dates.dt.year <= end_year)
            except:
                print(f"Warning: Couldn't convert {date_column} to dates. Skipping filter.")
                mask = pd.Series(True, index=df.index)
    else:
        print(f"Warning: {date_column} not found. Skipping filter.")
        mask = pd.Series(True, index=df.index)
    
    return df[mask]

def combine_data(bef_df, mfr_df):
    """Combine BEF and MFR data similar to the Rust implementation."""
    # Ensure we have standard columns in both dataframes
    standard_columns = ["PNR", "FOED_DAG", "FAR_ID", "MOR_ID", "FAMILIE_ID"]
    
    for col in standard_columns:
        if col not in bef_df.columns:
            bef_df[col] = None
        if col not in mfr_df.columns:
            mfr_df[col] = None
    
    # Count missing values in BEF
    bef_missing_father = bef_df["FAR_ID"].isna().sum()
    bef_missing_mother = bef_df["MOR_ID"].isna().sum()
    
    # Count missing values in MFR
    mfr_missing_father = mfr_df["FAR_ID"].isna().sum()
    mfr_missing_mother = mfr_df["MOR_ID"].isna().sum()
    
    # Get unique PNRs in each dataset
    bef_pnrs = set(bef_df["PNR"].dropna().unique())
    mfr_pnrs = set(mfr_df["PNR"].dropna().unique())
    
    # Count records only in one dataset
    records_only_in_bef = len(bef_pnrs - mfr_pnrs)
    records_only_in_mfr = len(mfr_pnrs - bef_pnrs)
    
    # Get all unique PNRs
    all_pnrs = bef_pnrs.union(mfr_pnrs)
    total_combined_records = len(all_pnrs)
    
    # Create combined dataframe
    # We'll do this by creating a new dataframe with all unique PNRs and merging data from both sources
    combined_df = pd.DataFrame({"PNR": list(all_pnrs)})
    
    # Merge BEF data
    combined_df = combined_df.merge(bef_df[standard_columns], on="PNR", how="left", suffixes=("", "_bef"))
    
    # Merge MFR data
    combined_df = combined_df.merge(mfr_df[standard_columns], on="PNR", how="left", suffixes=("", "_mfr"))
    
    # Combine data, preferring BEF if available
    for col in standard_columns[1:]:  # Skip PNR
        bef_col = col
        mfr_col = f"{col}_mfr"
        if mfr_col in combined_df.columns:
            # Fill missing values from BEF with values from MFR
            combined_df[col] = combined_df[bef_col].fillna(combined_df[mfr_col])
            # Drop the extra column
            combined_df = combined_df.drop(columns=[mfr_col])
    
    # Count missing values in combined data
    combined_missing_father = combined_df["FAR_ID"].isna().sum()
    combined_missing_mother = combined_df["MOR_ID"].isna().sum()
    
    # Print summary statistics
    print("\nSummary Statistics:")
    print(f"Total BEF records: {len(bef_df)}")
    print(f"Total MFR records: {len(mfr_df)}")
    print(f"BEF missing father: {bef_missing_father}")
    print(f"BEF missing mother: {bef_missing_mother}")
    print(f"MFR missing father: {mfr_missing_father}")
    print(f"MFR missing mother: {mfr_missing_mother}")
    print(f"Records only in BEF: {records_only_in_bef}")
    print(f"Records only in MFR: {records_only_in_mfr}")
    print(f"Total combined records: {total_combined_records}")
    print(f"Combined missing father: {combined_missing_father}")
    print(f"Combined missing mother: {combined_missing_mother}")
    
    return combined_df

def verify_output_file(output_path, expected_count):
    """Verify the output parquet file contains the expected number of records."""
    if not os.path.exists(output_path):
        print(f"Error: Output file {output_path} not found")
        return False
    
    try:
        # Read the file
        table = pq.read_table(output_path)
        df = table.to_pandas()
        
        # Check record count
        actual_count = len(df)
        if actual_count == expected_count:
            print(f"\nVerification successful: Output file has {actual_count} records as expected")
            return True
        else:
            print(f"\nVerification failed: Output file has {actual_count} records, expected {expected_count}")
            return False
    except Exception as e:
        print(f"Error verifying output file: {e}")
        return False

def main():
    """Main validation function."""
    print("Starting population data validation")
    
    # Load data
    try:
        bef_data = load_bef_data(BEF_PATH)
        mfr_data = load_mfr_data(MFR_PATH)
        
        # Filter by birth year
        bef_filtered = filter_by_birth_year(bef_data, "FOED_DAG", START_YEAR, END_YEAR)
        mfr_filtered = filter_by_birth_year(mfr_data, "FOED_DAG", START_YEAR, END_YEAR)
        
        print(f"BEF records after filtering: {len(bef_filtered)}")
        print(f"MFR records after filtering: {len(mfr_filtered)}")
        
        # Combine data
        combined_data = combine_data(bef_filtered, mfr_filtered)
        
        # Verify output
        verify_output_file(OUTPUT_PATH, len(combined_data))
        
    except Exception as e:
        print(f"Error during validation: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()