use crate::error::Result;
use arrow::array::{ArrayRef, BooleanArray, StringArray, UInt32Array, UInt64Array};
use arrow::datatypes::{DataType, SchemaRef};
use datafusion::common::{Column, ScalarValue};
use datafusion::logical_expr::{Expr};
use datafusion::physical_optimizer::pruning::{PruningStatistics};

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a file's statistics for pruning decisions
#[derive(Debug, Clone)]
pub struct FileStatistics {
    /// Path to the file
    pub path: PathBuf,
    /// Size of the file in bytes
    pub size: u64,
    /// Number of rows in the file
    pub row_count: u64,
    /// Minimum values for columns (for range filtering)
    pub min_values: HashMap<String, ScalarValue>,
    /// Maximum values for columns (for range filtering)
    pub max_values: HashMap<String, ScalarValue>,
    /// Optional list of all unique values for certain columns (for IN filtering)
    pub unique_values: HashMap<String, HashSet<String>>,
    /// Optional bloom filter for efficient membership testing
    pub bloom_filters: HashMap<String, Vec<u8>>,
}

impl FileStatistics {
    /// Create new file statistics
    pub fn new(path: impl AsRef<Path>, size: u64, row_count: u64) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            size,
            row_count,
            min_values: HashMap::new(),
            max_values: HashMap::new(),
            unique_values: HashMap::new(),
            bloom_filters: HashMap::new(),
        }
    }

    /// Add a minimum value for a column
    pub fn with_min_value(mut self, column: &str, value: ScalarValue) -> Self {
        self.min_values.insert(column.to_string(), value);
        self
    }

    /// Add a maximum value for a column
    pub fn with_max_value(mut self, column: &str, value: ScalarValue) -> Self {
        self.max_values.insert(column.to_string(), value);
        self
    }

    /// Add unique values for a column
    pub fn with_unique_values(mut self, column: &str, values: HashSet<String>) -> Self {
        self.unique_values.insert(column.to_string(), values);
        self
    }

    /// Check if a file should be processed based on a filter
    pub fn should_process(&self, expr: &Expr) -> bool {
        match expr {
            // Handle binary expressions (like =, >, <, etc.)
            Expr::BinaryExpr(binary) => {
                // Extract column name if left side is a column
                if let Expr::Column(col) = &*binary.left {
                    let column_name = &col.name;

                    // Check if we have statistics for this column
                    if let (Some(min_value), Some(max_value)) = (
                        self.min_values.get(column_name),
                        self.max_values.get(column_name),
                    ) {
                        // For column = value
                        if binary.op == datafusion::logical_expr::Operator::Eq {
                            if let Expr::Literal(val) = &*binary.right {
                                // Check if value is outside min/max range
                                return match (min_value, val, max_value) {
                                    (
                                        ScalarValue::Utf8(Some(min)),
                                        ScalarValue::Utf8(Some(val)),
                                        ScalarValue::Utf8(Some(max)),
                                    ) => val >= min && val <= max,
                                    (
                                        ScalarValue::UInt32(Some(min)),
                                        ScalarValue::UInt32(Some(val)),
                                        ScalarValue::UInt32(Some(max)),
                                    ) => val >= min && val <= max,
                                    // Add more types as needed
                                    _ => true, // If we don't know how to compare, process the file
                                };
                            }
                        }

                        // For column > value
                        if binary.op == datafusion::logical_expr::Operator::Gt {
                            if let Expr::Literal(val) = &*binary.right {
                                // Check if max value is <= the filter value
                                return match (max_value, val) {
                                    (
                                        ScalarValue::Utf8(Some(max)),
                                        ScalarValue::Utf8(Some(val)),
                                    ) => max > val,
                                    (
                                        ScalarValue::UInt32(Some(max)),
                                        ScalarValue::UInt32(Some(val)),
                                    ) => max > val,
                                    // Add more types as needed
                                    _ => true, // If we don't know how to compare, process the file
                                };
                            }
                        }

                        // For column < value
                        if binary.op == datafusion::logical_expr::Operator::Lt {
                            if let Expr::Literal(val) = &*binary.right {
                                // Check if min value is >= the filter value
                                return match (min_value, val) {
                                    (
                                        ScalarValue::Utf8(Some(min)),
                                        ScalarValue::Utf8(Some(val)),
                                    ) => min < val,
                                    (
                                        ScalarValue::UInt32(Some(min)),
                                        ScalarValue::UInt32(Some(val)),
                                    ) => min < val,
                                    // Add more types as needed
                                    _ => true, // If we don't know how to compare, process the file
                                };
                            }
                        }
                    }
                }
                true // Default to processing if we can't determine
            }

            // Handle IN expressions for membership testing
            Expr::InList(in_list) => {
                if let Expr::Column(col) = &*in_list.expr {
                    let column_name = &col.name;

                    // For columns with unique values known
                    if let Some(unique_values) = self.unique_values.get(column_name) {
                        // Check if any of the filter values are in our unique values
                        for list_expr in &in_list.list {
                            if let Expr::Literal(ScalarValue::Utf8(Some(val))) = list_expr {
                                if unique_values.contains(val) {
                                    return true;
                                }
                            }
                        }

                        // If we checked all values and none matched, we can skip this file
                        return false;
                    }

                    // For PNR IN list - special optimization for most common case
                    if column_name == "PNR" || column_name == "CPR" {
                        // If we have a bloom filter, use it for faster checking
                        if let Some(_bloom_filter) = self.bloom_filters.get(column_name) {
                            // TODO: Implement bloom filter checking
                            // This would check if any value in the IN list has a chance of being in the filter
                            return true;
                        }
                    }
                }
                true // Default to processing
            }

            // Handle other expression types
            _ => true, // Default to processing for unsupported expressions
        }
    }
}

/// Pruning statistics for efficient file filtering
#[derive(Debug)]
pub struct RegistryPruningStatistics {
    pub schema: SchemaRef,
    pub files: Vec<FileStatistics>,
    pub min_values: HashMap<String, ArrayRef>,
    pub max_values: HashMap<String, ArrayRef>,
    pub row_counts: Option<ArrayRef>,
}

impl RegistryPruningStatistics {
    /// Create a new instance with schema
    #[must_use]
    pub fn new(schema: SchemaRef) -> Self {
        Self {
            schema,
            files: Vec::new(),
            min_values: HashMap::new(),
            max_values: HashMap::new(),
            row_counts: None,
        }
    }

    /// Add files statistics
    pub fn with_files(mut self, files: Vec<FileStatistics>) -> Self {
        self.files = files;

        // Extract min/max values from files for columns
        for field in self.schema.fields() {
            let column_name = field.name();

            match field.data_type() {
                DataType::Utf8 => {
                    // Extract min string values
                    let min_strings: Vec<Option<String>> = self
                        .files
                        .iter()
                        .map(|f| {
                            f.min_values.get(column_name).and_then(|v| {
                                if let ScalarValue::Utf8(s) = v {
                                    s.clone()
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();

                    if !min_strings.is_empty() && min_strings.iter().any(|s| s.is_some()) {
                        let array = StringArray::from(min_strings);
                        self.min_values.insert(column_name.clone(), Arc::new(array));
                    }

                    // Extract max string values
                    let max_strings: Vec<Option<String>> = self
                        .files
                        .iter()
                        .map(|f| {
                            f.max_values.get(column_name).and_then(|v| {
                                if let ScalarValue::Utf8(s) = v {
                                    s.clone()
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();

                    if !max_strings.is_empty() && max_strings.iter().any(|s| s.is_some()) {
                        let array = StringArray::from(max_strings);
                        self.max_values.insert(column_name.clone(), Arc::new(array));
                    }
                }
                DataType::UInt32 => {
                    // Extract min uint32 values
                    let min_ints: Vec<Option<u32>> = self
                        .files
                        .iter()
                        .map(|f| {
                            f.min_values.get(column_name).and_then(|v| {
                                if let ScalarValue::UInt32(i) = v {
                                    *i
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();

                    if !min_ints.is_empty() && min_ints.iter().any(|i| i.is_some()) {
                        let array = UInt32Array::from(min_ints);
                        self.min_values.insert(column_name.clone(), Arc::new(array));
                    }

                    // Extract max uint32 values
                    let max_ints: Vec<Option<u32>> = self
                        .files
                        .iter()
                        .map(|f| {
                            f.max_values.get(column_name).and_then(|v| {
                                if let ScalarValue::UInt32(i) = v {
                                    *i
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();

                    if !max_ints.is_empty() && max_ints.iter().any(|i| i.is_some()) {
                        let array = UInt32Array::from(max_ints);
                        self.max_values.insert(column_name.clone(), Arc::new(array));
                    }
                }
                // Add other types as needed
                _ => {}
            }
        }

        // Create row counts array
        let row_counts: Vec<Option<u64>> = self.files.iter().map(|f| Some(f.row_count)).collect();

        if !row_counts.is_empty() {
            self.row_counts = Some(Arc::new(UInt64Array::from(row_counts)));
        }

        self
    }

    /// Filter files based on an expression
    pub fn filter_files(&self, expr: &Expr) -> Vec<FileStatistics> {
        self.files
            .iter()
            .filter(|file| file.should_process(expr))
            .cloned()
            .collect()
    }
}

impl PruningStatistics for RegistryPruningStatistics {
    fn num_containers(&self) -> usize {
        self.files.len()
    }

    fn min_values(&self, column: &Column) -> Option<ArrayRef> {
        self.min_values.get(&column.name).cloned()
    }

    fn max_values(&self, column: &Column) -> Option<ArrayRef> {
        self.max_values.get(&column.name).cloned()
    }

    fn row_counts(&self, _column: &Column) -> Option<ArrayRef> {
        self.row_counts.clone()
    }

    fn null_counts(&self, _column: &Column) -> Option<ArrayRef> {
        None // We don't track null counts for now
    }

    fn contained(&self, column: &Column, values: &HashSet<ScalarValue>) -> Option<BooleanArray> {
        // Implementation for specific IN list pruning if needed
        let column_name = &column.name;

        // For each file, check if any of the values in the set could be in the file
        let contained: Vec<bool> = self
            .files
            .iter()
            .map(|file| {
                // Check if we have unique values for this column
                if let Some(unique_values) = file.unique_values.get(column_name) {
                    // If we have exact unique values, check for any overlap
                    for value in values {
                        if let ScalarValue::Utf8(Some(s)) = value {
                            if unique_values.contains(s) {
                                return true;
                            }
                        }
                    }
                    false // No overlap found
                } else {
                    // If we don't have unique values, check min/max bounds
                    if let (Some(min), Some(max)) = (
                        file.min_values.get(column_name),
                        file.max_values.get(column_name),
                    ) {
                        // For each value, check if it could be in the range
                        for value in values {
                            let in_range = match (min, value, max) {
                                (
                                    ScalarValue::Utf8(Some(min_s)),
                                    ScalarValue::Utf8(Some(val_s)),
                                    ScalarValue::Utf8(Some(max_s)),
                                ) => val_s >= min_s && val_s <= max_s,
                                (
                                    ScalarValue::UInt32(Some(min_i)),
                                    ScalarValue::UInt32(Some(val_i)),
                                    ScalarValue::UInt32(Some(max_i)),
                                ) => val_i >= min_i && val_i <= max_i,
                                // Add more types as needed
                                _ => true, // If we can't compare, assume it might be contained
                            };

                            if in_range {
                                return true;
                            }
                        }
                        false // No values in range
                    } else {
                        true // If we don't have min/max, assume it might be contained
                    }
                }
            })
            .collect();

        Some(BooleanArray::from(contained))
    }
}

/// Extract statistics from parquet files for a column
pub async fn extract_column_statistics(
    paths: &[impl AsRef<Path>],
    column_name: &str,
) -> Result<HashMap<PathBuf, (ScalarValue, ScalarValue)>> {
    use datafusion::execution::context::SessionContext;
    use datafusion::functions_aggregate::expr_fn::{max, min};
    use datafusion::logical_expr::col;
    use datafusion::prelude::*;

    let mut result = HashMap::new();
    let ctx = SessionContext::new();

    for path in paths {
        let path = path.as_ref();
        if !path.exists() || !path.is_file() {
            continue;
        }

        // Read the file with page index enabled
        let df = ctx
            .read_parquet(
                path.to_string_lossy().to_string(),
                ParquetReadOptions::default(),
            )
            .await?;

        // Read minimal data to compute statistics
        let df = df.select_columns(&[column_name])?;
        let min_df = df.clone().aggregate(vec![], vec![min(col(column_name))])?;
        let max_df = df.aggregate(vec![], vec![max(col(column_name))])?;

        // Execute and get results
        let min_batch = min_df.collect().await?;
        let max_batch = max_df.collect().await?;

        if !min_batch.is_empty() && !max_batch.is_empty() {
            // Extract min value
            let min_col = min_batch[0].column(0);
            let min_val = if min_col.is_null(0) {
                continue;
            } else {
                // Get ScalarValue based on type
                match min_col.data_type() {
                    DataType::Utf8 => {
                        let array = min_col.as_any().downcast_ref::<StringArray>().unwrap();
                        ScalarValue::Utf8(Some(array.value(0).to_string()))
                    }
                    DataType::UInt32 => {
                        let array = min_col.as_any().downcast_ref::<UInt32Array>().unwrap();
                        ScalarValue::UInt32(Some(array.value(0)))
                    }
                    // Add more types as needed
                    _ => continue,
                }
            };

            // Extract max value
            let max_col = max_batch[0].column(0);
            let max_val = if max_col.is_null(0) {
                continue;
            } else {
                // Get ScalarValue based on type
                match max_col.data_type() {
                    DataType::Utf8 => {
                        let array = max_col.as_any().downcast_ref::<StringArray>().unwrap();
                        ScalarValue::Utf8(Some(array.value(0).to_string()))
                    }
                    DataType::UInt32 => {
                        let array = max_col.as_any().downcast_ref::<UInt32Array>().unwrap();
                        ScalarValue::UInt32(Some(array.value(0)))
                    }
                    // Add more types as needed
                    _ => continue,
                }
            };

            result.insert(path.to_path_buf(), (min_val, max_val));
        }
    }

    Ok(result)
}