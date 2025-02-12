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

impl DefaultPlotter {
    pub const fn new() -> Self {
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
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

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
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc("Frequency")
            .draw()
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        chart
            .draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
                Rectangle::new([(i, 0.0), (i + 1, f64::from(count))], RED.mix(0.3).filled())
            }))
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        let mean = data.iter().sum::<i64>() as f64 / data.len() as f64;
        chart
            .draw_series(vec![Text::new(
                format!("Mean: {mean:.1} days"),
                (5, max_count * 0.9),
                ("sans-serif", 20).into_font(),
            )])
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        root.present()
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;
        Ok(())
    }

    fn plot_utilization_summary(
        &self,
        output_file: &str,
        utilization_rate: f64,
        average_reuse: f64,
    ) -> Result<(), PlottingError> {
        let root = BitMapBackend::new(output_file, (800, 400)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Control Utilization Summary", ("sans-serif", 30))
            .margin(5)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..2, 0f64..1f64)
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        chart
            .configure_mesh()
            .draw()
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        chart
            .draw_series(vec![
                Text::new(
                    format!("Utilization Rate: {:.1}%", utilization_rate * 100.0),
                    (0, 0.8),
                    ("sans-serif", 20).into_font(),
                ),
                Text::new(
                    format!("Average Reuse: {average_reuse:.2}"),
                    (0, 0.6),
                    ("sans-serif", 20).into_font(),
                ),
            ])
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;

        root.present()
            .map_err(|e| PlottingError::PlotError(e.to_string()))?;
        Ok(())
    }
}
