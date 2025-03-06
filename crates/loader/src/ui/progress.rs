use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;

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
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.green/blue} {pos}/{len} ({percent}%) {msg}")
            .expect("Failed to create progress bar template - this is a static template that should never fail");

        let sub_pb = self.multi_progress.add(ProgressBar::new(total));
        sub_pb.set_style(style);
        sub_pb.set_prefix(prefix);
        self.sub_pb = Some(sub_pb);
    }

    /// Start a sub-progress tracker for a specific file
    pub fn start_file_progress(&mut self, path: &Path) -> ProgressBar {
        let filename = path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown");
            
        // Try to get file size for accurate progress
        let file_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(1000);
            
        self.create_file_progress(file_size, filename)
    }

    /// Start a batch progress tracker
    pub fn start_batch_progress(&mut self, total_batches: u64, prefix: &str) {
        let style = ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} batches ({percent}%)")
            .expect("Failed to create progress bar template - this is a static template that should never fail");

        let sub_pb = self.multi_progress.add(ProgressBar::new(total_batches));
        sub_pb.set_style(style);
        sub_pb.set_prefix(prefix.to_string());
        self.sub_pb = Some(sub_pb);
    }
    
    /// Increment the main progress
    pub fn inc_main(&self) {
        self.main_pb.inc(1);
    }
    
    /// Increment the sub progress
    pub fn inc_sub(&self) {
        if let Some(pb) = &self.sub_pb {
            pb.inc(1);
        }
    }
    
    /// Set a message on the main progress
    pub fn set_main_message(&self, msg: &str) {
        self.main_pb.set_message(msg.to_string());
    }
    
    /// Set a message on the sub progress
    pub fn set_sub_message(&self, msg: &str) {
        if let Some(pb) = &self.sub_pb {
            pb.set_message(msg.to_string());
        }
    }
    
    /// Finish the main progress
    pub fn finish_main(&self) {
        self.main_pb.finish_with_message("Complete");
    }
    
    /// Finish the sub progress
    pub fn finish_sub(&self) {
        if let Some(pb) = &self.sub_pb {
            pb.finish();
        }
    }
    
    /// Finish all progress bars
    pub fn finish(&self) {
        self.finish_sub();
        self.finish_main();
    }
}