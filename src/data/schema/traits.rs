use arrow::datatypes::{Schema, SchemaRef};
use std::sync::Arc;

/// Registry schema trait that defines schema behaviors
pub trait RegistrySchema: 'static + Send + Sync {
    /// Get the Arrow schema for this registry
    fn schema() -> Schema where Self: Sized;

    /// Get the Arrow schema for this registry as an Arc
    #[must_use] fn schema_arc() -> SchemaRef where Self: Sized {
        Arc::new(Self::schema())
    }

    /// Get the schema with additional metadata
    #[must_use] fn schema_with_metadata() -> Schema where Self: Sized {
        let schema = Self::schema();
        schema.clone().with_metadata(Self::default_metadata());
        schema
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> where Self: Sized;

    /// Get default metadata for this schema
    #[must_use] fn default_metadata() -> std::collections::HashMap<String, String> where Self: Sized {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata
    }
}

pub struct DynamicSchema {
    schema: SchemaRef,
    column_names: Vec<String>,
}

impl DynamicSchema {
    #[must_use] pub fn new(schema: Schema) -> Self {
        let column_names = schema.fields().iter().map(|f| f.name().clone()).collect();
        Self {
            schema: Arc::new(schema),
            column_names,
        }
    }
    #[must_use] pub fn schema_arc(&self) -> SchemaRef {
        self.schema.clone()
    }

    #[must_use] pub fn column_names(&self) -> Vec<String> {
        self.column_names.clone()
    }
}

impl RegistrySchema for DynamicSchema {
    fn schema() -> Schema {
        panic!("Use schema_arc() instance method instead")
    }

    fn schema_arc() -> SchemaRef {
        panic!("Use schema_arc() instance method instead")
    }

    fn column_names() -> Vec<&'static str> {
        panic!("Use column_names() instance method instead")
    }
}
