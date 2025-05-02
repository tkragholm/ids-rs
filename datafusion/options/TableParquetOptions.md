Struct TableParquetOptions
Settings
Help
Source

pub struct TableParquetOptions {
    pub global: ParquetOptions,
    pub column_specific_options: HashMap<String, ParquetColumnOptions>,
    pub key_value_metadata: HashMap<String, Option<String>>,
}

Options that control how Parquet files are read, including global options that apply to all columns and optional column-specific overrides

Closely tied to ParquetWriterOptions. Properties not included in TableParquetOptions may not be configurable at the external API (e.g. sorting_columns).
Fields
global: ParquetOptions

Global Parquet options that propagates to all columns.
column_specific_options: HashMap<String, ParquetColumnOptions>

Column specific options. Default usage is parquet.XX::column.
key_value_metadata: HashMap<String, Option<String>>

Additional file-level metadata to include. Inserted into the key_value_metadata for the written FileMetaData.

Multiple entries are permitted

OPTIONS (

   'format.metadata::key1' '',

   'format.metadata::key2' 'value',

   'format.metadata::key3' 'value has spaces',

   'format.metadata::key4' 'value has special chars :: :',

   'format.metadata::key_dupe' 'original will be overwritten',

   'format.metadata::key_dupe' 'final'

)

Implementations
Source
impl TableParquetOptions
Source
pub fn new() -> TableParquetOptions

Return new default TableParquetOptions
Source
pub fn with_skip_arrow_metadata(self, skip: bool) -> TableParquetOptions

Set whether the encoding of the arrow metadata should occur during the writing of parquet.

Default is to encode the arrow schema in the file kv_metadata.
Source
impl TableParquetOptions
Source
pub fn arrow_schema(&mut self, schema: &Arc<Schema>)

Add the arrow schema to the parquet kv_metadata. If already exists, then overwrites.
