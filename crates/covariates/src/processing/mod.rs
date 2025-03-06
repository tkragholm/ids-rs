pub mod processor;
pub mod factory;
pub mod demographic;
pub mod education;
pub mod income;
pub mod occupation;

// Re-export the main processor interfaces for convenience
pub use processor::{ConfigurableProcessor, ConfigurableVariableProcessor};
pub use factory::ProcessorFactory;

// Re-export the specific processors
pub use demographic::DemographicsProcessor;
pub use education::EducationProcessor;
pub use income::IncomeProcessor;
pub use occupation::OccupationProcessor;