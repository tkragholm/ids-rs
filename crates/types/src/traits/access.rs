use crate::error::Result;
use crate::models::family::relations::FamilyRelations;
use crate::storage::arrow::values::ArrowValue;

/// Core trait for accessing data from a backend store
///
/// This trait defines the fundamental operations for retrieving data from
/// any storage backend. Implementations are responsible for providing efficient
/// access to stored data with proper error handling.
pub trait Store {
    /// Get data for a specific year and field
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
    fn get_data(&self, year: i32, field: &str) -> Result<ArrowValue>;
    
    /// Check if data exists for a specific year and field
    ///
    /// # Arguments
    /// * `year` - The year to check
    /// * `field` - The field name to check
    ///
    /// # Returns
    /// * `bool` - True if the data exists, false otherwise
    fn has_data(&self, year: i32, field: &str) -> bool;
    
    /// Get all years available in the store
    ///
    /// # Returns
    /// * `Vec<i32>` - List of years for which data is available
    fn get_years(&self) -> Vec<i32>;
    
    /// Get all fields available in the store
    ///
    /// # Returns
    /// * `Vec<String>` - List of field names available in the store
    fn get_fields(&self) -> Vec<String>;
    
    /// Get family relations if available
    ///
    /// # Returns
    /// * `Option<&FamilyRelations>` - Family relations data if available, None otherwise
    fn get_family_relations(&self) -> Option<&FamilyRelations>;
}

/// Backend implementation marker trait
///
/// This trait serves as a marker for types that implement the `Store` trait
/// and can be used as a backend in the data store. This allows for type-safe
/// extensions of backend functionality.
pub trait Backend: Store {}

// Automatically implement Backend for any type that implements Store
impl<T: Store> Backend for T {}