use statrs::statistics::Statistics;

pub(crate) struct StatisticalSummary {
    pub mean: f64,
    pub variance: f64,
}

pub(crate) struct StatisticalCalculations;

impl StatisticalCalculations {
    pub fn calculate_summary(values: &[f64]) -> StatisticalSummary {
        StatisticalSummary {
            mean: values.mean(),
            variance: values.variance(),
        }
    }

    pub fn calculate_standardized_difference_from_summaries(
        case_summary: &StatisticalSummary,
        control_summary: &StatisticalSummary,
    ) -> f64 {
        let pooled_sd = ((case_summary.variance + control_summary.variance) / 2.0).sqrt();
        (case_summary.mean - control_summary.mean) / pooled_sd
    }

    pub fn calculate_variance_ratio_from_summaries(
        case_summary: &StatisticalSummary,
        control_summary: &StatisticalSummary,
    ) -> f64 {
        case_summary.variance / control_summary.variance
    }
}
