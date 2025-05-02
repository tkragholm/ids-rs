Struct ListingOptions
Settings
Help
Source

pub struct ListingOptions {
    pub file_extension: String,
    pub format: Arc<dyn FileFormat>,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub collect_stat: bool,
    pub target_partitions: usize,
    pub file_sort_order: Vec<Vec<SortExpr>>,
}

Options for creating a ListingTable
Fields
file_extension: String

A suffix on which files should be filtered (leave empty to keep all files on the path)
format: Arc<dyn FileFormat>

The file format
table_partition_cols: Vec<(String, DataType)>

The expected partition column names in the folder structure. See Self::with_table_partition_cols for details
collect_stat: bool

Set true to try to guess statistics from the files. This can add a lot of overhead as it will usually require files to be opened and at least partially parsed.
target_partitions: usize

Group files to avoid that the number of partitions exceeds this limit
file_sort_order: Vec<Vec<SortExpr>>

Optional pre-known sort order(s). Must be SortExprs.

DataFusion may take advantage of this ordering to omit sorts or use more efficient algorithms. Currently sortedness must be provided if it is known by some external mechanism, but may in the future be automatically determined, for example using parquet metadata.

See https://github.com/apache/datafusion/issues/4177 NOTE: This attribute stores all equivalent orderings (the outer Vec) where each ordering consists of an individual lexicographic ordering (encapsulated by a Vec<Expr>). If there aren’t multiple equivalent orderings, the outer Vec will have a single element.
Implementations
Source
impl ListingOptions
Source
pub fn new(format: Arc<dyn FileFormat>) -> Self

Creates an options instance with the given format Default values:

    use default file extension filter
    no input partition to discover
    one target partition
    stat collection

Source
pub fn with_file_extension(self, file_extension: impl Into<String>) -> Self

Set file extension on ListingOptions and returns self.
Example


let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_file_extension(".parquet");


assert_eq!(listing_options.file_extension, ".parquet");

Source
pub fn with_file_extension_opt<S>(self, file_extension: Option<S>) -> Self
where
    S: Into<String>,

Optionally set file extension on ListingOptions and returns self.

If file_extension is None, the file extension will not be changed
Example

let extension = Some(".parquet");

let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_file_extension_opt(extension);


assert_eq!(listing_options.file_extension, ".parquet");

Source
pub fn with_table_partition_cols(
    self,
    table_partition_cols: Vec<(String, DataType)>,
) -> Self

Set table partition columns on ListingOptions and returns self.

“partition columns,” used to support Hive Partitioning, are columns added to the data that is read, based on the folder structure where the data resides.

For example, give the following files in your filesystem:

/mnt/nyctaxi/year=2022/month=01/tripdata.parquet

/mnt/nyctaxi/year=2021/month=12/tripdata.parquet

/mnt/nyctaxi/year=2021/month=11/tripdata.parquet

A ListingTable created at /mnt/nyctaxi/ with partition columns “year” and “month” will include new year and month columns while reading the files. The year column would have value 2022 and the month column would have value 01 for the rows read from /mnt/nyctaxi/year=2022/month=01/tripdata.parquet
Notes

    If only one level (e.g. year in the example above) is specified, the other levels are ignored but the files are still read.

    Files that don’t follow this partitioning scheme will be ignored.

    Since the columns have the same value for all rows read from each individual file (such as dates), they are typically dictionary encoded for efficiency. You may use wrap_partition_type_in_dict to request a dictionary-encoded type.

    The partition columns are solely extracted from the file path. Especially they are NOT part of the parquet files itself.

Example


// listing options for files with paths such as  `/mnt/data/col_a=x/col_b=y/data.parquet`

// `col_a` and `col_b` will be included in the data read from those files

let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_table_partition_cols(vec![("col_a".to_string(), DataType::Utf8),

      ("col_b".to_string(), DataType::Utf8)]);


assert_eq!(listing_options.table_partition_cols, vec![("col_a".to_string(), DataType::Utf8),

    ("col_b".to_string(), DataType::Utf8)]);

Source
pub fn with_collect_stat(self, collect_stat: bool) -> Self

Set stat collection on ListingOptions and returns self.


let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_collect_stat(true);


assert_eq!(listing_options.collect_stat, true);

Source
pub fn with_target_partitions(self, target_partitions: usize) -> Self

Set number of target partitions on ListingOptions and returns self.


let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_target_partitions(8);


assert_eq!(listing_options.target_partitions, 8);

Source
pub fn with_file_sort_order(self, file_sort_order: Vec<Vec<SortExpr>>) -> Self

Set file sort order on ListingOptions and returns self.


 // Tell datafusion that the files are sorted by column "a"

 let file_sort_order = vec![vec![

   col("a").sort(true, true)

 ]];


let listing_options = ListingOptions::new(Arc::new(

    ParquetFormat::default()

  ))

  .with_file_sort_order(file_sort_order.clone());


assert_eq!(listing_options.file_sort_order, file_sort_order);

Source
pub async fn infer_schema<'a>(
    &'a self,
    state: &dyn Session,
    table_path: &'a ListingTableUrl,
) -> Result<SchemaRef>

Infer the schema of the files at the given path on the provided object store. The inferred schema does not include the partitioning columns.

This method will not be called by the table itself but before creating it. This way when creating the logical plan we can decide to resolve the schema locally or ask a remote service to do it (e.g a scheduler).
Source
pub async fn validate_partitions(
    &self,
    state: &dyn Session,
    table_path: &ListingTableUrl,
) -> Result<()>

Infers the partition columns stored in LOCATION and compares them with the columns provided in PARTITIONED BY to help prevent accidental corrupts of partitioned tables.

Allows specifying partial partitions.
