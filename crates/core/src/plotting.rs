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

        let max_count = f64::from(*histogram_data.iter().max().unwrap_or(&1));

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
                Rectangle::new([(i, 0.0), (i + 1, f64::from(count))], RED.mix(0.3).filled())
            }))
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        let mean = data.iter().sum::<i64>() as f64 / data.len() as f64;
        chart
            .draw_series(vec![Text::new(
                format!("Mean: {mean:.1} days"),
                (5, max_count * 0.9),
                ("sans-serif", 20).into_font(),
            )])
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;
        Ok(())
    }

    fn plot_utilization_summary(
        &self,
        output_file: &str,
        utilization_rate: f64,
        average_reuse: f64,
    ) -> Result<(), PlottingError> {
        let root = BitMapBackend::new(output_file, (600, 400)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Define data for a simple bar chart
        let data = [(0, utilization_rate), (1, average_reuse)];
        let max_value = 1.0f64.max(data.iter().map(|(_, v)| *v).fold(f64::NAN, f64::max));

        let mut chart = ChartBuilder::on(&root)
            .caption("Utilization Summary", ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(
                0..2,  // Using a simpler range to avoid segmented issues
                0f64..max_value * 1.1,
            )
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .y_desc("Rate")
            .draw()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Draw bars without type conversion issues
        chart
            .draw_series(data.iter().map(|(x, y)| {
                let color = if *x == 0 { GREEN.mix(0.5) } else { BLUE.mix(0.5) };
                Rectangle::new([(*x, 0.0), (*x + 1, *y)], color.filled())
            }))
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        // Add labels
        let labels = ["Utilization", "Avg. Reuse"];
        for (i, (&(x, y), label)) in data.iter().zip(labels.iter()).enumerate() {
            chart
                .draw_series(std::iter::once(Text::new(
                    format!("{label}: {y:.2}"),
                    (x, max_value * (0.9 - 0.1 * i as f64)),
                    ("sans-serif", 15),
                )))
                .map_err(|e| PlottingError::plotting(e.to_string()))?;
        }

        chart
            .configure_series_labels()
            .border_style(BLACK)
            .background_style(WHITE.mix(0.8))
            .draw()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        root.present()
            .map_err(|e| PlottingError::plotting(e.to_string()))?;

        Ok(())
    }
}