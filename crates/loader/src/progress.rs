use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Progress tracking for data loading operations
pub struct LoaderProgress {
    multi_progress: MultiProgress,
    main_pb: ProgressBar,
    sub_pb: Option<ProgressBar>,
}

impl Default for LoaderProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl LoaderProgress {
    /// Create a new progress tracker
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        let main_style = ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .expect("Failed to create progress bar template - this is a static template that should never fail");

        let main_pb = multi_progress.add(ProgressBar::new(5));
        main_pb.set_style(main_style);
        main_pb.set_prefix("Overall Progress");

        Self {
            multi_progress,
            main_pb,
            sub_pb: None,
        }
    }

    /// Create a progress bar for tracking file loading progress
    pub fn create_file_progress(&self, size: u64, filename: &str) -> ProgressBar {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {bytes}/{total_bytes} ({percent}%) {msg}")
                .expect("Failed to create progress bar template - this is a static template that should never fail")
                .progress_chars("█▇▆▅▄▃▂▁  ");

        let pb = self.multi_progress.add(ProgressBar::new(size));
        pb.set_style(style);
        pb.set_prefix(filename.to_string());
        pb
    }

    /// Start a sub-progress tracker for a specific operation
    pub fn start_sub_progress(&mut self, total: u64, prefix: String) {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} ({percent}%) {msg}")
                .expect("Failed to create progress bar template - this is a static template that should never fail");

        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(style);
        pb.set_prefix(prefix);
        self.sub_pb = Some(pb);
    }

    /// Increment the main progress bar
    pub fn increment_main(&self) {
        self.main_pb.inc(1);
    }

    /// Increment the sub-progress bar if available
    pub fn increment_sub(&self) {
        if let Some(pb) = &self.sub_pb {
            pb.inc(1);
        }
    }
    
    /// Get a reference to the main progress bar
    pub fn main_progress_bar(&self) -> &ProgressBar {
        &self.main_pb
    }
    
    /// Get a reference to the sub progress bar
    pub fn sub_progress_bar(&self) -> Option<&ProgressBar> {
        self.sub_pb.as_ref()
    }

    /// Finish the main progress bar with a completion message
    pub fn finish_main(&self, msg: &str) {
        self.main_pb.finish_with_message(msg.to_string());
    }
    
    /// Creates a progress bar for tracking parallel operations
    pub fn create_main_progress(&self, total: u64, operation_name: String) -> ProgressBar {
        let style = ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.green/blue} {pos}/{len} ({percent}%) {msg}")
            .expect("Failed to create progress bar template - this is a static template that should never fail");

        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(style);
        pb.set_prefix(operation_name);
        pb
    }
    
    /// Create a spinner for operations with unknown duration
    pub fn start_with_spinner(&self, message: String) -> ProgressBar {
        let spinner_style = ProgressStyle::default_spinner()
            .template("{spinner:.green} {prefix:.bold.dim} [{elapsed_precise}] {msg}")
            .expect("Failed to create spinner template - this is a static template that should never fail");
            
        let spinner = self.multi_progress.add(ProgressBar::new_spinner());
        spinner.set_style(spinner_style);
        spinner.set_prefix(message);  // String implements Into<Cow<'static, str>> so this is safe
        spinner.set_message("Processing...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));
        
        spinner
    }
}