use crate::error::Result;
use crate::models::family::relations::FamilyRelations;

// Conditionally import ArrowValue based on feature
#[cfg(feature = "arrow-integration")]
use crate::storage::arrow::values::ArrowValue;
#[cfg(not(feature = "arrow-integration"))]
use crate::models::TimeVaryingValue as ArrowValue;

/// Core trait for accessing data from a backend store
///
/// This trait defines the fundamental operations for retrieving data from
/// any storage backend. Implementations are responsible for providing efficient
/// access to stored data with proper error handling.
pub trait Store {
    /// Retrieves data for a specific year and field
    ///
    /// # Arguments
    /// * `year` - The year for which to retrieve data
    /// * `field` - The field name to retrieve
    ///
    /// # Returns
    /// * `Result<ArrowValue>` - The data wrapped in an ArrowValue or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The data does not exist for the given year/field
    /// - There was a problem accessing the data
    /// - The data could not be converted to an ArrowValue
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::prelude::*;
    /// # fn example() -> Result<()> {
    /// # let store = DataStore::new();
    /// // Access data for 2020 employment status
    /// let employment_data = store.data(2020, "employment_status")?;
    /// # Ok(())
    /// # }
    /// ```
    fn data(&self, year: i32, field: &str) -> Result<ArrowValue>;
    
    /// Checks if data exists for a specific year and field
    ///
    /// # Arguments
    /// * `year` - The year to check
    /// * `field` - The field name to check
    ///
    /// # Returns
    /// * `bool` - True if the data exists, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::prelude::*;
    /// # let store = DataStore::new();
    /// // Check if 2020 employment data exists
    /// if store.has_data(2020, "employment_status") {
    ///     println!("Data exists for 2020 employment status");
    /// }
    /// ```
    fn has_data(&self, year: i32, field: &str) -> bool;
    
    /// Returns all years available in the store
    ///
    /// # Returns
    /// * `Vec<i32>` - List of years for which data is available
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::prelude::*;
    /// # let store = DataStore::new();
    /// // Get all available years
    /// let available_years = store.years();
    /// println!("Data available for years: {:?}", available_years);
    /// ```
    fn years(&self) -> Vec<i32>;
    
    /// Returns all fields available in the store
    ///
    /// # Returns
    /// * `Vec<String>` - List of field names available in the store
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::prelude::*;
    /// # let store = DataStore::new();
    /// // Get all available fields
    /// let available_fields = store.fields();
    /// println!("Available fields: {:?}", available_fields);
    /// ```
    fn fields(&self) -> Vec<String>;
    
    /// Returns family relations if available
    ///
    /// # Returns
    /// * `Option<&FamilyRelations>` - Family relations data if available, None otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// # use types::prelude::*;
    /// # let store = DataStore::new();
    /// // Access family relations data
    /// if let Some(family_data) = store.family_relations() {
    ///     println!("Family data available with {} families", family_data.count());
    /// }
    /// ```
    fn family_relations(&self) -> Option<&FamilyRelations>;
}

/// Backend implementation marker trait
///
/// This trait serves as a marker for types that implement the `Store` trait
/// and can be used as a backend in the data store. This allows for type-safe
/// extensions of backend functionality.
pub trait Backend: Store {}

// Automatically implement Backend for any type that implements Store
impl<T: Store> Backend for T {}

/// Legacy compatibility trait for the Store interface
///
/// This trait provides backward compatibility for code that uses the old
/// method names with `get_` prefixes. New code should use the Store trait
/// methods directly without the `get_` prefix.
///
/// This trait is automatically implemented for any type that implements Store.
#[doc(hidden)]
#[deprecated(
    since = "0.2.0",
    note = "Use Store trait methods without 'get_' prefix instead"
)]
pub trait LegacyStore: Store {
    /// Legacy method - use `data` instead
    #[deprecated(since = "0.2.0", note = "Use `data` method instead")]
    fn get_data(&self, year: i32, field: &str) -> Result<ArrowValue> {
        self.data(year, field)
    }
    
    /// Legacy method - use `years` instead
    #[deprecated(since = "0.2.0", note = "Use `years` method instead")]
    fn get_years(&self) -> Vec<i32> {
        self.years()
    }
    
    /// Legacy method - use `fields` instead
    #[deprecated(since = "0.2.0", note = "Use `fields` method instead")]
    fn get_fields(&self) -> Vec<String> {
        self.fields()
    }
    
    /// Legacy method - use `family_relations` instead
    #[deprecated(since = "0.2.0", note = "Use `family_relations` method instead")]
    fn get_family_relations(&self) -> Option<&FamilyRelations> {
        self.family_relations()
    }
}

// Automatically implement LegacyStore for any type that implements Store
#[allow(deprecated)]
impl<T: Store> LegacyStore for T {}