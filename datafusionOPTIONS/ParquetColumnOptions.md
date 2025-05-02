Struct ParquetColumnOptions
Settings
Help
Source

pub struct ParquetColumnOptions {
    pub bloom_filter_enabled: Option<bool>,
    pub encoding: Option<String>,
    pub dictionary_enabled: Option<bool>,
    pub compression: Option<String>,
    pub statistics_enabled: Option<String>,
    pub bloom_filter_fpp: Option<f64>,
    pub bloom_filter_ndv: Option<u64>,
    pub max_statistics_size: Option<usize>,
}

Options controlling parquet format for individual columns.

See ParquetOptions for more details
Fields
bloom_filter_enabled: Option<bool>

Sets if bloom filter is enabled for the column path.
encoding: Option<String>

Sets encoding for the column path. Valid values are: plain, plain_dictionary, rle, bit_packed, delta_binary_packed, delta_length_byte_array, delta_byte_array, rle_dictionary, and byte_stream_split. These values are not case-sensitive. If NULL, uses default parquet options
dictionary_enabled: Option<bool>

Sets if dictionary encoding is enabled for the column path. If NULL, uses default parquet options
compression: Option<String>

Sets default parquet compression codec for the column path. Valid values are: uncompressed, snappy, gzip(level), lzo, brotli(level), lz4, zstd(level), and lz4_raw. These values are not case-sensitive. If NULL, uses default parquet options
statistics_enabled: Option<String>

Sets if statistics are enabled for the column Valid values are: “none”, “chunk”, and “page” These values are not case sensitive. If NULL, uses default parquet options
bloom_filter_fpp: Option<f64>

Sets bloom filter false positive probability for the column path. If NULL, uses default parquet options
bloom_filter_ndv: Option<u64>

Sets bloom filter number of distinct values. If NULL, uses default parquet options
max_statistics_size: Option<usize>
👎Deprecated since 45.0.0: Setting does not do anything

Sets max statistics size for the column path. If NULL, uses default parquet options max_statistics_size is deprecated, currently it is not being used
Trait Implementations
Source
impl Clone for ParquetColumnOptions
Source
fn clone(&self) -> ParquetColumnOptions
Returns a copy of the value. Read more
1.0.0 · Source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
Source
impl ConfigField for ParquetColumnOptions
Source
fn set(&mut self, key: &str, value: &str) -> Result<(), DataFusionError>
Source
fn visit<V>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
where
    V: Visit,
Source
impl Debug for ParquetColumnOptions
Source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
Source
impl Default for ParquetColumnOptions
Source
fn default() -> ParquetColumnOptions
Returns the “default value” for a type. Read more
Source
impl PartialEq for ParquetColumnOptions
Source
fn eq(&self, other: &ParquetColumnOptions) -> bool
Tests for self and other values to be equal, and is used by ==.
1.0.0 · Source
fn ne(&self, other: &Rhs) -> bool
Tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.
