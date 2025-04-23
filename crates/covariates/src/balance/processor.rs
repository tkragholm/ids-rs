use super::BalanceChecker;
use chrono::NaiveDate;
use types::models::{Covariate, CovariateType};

// Re-export optimization strategy from separate module
pub use super::optimization::OptimizationStrategy;

// Import components from the proc_impl module
use super::proc_impl::categorical::CategoricalProcessor;
use super::proc_impl::config::ProcessorConfig;
use super::proc_impl::numeric::NumericProcessor;

/// Main processor for covariate values
pub struct ValueProcessor {
    config: ProcessorConfig,
    numeric_processor: NumericProcessor,
    categorical_processor: CategoricalProcessor,
}

impl Default for ValueProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueProcessor {
    /// Create a new ValueProcessor with default settings
    pub fn new() -> Self {
        let config = ProcessorConfig::new();
        Self {
            numeric_processor: NumericProcessor::new(config.clone()),
            categorical_processor: CategoricalProcessor::new(config.clone()),
            config,
        }
    }

    /// Create a new ValueProcessor with custom configuration
    #[allow(dead_code)]
    pub fn with_config(
        thread_count: Option<usize>,
        chunk_size_multiplier: Option<usize>,
        optimization_strategy: Option<OptimizationStrategy>,
    ) -> Self {
        let config = ProcessorConfig::with_config(
            thread_count,
            chunk_size_multiplier,
            optimization_strategy,
        );
        Self {
            numeric_processor: NumericProcessor::new(config.clone()),
            categorical_processor: CategoricalProcessor::new(config.clone()),
            config,
        }
    }

    /// Configure optimization strategy
    #[allow(dead_code)]
    pub fn with_optimization_strategy(mut self, strategy: OptimizationStrategy) -> Self {
        self.config = self.config.with_optimization_strategy(strategy);
        self.numeric_processor = NumericProcessor::new(self.config.clone());
        self.categorical_processor = CategoricalProcessor::new(self.config.clone());
        self
    }

    /// Automatically select optimization strategy based on system resources
    #[allow(dead_code)]
    pub fn auto_configure(mut self) -> Self {
        self.config = self.config.auto_configure();
        self.numeric_processor = NumericProcessor::new(self.config.clone());
        self.categorical_processor = CategoricalProcessor::new(self.config.clone());
        self
    }

    /// Collect numeric values for a list of subjects
    /// Delegates to the numeric processor
    pub fn collect_numeric_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<f64>, usize)
    where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        self.numeric_processor
            .collect_numeric_values(subjects, covariate_type, checker, extractor)
    }

    /// Collect categorical values for a list of subjects
    /// Delegates to the categorical processor
    pub fn collect_categorical_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<String>, usize)
    where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        self.categorical_processor.collect_categorical_values(
            subjects,
            covariate_type,
            checker,
            extractor,
        )
    }
}
