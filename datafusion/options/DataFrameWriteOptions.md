Struct DataFrameWriteOptions
Source
Settings
Help

pub struct DataFrameWriteOptions { /* private fields */ }

Contains options that control how data is written out from a DataFrame
Implementations
Source
impl DataFrameWriteOptions
Source
pub fn new() -> Self

Create a new DataFrameWriteOptions with default values
Source
pub fn with_insert_operation(self, insert_op: InsertOp) -> Self

Set the insert operation
Source
pub fn with_single_file_output(self, single_file_output: bool) -> Self

Set the single_file_output value to true or false
Source
pub fn with_partition_by(self, partition_by: Vec<String>) -> Self

Sets the partition_by columns for output partitioning
Source
pub fn with_sort_by(self, sort_by: Vec<SortExpr>) -> Self

Sets the sort_by columns for output sorting
