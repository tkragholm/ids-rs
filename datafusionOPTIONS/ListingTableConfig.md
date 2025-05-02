Struct ListingTableConfig
Settings
Help
Source

pub struct ListingTableConfig {
    pub table_paths: Vec<ListingTableUrl>,
    pub file_schema: Option<SchemaRef>,
    pub options: Option<ListingOptions>,
}

Configuration for creating a ListingTable
Fields
table_paths: Vec<ListingTableUrl>

Paths on the ObjectStore for creating ListingTable. They should share the same schema and object store.
file_schema: Option<SchemaRef>

Optional SchemaRef for the to be created ListingTable.
options: Option<ListingOptions>

Optional ListingOptions for the to be created ListingTable.
Implementations
Source
impl ListingTableConfig
Source
pub fn new(table_path: ListingTableUrl) -> Self

Creates new ListingTableConfig.

The SchemaRef and ListingOptions are inferred based on the suffix of the provided table_paths first element.
Source
pub fn new_with_multi_paths(table_paths: Vec<ListingTableUrl>) -> Self

Creates new ListingTableConfig with multiple table paths.

The SchemaRef and ListingOptions are inferred based on the suffix of the provided table_paths first element.
Source
pub fn with_schema(self, schema: SchemaRef) -> Self

Add schema to ListingTableConfig
Source
pub fn with_listing_options(self, listing_options: ListingOptions) -> Self

Add listing_options to ListingTableConfig
Source
pub async fn infer_options(self, state: &dyn Session) -> Result<Self>

Infer ListingOptions based on table_path suffix.
Source
pub async fn infer_schema(self, state: &dyn Session) -> Result<Self>

Infer the SchemaRef based on table_path suffix. Requires self.options to be set prior to using.
Source
pub async fn infer(self, state: &dyn Session) -> Result<Self>

Convenience wrapper for calling infer_options and infer_schema
Source
pub async fn infer_partitions_from_path(
    self,
    state: &dyn Session,
) -> Result<Self>

Infer the partition columns from the path. Requires self.options to be set prior to using.
