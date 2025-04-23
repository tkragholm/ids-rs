use indicatif::ProgressStyle;
use types::models::CovariateType;

/// Create a progress style with a custom template and dynamic covariate type display
pub fn create_progress_style(covariate_type: CovariateType) -> ProgressStyle {
    ProgressStyle::default_bar()
        .template(
            "{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} \
             ({percent}%) {msg}\n\
             ⏱️  ETA: {eta_precise:.dim} | 🚀 {per_sec:.green} records/sec | \
             📊 Processing: {covariate_type}",
        )
        .unwrap()
        .with_key(
            "covariate_type",
            move |_state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                write!(w, "{:?}", covariate_type).unwrap()
            },
        )
}
