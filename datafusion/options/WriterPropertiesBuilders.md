Struct WriterPropertiesBuilder
Settings
Help
Source

pub struct WriterPropertiesBuilder { /* private fields */ }

Builder for WriterProperties parquet writer configuration.

See example on WriterProperties
Implementations
Source
impl WriterPropertiesBuilder
Source
pub fn build(self) -> WriterProperties

Finalizes the configuration and returns immutable writer properties struct.
Source
pub fn set_writer_version(self, value: WriterVersion) -> Self

Sets the WriterVersion written into the parquet metadata (defaults to PARQUET_1_0)

This value can determine what features some readers will support.
Source
pub fn set_data_page_size_limit(self, value: usize) -> Self

Sets best effort maximum size of a data page in bytes (defaults to 1024 * 1024).

The parquet writer will attempt to limit the sizes of each DataPage to this many bytes. Reducing this value will result in larger parquet files, but may improve the effectiveness of page index based predicate pushdown during reading.

Note: this is a best effort limit based on value of set_write_batch_size.
Source
pub fn set_data_page_row_count_limit(self, value: usize) -> Self

Sets best effort maximum number of rows in a data page (defaults to 20_000).

The parquet writer will attempt to limit the number of rows in each DataPage to this value. Reducing this value will result in larger parquet files, but may improve the effectiveness of page index based predicate pushdown during reading.

Note: this is a best effort limit based on value of set_write_batch_size.
Source
pub fn set_dictionary_page_size_limit(self, value: usize) -> Self

Sets best effort maximum dictionary page size, in bytes (defaults to 1024 * 1024).

The parquet writer will attempt to limit the size of each DataPage used to store dictionaries to this many bytes. Reducing this value will result in larger parquet files, but may improve the effectiveness of page index based predicate pushdown during reading.

Note: this is a best effort limit based on value of set_write_batch_size.
Source
pub fn set_write_batch_size(self, value: usize) -> Self

Sets write batch size (defaults to 1024).

For performance reasons, data for each column is written in batches of this size.

Additional limits such as such as set_data_page_row_count_limit are checked between batches, and thus the write batch size value acts as an upper-bound on the enforcement granularity of other limits.
Source
pub fn set_max_row_group_size(self, value: usize) -> Self

Sets maximum number of rows in a row group (defaults to 1024 * 1024).
Panics

If the value is set to 0.
Source
pub fn set_bloom_filter_position(self, value: BloomFilterPosition) -> Self

Sets where in the final file Bloom Filters are written (default AfterRowGroup)
Source
pub fn set_created_by(self, value: String) -> Self

Sets “created by” property (defaults to parquet-rs version <VERSION>).
Source
pub fn set_offset_index_disabled(self, value: bool) -> Self

Sets whether the writing of offset indexes is disabled (defaults to false).

If statistics level is set to Page this setting will be overridden with false.

Note: As the offset indexes are useful for accessing data by row number, they are always written by default, regardless of whether other statistics are enabled. Disabling this metadata may result in a degradation in read performance, so use this option with care.
Source
pub fn set_key_value_metadata(self, value: Option<Vec<KeyValue>>) -> Self

Sets “key_value_metadata” property (defaults to None).
Source
pub fn set_sorting_columns(self, value: Option<Vec<SortingColumn>>) -> Self

Sets sorting order of rows in the row group if any (defaults to None).
Source
pub fn set_encoding(self, value: Encoding) -> Self

Sets default encoding for all columns.

If dictionary is not enabled, this is treated as a primary encoding for all columns. In case when dictionary is enabled for any column, this value is considered to be a fallback encoding for that column.
Panics

if dictionary encoding is specified, regardless of dictionary encoding flag being set.
Source
pub fn set_compression(self, value: Compression) -> Self

Sets default compression codec for all columns (default to UNCOMPRESSED).
Source
pub fn set_dictionary_enabled(self, value: bool) -> Self

Sets default flag to enable/disable dictionary encoding for all columns (defaults to true).

Use this method to set dictionary encoding, instead of explicitly specifying encoding in set_encoding method.
Source
pub fn set_statistics_enabled(self, value: EnabledStatistics) -> Self

Sets default statistics level for all columns (defaults to Page).
Source
pub fn set_max_statistics_size(self, value: usize) -> Self
👎Deprecated since 54.0.0: Unused; will be removed in 56.0.0

Sets default max statistics size for all columns (defaults to 4096).

Applicable only if statistics are enabled.
Source
pub fn set_bloom_filter_enabled(self, value: bool) -> Self

Sets if bloom filter is enabled by default for all columns (defaults to false).
Notes

    If the bloom filter is enabled previously then it is a no-op.

    If the bloom filter is not enabled, default values for ndv and fpp value are used used. See set_bloom_filter_ndv and set_bloom_filter_fpp to further adjust the ndv and fpp.

Source
pub fn set_bloom_filter_fpp(self, value: f64) -> Self

Sets the default target bloom filter false positive probability (fpp) for all columns (defaults to 0.05).

Implicitly enables bloom writing, as if set_bloom_filter_enabled had been called.
Source
pub fn set_bloom_filter_ndv(self, value: u64) -> Self

Sets default number of distinct values (ndv) for bloom filter for all columns (defaults to 1_000_000).

Implicitly enables bloom writing, as if set_bloom_filter_enabled had been called.
Source
pub fn set_column_encoding(self, col: ColumnPath, value: Encoding) -> Self

Sets encoding for a specific column.

Takes precedence over Self::set_encoding.

If dictionary is not enabled, this is treated as a primary encoding for this column. In case when dictionary is enabled for this column, either through global defaults or explicitly, this value is considered to be a fallback encoding for this column.
Panics

If user tries to set dictionary encoding here, regardless of dictionary encoding flag being set.
Source
pub fn set_column_compression(self, col: ColumnPath, value: Compression) -> Self

Sets compression codec for a specific column.

Takes precedence over Self::set_compression.
Source
pub fn set_column_dictionary_enabled(self, col: ColumnPath, value: bool) -> Self

Sets flag to enable/disable dictionary encoding for a specific column.

Takes precedence over Self::set_dictionary_enabled.
Source
pub fn set_column_statistics_enabled(
    self,
    col: ColumnPath,
    value: EnabledStatistics,
) -> Self

Sets statistics level for a specific column.

Takes precedence over Self::set_statistics_enabled.
Source
pub fn set_column_max_statistics_size(
    self,
    col: ColumnPath,
    value: usize,
) -> Self
👎Deprecated since 54.0.0: Unused; will be removed in 56.0.0

Sets max size for statistics for a specific column.

Takes precedence over Self::set_max_statistics_size.
Source
pub fn set_column_bloom_filter_enabled(
    self,
    col: ColumnPath,
    value: bool,
) -> Self

Sets whether a bloom filter should be written for a specific column.

Takes precedence over Self::set_bloom_filter_enabled.
Source
pub fn set_column_bloom_filter_fpp(self, col: ColumnPath, value: f64) -> Self

Sets the false positive probability for bloom filter for a specific column.

Takes precedence over Self::set_bloom_filter_fpp.
Source
pub fn set_column_bloom_filter_ndv(self, col: ColumnPath, value: u64) -> Self

Sets the number of distinct values for bloom filter for a specific column.

Takes precedence over Self::set_bloom_filter_ndv.
Source
pub fn set_column_index_truncate_length(self, max_length: Option<usize>) -> Self

Sets the max length of min/max value fields when writing the column Index (defaults to None).

This can be used to prevent columns with very long values (hundreds of bytes long) from causing the parquet metadata to become huge.
Notes

The column Index is written when Self::set_statistics_enabled is set to EnabledStatistics::Page.

    If Some, must be greater than 0, otherwise will panic
    If None, there’s no effective limit.

Source
pub fn set_statistics_truncate_length(self, max_length: Option<usize>) -> Self

Sets the max length of min/max value fields in row group level Statistics (defaults to None).
Notes

Row group level Statistics are written when Self::set_statistics_enabled is set to EnabledStatistics::Chunk or EnabledStatistics::Page.

    If Some, must be greater than 0, otherwise will panic
    If None, there’s no effective limit.

Source
pub fn set_coerce_types(self, coerce_types: bool) -> Self

Should the writer coerce types to parquet native types (defaults to false).

Leaving this option the default false will ensure the exact same data written to parquet using this library will be read.

Setting this option to true will result in parquet files that can be read by more readers, but potentially lose information in the process.

    Types such as DataType::Date64, which have no direct corresponding Parquet type, may be stored with lower precision.

    The internal field names of List and Map types will be renamed if necessary to match what is required by the newest Parquet specification.

See ArrowToParquetSchemaConverter::with_coerce_types for more details
Source
pub fn with_file_encryption_properties(
    self,
    file_encryption_properties: FileEncryptionProperties,
) -> Self
Available on crate feature encryption only.

Sets FileEncryptionProperties.
