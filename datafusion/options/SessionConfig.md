Struct SessionConfig
Settings
Help
Source

pub struct SessionConfig { /* private fields */ }

Configuration options for SessionContext.

Can be passed to SessionContext::new_with_config to customize the configuration of DataFusion.

Options can be set using namespaces keys with . as the separator, where the namespace determines which configuration struct the value to routed to. All built-in options are under the datafusion namespace.

For example, the key datafusion.execution.batch_size will set ExecutionOptions::batch_size, because ConfigOptions::execution is ExecutionOptions. Similarly, the key datafusion.execution.parquet.pushdown_filters will set ParquetOptions::pushdown_filters, since ExecutionOptions::parquet is ParquetOptions.

Some options have convenience methods. For example SessionConfig::with_batch_size is shorthand for setting datafusion.execution.batch_size.

use datafusion_execution::config::SessionConfig;

use datafusion_common::ScalarValue;


let config = SessionConfig::new()

   .set("datafusion.execution.batch_size", &ScalarValue::UInt64(Some(1234)))

   .set_bool("datafusion.execution.parquet.pushdown_filters", true);


assert_eq!(config.batch_size(), 1234);

assert_eq!(config.options().execution.batch_size, 1234);

assert_eq!(config.options().execution.parquet.pushdown_filters, true);

You can also directly mutate the options via SessionConfig::options_mut. So the following is equivalent to the above:

let mut config = SessionConfig::new();

config.options_mut().execution.batch_size = 1234;

config.options_mut().execution.parquet.pushdown_filters = true;

Built-in options
Namespace	Config struct
datafusion.catalog	CatalogOptions
datafusion.execution	ExecutionOptions
datafusion.execution.parquet	ParquetOptions
datafusion.optimizer	OptimizerOptions
datafusion.sql_parser	SqlParserOptions
datafusion.explain	ExplainOptions
Custom configuration

Configuration options can be extended. See SessionConfig::with_extension for details.
Implementations
Source
impl SessionConfig
Source
pub fn new() -> SessionConfig

Create an execution config with default setting
Source
pub fn from_env() -> Result<SessionConfig, DataFusionError>

Create an execution config with config options read from the environment
Source
pub fn from_string_hash_map(
    settings: &HashMap<String, String>,
) -> Result<SessionConfig, DataFusionError>

Create new ConfigOptions struct, taking values from a string hash map.
Source
pub fn options(&self) -> &ConfigOptions

Return a handle to the configuration options.

Can be used to read the current configuration.

use datafusion_execution::config::SessionConfig;


let config = SessionConfig::new();

assert!(config.options().execution.batch_size > 0);

Source
pub fn options_mut(&mut self) -> &mut ConfigOptions

Return a mutable handle to the configuration options.

Can be used to set configuration options.

use datafusion_execution::config::SessionConfig;


let mut config = SessionConfig::new();

config.options_mut().execution.batch_size = 1024;

assert_eq!(config.options().execution.batch_size, 1024);

Source
pub fn set(self, key: &str, value: &ScalarValue) -> SessionConfig

Set a configuration option
Source
pub fn set_bool(self, key: &str, value: bool) -> SessionConfig

Set a boolean configuration option
Source
pub fn set_u64(self, key: &str, value: u64) -> SessionConfig

Set a generic u64 configuration option
Source
pub fn set_usize(self, key: &str, value: usize) -> SessionConfig

Set a generic usize configuration option
Source
pub fn set_str(self, key: &str, value: &str) -> SessionConfig

Set a generic str configuration option
Source
pub fn with_batch_size(self, n: usize) -> SessionConfig

Customize batch size
Source
pub fn with_target_partitions(self, n: usize) -> SessionConfig

Customize target_partitions
Source
pub fn with_option_extension<T>(self, extension: T) -> SessionConfig
where
    T: ConfigExtension,

Insert new ConfigExtension
Source
pub fn target_partitions(&self) -> usize

Get target_partitions
Source
pub fn information_schema(&self) -> bool

Is the information schema enabled?
Source
pub fn create_default_catalog_and_schema(&self) -> bool

Should the context create the default catalog and schema?
Source
pub fn repartition_joins(&self) -> bool

Are joins repartitioned during execution?
Source
pub fn repartition_aggregations(&self) -> bool

Are aggregates repartitioned during execution?
Source
pub fn repartition_window_functions(&self) -> bool

Are window functions repartitioned during execution?
Source
pub fn repartition_sorts(&self) -> bool

Do we execute sorts in a per-partition fashion and merge afterwards, or do we coalesce partitions first and sort globally?
Source
pub fn prefer_existing_sort(&self) -> bool

Prefer existing sort (true) or maximize parallelism (false). See prefer_existing_sort for more details
Source
pub fn collect_statistics(&self) -> bool

Are statistics collected during execution?
Source
pub fn with_default_catalog_and_schema(
    self,
    catalog: impl Into<String>,
    schema: impl Into<String>,
) -> SessionConfig

Selects a name for the default catalog and schema
Source
pub fn with_create_default_catalog_and_schema(
    self,
    create: bool,
) -> SessionConfig

Controls whether the default catalog and schema will be automatically created
Source
pub fn with_information_schema(self, enabled: bool) -> SessionConfig

Enables or disables the inclusion of information_schema virtual tables
Source
pub fn with_repartition_joins(self, enabled: bool) -> SessionConfig

Enables or disables the use of repartitioning for joins to improve parallelism
Source
pub fn with_repartition_aggregations(self, enabled: bool) -> SessionConfig

Enables or disables the use of repartitioning for aggregations to improve parallelism
Source
pub fn with_repartition_file_min_size(self, size: usize) -> SessionConfig

Sets minimum file range size for repartitioning scans
Source
pub fn with_allow_symmetric_joins_without_pruning(
    self,
    enabled: bool,
) -> SessionConfig

Enables or disables the allowing unordered symmetric hash join
Source
pub fn with_repartition_file_scans(self, enabled: bool) -> SessionConfig

Enables or disables the use of repartitioning for file scans
Source
pub fn with_repartition_windows(self, enabled: bool) -> SessionConfig

Enables or disables the use of repartitioning for window functions to improve parallelism
Source
pub fn with_repartition_sorts(self, enabled: bool) -> SessionConfig

Enables or disables the use of per-partition sorting to improve parallelism
Source
pub fn with_prefer_existing_sort(self, enabled: bool) -> SessionConfig

Prefer existing sort (true) or maximize parallelism (false). See prefer_existing_sort for more details
Source
pub fn with_prefer_existing_union(self, enabled: bool) -> SessionConfig

Prefer existing union (true). See prefer_existing_union for more details
Source
pub fn with_parquet_pruning(self, enabled: bool) -> SessionConfig

Enables or disables the use of pruning predicate for parquet readers to skip row groups
Source
pub fn parquet_pruning(&self) -> bool

Returns true if pruning predicate should be used to skip parquet row groups
Source
pub fn parquet_bloom_filter_pruning(&self) -> bool

Returns true if bloom filter should be used to skip parquet row groups
Source
pub fn with_parquet_bloom_filter_pruning(self, enabled: bool) -> SessionConfig

Enables or disables the use of bloom filter for parquet readers to skip row groups
Source
pub fn parquet_page_index_pruning(&self) -> bool

Returns true if page index should be used to skip parquet data pages
Source
pub fn with_parquet_page_index_pruning(self, enabled: bool) -> SessionConfig

Enables or disables the use of page index for parquet readers to skip parquet data pages
Source
pub fn with_collect_statistics(self, enabled: bool) -> SessionConfig

Enables or disables the collection of statistics after listing files
Source
pub fn batch_size(&self) -> usize

Get the currently configured batch size
Source
pub fn with_coalesce_batches(self, enabled: bool) -> SessionConfig

Enables or disables the coalescence of small batches into larger batches
Source
pub fn coalesce_batches(&self) -> bool

Returns true if record batches will be examined between each operator and small batches will be coalesced into larger batches.
Source
pub fn with_round_robin_repartition(self, enabled: bool) -> SessionConfig

Enables or disables the round robin repartition for increasing parallelism
Source
pub fn round_robin_repartition(&self) -> bool

Returns true if the physical plan optimizer will try to add round robin repartition to increase parallelism to leverage more CPU cores.
Source
pub fn with_sort_spill_reservation_bytes(
    self,
    sort_spill_reservation_bytes: usize,
) -> SessionConfig

Set the size of sort_spill_reservation_bytes to control memory pre-reservation
Source
pub fn with_sort_in_place_threshold_bytes(
    self,
    sort_in_place_threshold_bytes: usize,
) -> SessionConfig

Set the size of sort_in_place_threshold_bytes to control how sort does things.
Source
pub fn with_enforce_batch_size_in_joins(
    self,
    enforce_batch_size_in_joins: bool,
) -> SessionConfig

Enables or disables the enforcement of batch size in joins
Source
pub fn enforce_batch_size_in_joins(&self) -> bool

Returns true if the joins will be enforced to output batches of the configured size
Source
pub fn to_props(&self) -> HashMap<String, String>

Convert configuration options to name-value pairs with values converted to strings.

Note that this method will eventually be deprecated and replaced by options.
Source
pub fn with_extension<T>(self, ext: Arc<T>) -> SessionConfig
where
    T: Send + Sync + 'static,

Add extensions.

Extensions can be used to attach extra data to the session config – e.g. tracing information or caches. Extensions are opaque and the types are unknown to DataFusion itself, which makes them extremely flexible. 1

Extensions are stored within an Arc so they do NOT require Clone. The are immutable. If you need to modify their state over their lifetime – e.g. for caches – you need to establish some for of interior mutability.

Extensions are indexed by their type T. If multiple values of the same type are provided, only the last one will be kept.

You may use get_extension to retrieve extensions.
Example

use std::sync::Arc;

use datafusion_execution::config::SessionConfig;


// application-specific extension types

struct Ext1(u8);

struct Ext2(u8);

struct Ext3(u8);


let ext1a = Arc::new(Ext1(10));

let ext1b = Arc::new(Ext1(11));

let ext2 = Arc::new(Ext2(2));


let cfg = SessionConfig::default()

    // will only remember the last Ext1

    .with_extension(Arc::clone(&ext1a))

    .with_extension(Arc::clone(&ext1b))

    .with_extension(Arc::clone(&ext2));


let ext1_received = cfg.get_extension::<Ext1>().unwrap();

assert!(!Arc::ptr_eq(&ext1_received, &ext1a));

assert!(Arc::ptr_eq(&ext1_received, &ext1b));


let ext2_received = cfg.get_extension::<Ext2>().unwrap();

assert!(Arc::ptr_eq(&ext2_received, &ext2));


assert!(cfg.get_extension::<Ext3>().is_none());

    Compare that to ConfigOptions which only supports ScalarValue payloads. ↩

Source
pub fn set_extension<T>(&mut self, ext: Arc<T>)
where
    T: Send + Sync + 'static,

Set extension. Pretty much the same as with_extension, but take mutable reference instead of owning it. Useful if you want to add another extension after the SessionConfig is created.
Example

use std::sync::Arc;

use datafusion_execution::config::SessionConfig;


// application-specific extension types

struct Ext1(u8);

struct Ext2(u8);

struct Ext3(u8);


let ext1a = Arc::new(Ext1(10));

let ext1b = Arc::new(Ext1(11));

let ext2 = Arc::new(Ext2(2));


let mut cfg = SessionConfig::default();


// will only remember the last Ext1

cfg.set_extension(Arc::clone(&ext1a));

cfg.set_extension(Arc::clone(&ext1b));

cfg.set_extension(Arc::clone(&ext2));


let ext1_received = cfg.get_extension::<Ext1>().unwrap();

assert!(!Arc::ptr_eq(&ext1_received, &ext1a));

assert!(Arc::ptr_eq(&ext1_received, &ext1b));


let ext2_received = cfg.get_extension::<Ext2>().unwrap();

assert!(Arc::ptr_eq(&ext2_received, &ext2));


assert!(cfg.get_extension::<Ext3>().is_none());

Source
pub fn get_extension<T>(&self) -> Option<Arc<T>>
where
    T: Send + Sync + 'static,

Get extension, if any for the specified type T exists.

See with_extension on how to add attach extensions.
