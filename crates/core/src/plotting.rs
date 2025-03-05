use crate::errors::PlottingError;
use plotters::prelude::*;
use std::fmt::Debug;

pub trait Plottable: Debug {
    fn plot_distribution(
        &self,
        data: &[i64],
        filename: &str,
        title: &str,
        x_label: &str,
    ) -> Result<(), PlottingError>;

    fn plot_utilization_summary(
        &self,
        output_file: &str,
        utilization_rate: f64,
        average_reuse: f64,
    ) -> Result<(), PlottingError>;
    
    fn plot_matching_stats(
        &self,
        output_file: &str,
        matched_count: usize,
        unmatched_count: usize,
        avg_controls: f64,
    ) -> Result<(), PlottingError>;
    
    fn plot_matched_pairs_summary(
        &self,
        output_file: &str,
        birth_differences: &[i64],
        mother_age_differences: &[i64],
        father_age_differences: &[i64],
    ) -> Result<(), PlottingError>;
}

#[derive(Debug)]
pub struct DefaultPlotter;

impl Default for DefaultPlotter {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultPlotter {
    #[must_use] pub const fn new() -> Self {
        Self
    }
}

impl Plottable for DefaultPlotter {
    fn plot_distribution(
        &self,
        data: &[i64],
        filename: &str,
        title: &str,
        x_label: &str,
    ) -> Result<(), PlottingError> {
        // Create a simple histogram using the basic plotters functionality
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        let max_diff = data.iter().max().copied().unwrap_or(0);
        const N_BINS: usize = 50;
        let bin_size = (max_diff as f64 / N_BINS as f64).ceil() as i64;

        let mut histogram_data = [0; N_BINS];
        for &value in data {
            let bin = ((value as f64 / bin_size as f64).floor() as usize).min(N_BINS - 1);
            histogram_data[bin] += 1;
        }

        let max_count = *histogram_data.iter().max().unwrap_or(&1) as f64;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..N_BINS, 0f64..max_count * 1.1)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc("Frequency")
            .draw()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        chart
            .draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
                Rectangle::new([(i, 0.0), (i + 1, count as f64)], RED.mix(0.3).filled())
            }))
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;
        Ok(())
    }

    // Simplified implementation for now - just create a text summary
    fn plot_utilization_summary(
        &self,
        output_file: &str,
        utilization_rate: f64,
        average_reuse: f64,
    ) -> Result<(), PlottingError> {
        let root = BitMapBackend::new(output_file, (800, 400)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Draw title
        root.draw_text(
            "Utilization Summary",
            &TextStyle::from(("sans-serif", 30).into_font()),
            (20, 30),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Draw statistics as text
        let stats = [
            format!("Utilization Rate: {:.2}%", utilization_rate * 100.0),
            format!("Average Control Reuse: {:.2} times", average_reuse),
        ];

        for (i, stat) in stats.iter().enumerate() {
            root.draw_text(
                stat,
                &TextStyle::from(("sans-serif", 20).into_font()),
                (40, 100 + i as i32 * 40),
            ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        }

        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;
        Ok(())
    }
    
    // Simplified implementation
    fn plot_matching_stats(
        &self,
        output_file: &str,
        matched_count: usize,
        unmatched_count: usize,
        avg_controls: f64,
    ) -> Result<(), PlottingError> {
        // Just save a summary text file for now - less prone to errors
        let root = BitMapBackend::new(output_file, (800, 400)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        let total = matched_count + unmatched_count;
        let matched_pct = matched_count as f64 / total as f64;
        
        // Draw a simple text report
        root.draw_text(
            &"Matching Statistics Summary".to_string(),
            &TextStyle::from(("sans-serif", 30).into_font()),
            (20, 30),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        // Display stats as text
        let stats = [
            format!("Total Cases: {}", total),
            format!("Matched Cases: {} ({:.1}%)", matched_count, matched_pct * 100.0),
            format!("Unmatched Cases: {} ({:.1}%)", unmatched_count, (1.0 - matched_pct) * 100.0),
            format!("Avg. Controls per Case: {:.2}", avg_controls),
        ];

        for (i, stat) in stats.iter().enumerate() {
            root.draw_text(
                stat,
                &TextStyle::from(("sans-serif", 20).into_font()),
                (40, 100 + i as i32 * 40),
            ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        }

        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;
        Ok(())
    }
    
    // Simplified implementation
    fn plot_matched_pairs_summary(
        &self,
        output_file: &str,
        birth_differences: &[i64],
        mother_age_differences: &[i64],
        father_age_differences: &[i64],
    ) -> Result<(), PlottingError> {
        // Simple text summary of the statistics
        let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Helper to calculate basic stats
        let calculate_stats = |data: &[i64]| {
            let mut sorted = data.to_vec();
            sorted.sort_unstable();
            
            let len = sorted.len();
            let median = if len == 0 { 0 } else { sorted[len / 2] };
            
            let mean = if !data.is_empty() {
                data.iter().sum::<i64>() as f64 / data.len() as f64
            } else {
                0.0
            };
            
            let p25 = if !sorted.is_empty() { sorted[(len as f64 * 0.25) as usize] } else { 0 };
            let p75 = if !sorted.is_empty() { sorted[(len as f64 * 0.75) as usize] } else { 0 };
            
            (mean, median, p25, p75)
        };
        
        let birth_stats = calculate_stats(birth_differences);
        let mother_stats = calculate_stats(mother_age_differences);
        let father_stats = calculate_stats(father_age_differences);
        
        // Draw a simple text summary
        root.draw_text(
            &"Matched Pairs Statistics Summary".to_string(),
            &TextStyle::from(("sans-serif", 30).into_font()),
            (20, 30),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        // Draw birth date stats
        root.draw_text(
            "Birth Date Differences:",
            &TextStyle::from(("sans-serif", 25).into_font()),
            (20, 100),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        root.draw_text(
            &format!("Mean: {:.1} days, Median: {} days", birth_stats.0, birth_stats.1),
            &TextStyle::from(("sans-serif", 20).into_font()),
            (40, 140),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        // Draw mother age stats
        root.draw_text(
            "Mother Age Differences:",
            &TextStyle::from(("sans-serif", 25).into_font()),
            (20, 200),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        root.draw_text(
            &format!("Mean: {:.1} days, Median: {} days", mother_stats.0, mother_stats.1),
            &TextStyle::from(("sans-serif", 20).into_font()),
            (40, 240),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        // Draw father age stats
        root.draw_text(
            "Father Age Differences:",
            &TextStyle::from(("sans-serif", 25).into_font()),
            (20, 300),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        root.draw_text(
            &format!("Mean: {:.1} days, Median: {} days", father_stats.0, father_stats.1),
            &TextStyle::from(("sans-serif", 20).into_font()),
            (40, 340),
        ).map_err(|e| PlottingError::plotting(e.to_string()))?;
        
        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;
        Ok(())
    }
}