use crate::data::registry::traits_proposal::{
    AsyncRegisterLoader, RegisterLoaderImpl, PnrFilter,
};
use crate::error::Result;
use crate::impl_loader_method;
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::path::Path;

/// Example registry implementation 
pub struct ExampleRegister {
    schema: SchemaRef,
}

impl ExampleRegister {
    /// Create a new example register
    pub fn new(schema: SchemaRef) -> Self {
        Self { schema }
    }
}

impl AsyncRegisterLoader for ExampleRegister {
    fn register_name(&self) -> &'static str {
        "EXAMPLE"
    }

    fn get_schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn load<'a>(
        &'a self,
        base_path: &'a str,
        pnr_filter: Option<&'a PnrFilter>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RecordBatch>>> + Send + 'a>> {
        impl_loader_method!(self, default_load, base_path, pnr_filter)
    }

    fn create_context<'a>(
        &'a self,
        base_path: &'a str,
        pnr_filter: Option<&'a PnrFilter>,
    ) -> Pin<Box<dyn Future<Output = Result<SessionContext>> + Send + 'a>> {
        impl_loader_method!(self, default_create_context, base_path, pnr_filter)
    }
}

// Implement the helper trait to get access to default implementations
impl RegisterLoaderImpl for ExampleRegister {
    // Override primary key if needed
    fn get_primary_key(&self) -> &'static str {
        "EXAMPLE_ID" 
    }
}

// Example of how to use the trait object approach
pub async fn example_usage(registry_name: &str, path: &str) -> Result<Vec<RecordBatch>> {
    use crate::data::registry::traits_proposal::RegisterLoaderFactory;
    
    // Get a loader without knowing its concrete type
    let loader = RegisterLoaderFactory::from_name(registry_name)?;
    
    // Use the loader through the trait interface - no downcasting needed!
    loader.load(path, None).await
}