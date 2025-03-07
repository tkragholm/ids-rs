import os
import sys
import pyarrow.parquet as pq
import io
from pathlib import Path

def get_parquet_schema_info(file_path):
    """Get detailed schema information using ParquetFile API."""
    try:
        # Use ParquetFile for efficient metadata-only access
        parquet_file = pq.ParquetFile(file_path)

        # Extract file metadata
        metadata = parquet_file.metadata
        schema = parquet_file.schema_arrow  # Use schema_arrow to get the Arrow schema

        # Get basic statistics
        num_row_groups = metadata.num_row_groups
        num_rows = metadata.num_rows
        num_columns = metadata.num_columns
        created_by = metadata.created_by

        # Get column-level information by accessing schema fields
        columns_info = []
        schema_fields = schema.names

        for i, field_name in enumerate(schema_fields):
            field = schema.field(i)
            field_type = field.type

            # Try to get statistics from first row group if available
            stats = None
            if num_row_groups > 0:
                try:
                    col_meta = metadata.row_group(0).column(i)
                    if col_meta.is_stats_set:
                        stats = {
                            'min': str(col_meta.statistics.min) if hasattr(col_meta.statistics, 'min') else 'N/A',
                            'max': str(col_meta.statistics.max) if hasattr(col_meta.statistics, 'max') else 'N/A',
                            'null_count': col_meta.statistics.null_count if hasattr(col_meta.statistics, 'null_count') else 'N/A',
                            'distinct_count': col_meta.statistics.distinct_count if hasattr(col_meta.statistics, 'distinct_count') else 'N/A',
                        }
                except Exception:
                    # Skip statistics if there's any error
                    pass

            columns_info.append({
                'name': field_name,
                'type': str(field_type),
                'stats': stats
            })

        return {
            'schema': schema,
            'metadata': {
                'num_rows': num_rows,
                'num_columns': num_columns,
                'num_row_groups': num_row_groups,
                'created_by': created_by
            },
            'columns_info': columns_info
        }
    except Exception as e:
        return {'error': str(e)}

def find_parquet_files(directory):
    """Recursively find all parquet files in a directory."""
    parquet_files = []

    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.parquet'):
                parquet_files.append(os.path.join(root, file))

    return parquet_files

def write_to_file(output_file, content):
    """Write content to file with UTF-8 encoding."""
    with open(output_file, 'a', encoding='utf-8') as f:
        f.write(content + '\n')

def scan_and_save_schemas(directory, output_file):
    """Scan directory recursively and save schema for each parquet file."""
    # Clear the output file if it exists
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("")

    parquet_files = find_parquet_files(directory)

    if not parquet_files:
        write_to_file(output_file, f"No parquet files found in {directory}")
        print(f"No parquet files found in {directory}")
        return

    write_to_file(output_file, f"Found {len(parquet_files)} parquet files\n")
    print(f"Found {len(parquet_files)} parquet files. Writing to {output_file}")

    for i, file_path in enumerate(parquet_files, 1):
        print(f"Processing [{i}/{len(parquet_files)}] {file_path}")

        file_info = f"[{i}/{len(parquet_files)}] {file_path}"
        write_to_file(output_file, file_info)

        schema_info = get_parquet_schema_info(file_path)

        if 'error' in schema_info:
            write_to_file(output_file, f"  Error: {schema_info['error']}")
            continue

        # Write metadata
        metadata = schema_info['metadata']
        write_to_file(output_file, f"  Created by: {metadata['created_by']}")
        write_to_file(output_file, f"  Rows: {metadata['num_rows']}")
        write_to_file(output_file, f"  Columns: {metadata['num_columns']}")
        write_to_file(output_file, f"  Row groups: {metadata['num_row_groups']}")

        write_to_file(output_file, "  Schema:")
        write_to_file(output_file, f"    {schema_info['schema']}")

        # Write column details with statistics if available
        write_to_file(output_file, "  Column Details:")
        for col in schema_info['columns_info']:
            write_to_file(output_file, f"    {col['name']} ({col['type']})")
            if col['stats']:
                stats = col['stats']
                write_to_file(output_file, f"      min: {stats['min']}")
                write_to_file(output_file, f"      max: {stats['max']}")
                write_to_file(output_file, f"      null_count: {stats['null_count']}")
                write_to_file(output_file, f"      distinct_count: {stats['distinct_count']}")

        write_to_file(output_file, "\n" + "-" * 80 + "\n")

if __name__ == "__main__":
    # Use command line arguments for directory and output file
    if len(sys.argv) < 2:
        print("Usage: python script.py <directory_path> [output_file]")
        print("Using current directory as default")
        directory_to_scan = "."
    else:
        directory_to_scan = sys.argv[1]

    # Default output file name
    output_file = sys.argv[2] if len(sys.argv) > 2 else "parquet_schema_report.txt"

    if not os.path.exists(directory_to_scan):
        print(f"Error: Directory '{directory_to_scan}' does not exist")
        exit(1)

    print(f"Scanning directory: {directory_to_scan}")
    scan_and_save_schemas(directory_to_scan, output_file)
    print(f"Schema report saved to {output_file}")
