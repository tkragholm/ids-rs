import os
import sys

import pyarrow as pa
import pyarrow.parquet as pq


def estimate_cardinality(stats):
    """Estimate cardinality using min/max values and pattern analysis"""
    if not stats.has_min_max or not stats.min or not stats.max:
        return None

    # Number of non-null values
    non_null_count = stats.num_values

    # Look at the pattern of min/max values
    min_val = stats.min
    max_val = stats.max

    # Check if values look like IDs (mix of numbers and special chars)
    if any(c.isdigit() for c in min_val) and any(not c.isalnum() for c in min_val):
        # Likely an ID field (e.g., "010100-5803")
        return int(non_null_count * 0.9)  # Assume high cardinality

    # If values are pure digits
    elif min_val.isdigit() and max_val.isdigit():
        # Estimate based on the numeric range
        range_size = int(max_val) - int(min_val)
        return min(range_size, non_null_count)

    # If values are very different in length
    elif abs(len(min_val) - len(max_val)) > 5:
        # Likely varying content, high cardinality
        return int(non_null_count * 0.8)

    # Default case - moderate cardinality
    return int(non_null_count * 0.5)


class SchemaGrouper:
    def __init__(self, similarity_threshold=0.9):
        self.schema_groups = {}
        self.similarity_threshold = similarity_threshold

    def _create_schema_fingerprint(self, schema_info):
        if "error" in schema_info:
            return f"ERROR:{schema_info['error']}"

        columns = tuple(
            (col["name"], col["type"]) for col in schema_info["columns_info"]
        )

        metadata = schema_info["metadata"]
        return (columns, tuple(metadata["compression_codecs"]))

    def _calculate_schema_similarity(self, schema1, schema2):
        cols1 = set((col["name"], col["type"]) for col in schema1["columns_info"])
        cols2 = set((col["name"], col["type"]) for col in schema2["columns_info"])

        common_cols = cols1.intersection(cols2)
        total_cols = max(len(cols1), len(cols2))

        return len(common_cols) / total_cols

    def _get_column_differences(self, base_schema, other_schema):
        base_cols = {col["name"]: col for col in base_schema["columns_info"]}
        other_cols = {col["name"]: col for col in other_schema["columns_info"]}

        added = set(other_cols.keys()) - set(base_cols.keys())
        removed = set(base_cols.keys()) - set(other_cols.keys())

        return {
            "added": [other_cols[name] for name in added],
            "removed": [base_cols[name] for name in removed],
        }

    def add_file(self, file_path, schema_info):
        if "error" in schema_info:
            group_key = f"ERROR:{schema_info['error']}"
            if group_key not in self.schema_groups:
                self.schema_groups[group_key] = {
                    "files": [],
                    "schema_info": schema_info,
                }
            self.schema_groups[group_key]["files"].append(file_path)
            return

        exact_fingerprint = self._create_schema_fingerprint(schema_info)

        if exact_fingerprint in self.schema_groups:
            group = self.schema_groups[exact_fingerprint]
            group["files"].append(file_path)
            group["row_counts"].append(schema_info["metadata"]["num_rows"])
            group["total_size"] += schema_info["metadata"]["total_compressed_size"]
            return

        best_match = None
        best_similarity = 0

        for fp, group in self.schema_groups.items():
            if isinstance(fp, str) and fp.startswith("ERROR:"):
                continue

            similarity = self._calculate_schema_similarity(
                group["schema_info"], schema_info
            )

            if similarity > best_similarity and similarity >= self.similarity_threshold:
                best_similarity = similarity
                best_match = fp

        if best_match:
            group = self.schema_groups[best_match]
            if "variations" not in group:
                group["variations"] = []
                group["variations"].append(
                    {
                        "files": group["files"].copy(),
                        "schema_info": group["schema_info"],
                        "row_counts": group["row_counts"].copy(),
                        "total_size": group["total_size"],
                    }
                )
                group["files"] = []
                group["row_counts"] = []
                group["total_size"] = 0

            variation = {
                "files": [file_path],
                "schema_info": schema_info,
                "row_counts": [schema_info["metadata"]["num_rows"]],
                "total_size": schema_info["metadata"]["total_compressed_size"],
                "differences": self._get_column_differences(
                    group["schema_info"], schema_info
                ),
            }
            group["variations"].append(variation)
        else:
            self.schema_groups[exact_fingerprint] = {
                "files": [file_path],
                "schema_info": schema_info,
                "row_counts": [schema_info["metadata"]["num_rows"]],
                "total_size": schema_info["metadata"]["total_compressed_size"],
            }


def get_parquet_schema_info(file_path):
    """Get detailed schema information using ParquetFile API."""
    try:
        parquet_file = pq.ParquetFile(file_path)
        metadata = parquet_file.metadata
        schema = parquet_file.schema_arrow

        print(f"\nDebug: Processing {file_path}")

        perf_metadata = {
            "num_row_groups": metadata.num_row_groups,
            "num_rows": metadata.num_rows,
            "num_columns": metadata.num_columns,
            "created_by": metadata.created_by,
            "total_compressed_size": metadata.serialized_size,
            "compression_codecs": set(),
            "masked_columns_count": 0,
            "high_cardinality_columns": 0,
        }

        # Process columns
        columns_info = []
        for i, field_name in enumerate(schema.names):
            field = schema.field(i)
            field_type = field.type

            is_string = pa.types.is_string(field_type) or (
                pa.types.is_dictionary(field_type)
                and pa.types.is_string(field_type.value_type)
            )

            if is_string:
                print(f"\nDebug: Processing string column {field_name}")

            stats = None
            if perf_metadata["num_row_groups"] > 0:
                try:
                    col_meta = metadata.row_group(0).column(i)
                    compression = col_meta.compression
                    perf_metadata["compression_codecs"].add(compression)

                    print(f"  Compression: {compression}")
                    print(f"  Has statistics: {col_meta.is_stats_set}")

                    stats = col_meta.statistics
                    if stats:
                        print(f"  Stats available: {bool(stats)}")
                        print(f"  Has min/max: {stats.has_min_max}")
                        if stats.has_min_max:
                            print(f"  Min value: {stats.min}")
                            print(f"  Max value: {stats.max}")
                        print(f"  Num values: {stats.num_values}")
                        print(f"  Null count: {stats.null_count}")

                        stats_dict = {
                            "null_count": stats.null_count,
                            "num_values": stats.num_values,
                            "is_masked": False,
                        }

                        if is_string and stats.has_min_max:
                            estimated_distinct = estimate_cardinality(stats)
                            print(f"  Estimated distinct count: {estimated_distinct}")

                            if estimated_distinct is not None:
                                non_null_count = stats.num_values
                                cardinality_ratio = estimated_distinct / non_null_count
                                stats_dict["cardinality_ratio"] = cardinality_ratio
                                print(
                                    f"  Calculated cardinality ratio: {cardinality_ratio:.2%}"
                                )

                                if cardinality_ratio > 0.25:
                                    stats_dict["is_masked"] = True
                                    perf_metadata["masked_columns_count"] += 1
                                    perf_metadata["high_cardinality_columns"] += 1

                        columns_info.append(
                            {
                                "name": field_name,
                                "type": str(field_type),
                                "is_string": is_string,
                                "stats": stats_dict,
                            }
                        )
                        continue

                except Exception as e:
                    print(f"  Error processing statistics: {e}")

            columns_info.append(
                {
                    "name": field_name,
                    "type": str(field_type),
                    "is_string": is_string,
                    "stats": None,
                }
            )

        return {
            "schema": schema,
            "metadata": perf_metadata,
            "columns_info": columns_info,
        }
    except Exception as e:
        return {"error": str(e)}


def write_compressed_report(output_file, schema_groups):
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(f"Schema Group Report\n{'=' * 50}\n\n")

        for i, (_, group) in enumerate(schema_groups.items(), 1):
            if "error" in group["schema_info"]:
                f.write(f"Error Group {i} ({len(group['files'])} files):\n")
                f.write(f"  Error: {group['schema_info']['error']}\n")
                f.write("  Files:\n")
                for file_path in group["files"]:
                    f.write(f"    - {file_path}\n")
                f.write("\n")
                continue

            total_files = len(group["files"])
            if "variations" in group:
                total_files += sum(len(var["files"]) for var in group["variations"])

            f.write(f"Schema Group {i} ({total_files} files):\n")
            metadata = group["schema_info"]["metadata"]

            if group.get("files"):
                f.write("  Base Schema Stats:\n")
                f.write(f"    Files: {len(group['files'])}\n")
                f.write(f"    Total Size: {group['total_size']:,} bytes\n")
                f.write(
                    f"    Row Count Range: {min(group['row_counts']):,} - {max(group['row_counts']):,}\n"
                )

            f.write(f"  Columns: {metadata['num_columns']}\n")
            f.write(f"  Compression: {', '.join(metadata['compression_codecs'])}\n")
            f.write(f"  Masked Columns: {metadata['masked_columns_count']}\n")
            f.write(
                f"  High Cardinality Columns: {metadata['high_cardinality_columns']}\n"
            )

            f.write("  Column Details:\n")
            for col in group["schema_info"]["columns_info"]:
                stats = col.get("stats")
                mask_indicator = " [MASKED]" if stats and stats.get("is_masked") else ""
                f.write(f"    {col['name']} ({col['type']}){mask_indicator}\n")

                if stats and col.get("is_string"):
                    card_ratio = stats.get("cardinality_ratio")
                    if card_ratio is not None:  # Only show cardinality if we have it
                        f.write(f"      cardinality: {card_ratio:.2%}\n")


def scan_and_save_schemas(directory, output_file):
    grouper = SchemaGrouper()
    parquet_files = find_parquet_files(directory)

    if not parquet_files:
        print(f"No parquet files found in {directory}")
        return

    print(f"Processing {len(parquet_files)} parquet files...")

    for i, file_path in enumerate(parquet_files, 1):
        print(f"\rProcessing file {i}/{len(parquet_files)}", end="")
        try:
            schema_info = get_parquet_schema_info(file_path)
            grouper.add_file(file_path, schema_info)
        except Exception as e:
            grouper.add_file(file_path, {"error": str(e)})

    print("\nWriting compressed report...")
    write_compressed_report(output_file, grouper.schema_groups)

    print(f"\nFound {len(grouper.schema_groups)} unique schema groups")
    print(f"Report saved to {output_file}")


def find_parquet_files(directory):
    """Recursively find all parquet files in a directory."""
    parquet_files = []
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".parquet"):
                parquet_files.append(os.path.join(root, file))
    return parquet_files


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <directory_path> [output_file]")
        print("Using current directory as default")
        directory_to_scan = "."
    else:
        directory_to_scan = sys.argv[1]

    output_file = sys.argv[2] if len(sys.argv) > 2 else "parquet_schema_report.txt"

    if not os.path.exists(directory_to_scan):
        print(f"Error: Directory '{directory_to_scan}' does not exist")
        exit(1)

    print(f"Scanning directory: {directory_to_scan}")
    scan_and_save_schemas(directory_to_scan, output_file)
