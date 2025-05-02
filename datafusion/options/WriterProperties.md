Struct WriterProperties
Settings
Help
Source

pub struct WriterProperties { /* private fields */ }

Configuration settings for writing parquet files.

Use Self::builder to create a WriterPropertiesBuilder to change settings.
Example

// Create properties with default configuration.

let props = WriterProperties::default();


// Use properties builder to set certain options and assemble the configuration.

let props = WriterProperties::builder()

    .set_writer_version(WriterVersion::PARQUET_1_0)

    .set_encoding(Encoding::PLAIN)

    .set_column_encoding(ColumnPath::from("col1"), Encoding::DELTA_BINARY_PACKED)

    .set_compression(Compression::SNAPPY)

    .build();


assert_eq!(props.writer_version(), WriterVersion::PARQUET_1_0);

assert_eq!(

    props.encoding(&ColumnPath::from("col1")),

    Some(Encoding::DELTA_BINARY_PACKED)

);

assert_eq!(

    props.encoding(&ColumnPath::from("col2")),

    Some(Encoding::PLAIN)

);

Implementations
Source
impl WriterProperties
Source
pub fn new() -> Self

Create a new WriterProperties with the default settings

See WriterProperties::builder for customising settings
Source
pub fn builder() -> WriterPropertiesBuilder

Returns a new default WriterPropertiesBuilder for creating writer properties.
Source
pub fn data_page_size_limit(&self) -> usize

Returns data page size limit.

Note: this is a best effort limit based on the write batch size

For more details see WriterPropertiesBuilder::set_data_page_size_limit
Source
pub fn dictionary_page_size_limit(&self) -> usize

Returns dictionary page size limit.

Note: this is a best effort limit based on the write batch size

For more details see WriterPropertiesBuilder::set_dictionary_page_size_limit
Source
pub fn data_page_row_count_limit(&self) -> usize

Returns the maximum page row count

Note: this is a best effort limit based on the write batch size

For more details see WriterPropertiesBuilder::set_data_page_row_count_limit
Source
pub fn write_batch_size(&self) -> usize

Returns configured batch size for writes.

When writing a batch of data, this setting allows to split it internally into smaller batches so we can better estimate the size of a page currently being written.
Source
pub fn max_row_group_size(&self) -> usize

Returns maximum number of rows in a row group.
Source
pub fn bloom_filter_position(&self) -> BloomFilterPosition

Returns bloom filter position.
Source
pub fn writer_version(&self) -> WriterVersion

Returns configured writer version.
Source
pub fn created_by(&self) -> &str

Returns created_by string.
Source
pub fn offset_index_disabled(&self) -> bool

Returns true if offset index writing is disabled.
Source
pub fn key_value_metadata(&self) -> Option<&Vec<KeyValue>>

Returns key_value_metadata KeyValue pairs.
Source
pub fn sorting_columns(&self) -> Option<&Vec<SortingColumn>>

Returns sorting columns.
Source
pub fn column_index_truncate_length(&self) -> Option<usize>

Returns the maximum length of truncated min/max values in the column index.

None if truncation is disabled, must be greater than 0 otherwise.
Source
pub fn statistics_truncate_length(&self) -> Option<usize>

Returns the maximum length of truncated min/max values in statistics.

None if truncation is disabled, must be greater than 0 otherwise.
Source
pub fn coerce_types(&self) -> bool

Returns true if type coercion is enabled.
Source
pub fn dictionary_data_page_encoding(&self) -> Encoding

Returns encoding for a data page, when dictionary encoding is enabled. This is not configurable.
Source
pub fn dictionary_page_encoding(&self) -> Encoding

Returns encoding for dictionary page, when dictionary encoding is enabled. This is not configurable.
Source
pub fn encoding(&self, col: &ColumnPath) -> Option<Encoding>

Returns encoding for a column, if set. In case when dictionary is enabled, returns fallback encoding.

If encoding is not set, then column writer will choose the best encoding based on the column type.
Source
pub fn compression(&self, col: &ColumnPath) -> Compression

Returns compression codec for a column.
Source
pub fn dictionary_enabled(&self, col: &ColumnPath) -> bool

Returns true if dictionary encoding is enabled for a column.
Source
pub fn statistics_enabled(&self, col: &ColumnPath) -> EnabledStatistics

Returns which statistics are written for a column.
Source
pub fn max_statistics_size(&self, col: &ColumnPath) -> usize
👎Deprecated since 54.0.0: Unused; will be removed in 56.0.0

Returns max size for statistics. Only applicable if statistics are enabled.
Source
pub fn bloom_filter_properties(
    &self,
    col: &ColumnPath,
) -> Option<&BloomFilterProperties>

Returns the BloomFilterProperties for the given column

Returns None if bloom filter is disabled
Source
pub fn file_encryption_properties(&self) -> Option<&FileEncryptionProperties>
Available on crate feature encryption only.

Return file encryption properties
