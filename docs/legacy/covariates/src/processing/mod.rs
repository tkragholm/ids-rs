pub mod demographic;
pub mod education;
pub mod factory;
pub mod income;
pub mod occupation;
pub mod processor;

// Re-export the main processor interfaces for convenience
pub use factory::ProcessorFactory;
pub use processor::{ConfigurableProcessor, ConfigurableVariableProcessor};

// Re-export the specific processors
pub use demographic::DemographicsProcessor;
pub use education::EducationProcessor;
pub use income::IncomeProcessor;
pub use occupation::OccupationProcessor;
