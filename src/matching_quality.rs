use plotters::prelude::*;
use std::error::Error;

pub struct MatchingQuality {
    pub total_cases: usize,
    pub matched_cases: usize,
    pub total_controls: usize,
    pub avg_controls_per_case: f64,
    pub birth_date_differences: Vec<i64>,
    pub mother_age_differences: Vec<i64>,
    pub father_age_differences: Vec<i64>,
    pub matching_rate: f64,
    pub birth_date_balance: f64,
    pub parent_age_balance: f64,
    pub birth_date_percentiles: Vec<i64>,
    pub mother_age_percentiles: Vec<i64>,
    pub father_age_percentiles: Vec<i64>,
}

impl MatchingQuality {
    pub fn calculate_percentiles(values: &[i64], percentiles: &[f64]) -> Vec<i64> {
        let mut sorted_values = values.to_vec();
        sorted_values.sort_unstable();

        percentiles
            .iter()
            .map(|&p| {
                let idx = (p * (sorted_values.len() - 1) as f64).round() as usize;
                sorted_values[idx]
            })
            .collect()
    }

    pub fn plot_distributions(&self, output_file: &str) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let max_diff = self
            .birth_date_differences
            .iter()
            .max()
            .copied()
            .unwrap_or(0);

        const N_BINS: usize = 50;
        let bin_size = (max_diff as f64 / N_BINS as f64).ceil() as i64;

        let mut histogram_data = [0; N_BINS];
        for &diff in &self.birth_date_differences {
            let bin = ((diff as f64 / bin_size as f64).floor() as usize).min(N_BINS - 1);
            histogram_data[bin] += 1;
        }

        let max_count = *histogram_data.iter().max().unwrap_or(&1) as f64;

        let mut chart = ChartBuilder::on(&root)
            .caption("Distribution of Birth Date Differences", ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..N_BINS, 0f64..max_count * 1.1)?;

        chart
            .configure_mesh()
            .x_desc("Difference in Days")
            .y_desc("Frequency")
            .draw()?;

        chart.draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
            let x0 = i;
            let x1 = i + 1;
            let y0 = 0.0;
            let y1 = count as f64;

            Rectangle::new([(x0, y0), (x1, y1)], RED.mix(0.3).filled())
        }))?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        let mean = self.birth_date_differences.iter().sum::<i64>() as f64
            / self.birth_date_differences.len() as f64;

        chart.draw_series(vec![Text::new(
            format!("Mean: {:.1} days", mean),
            (5, (max_count * 0.9)),
            ("sans-serif", 20).into_font(),
        )])?;

        root.present()?;

        Ok(())
    }

    pub fn plot_all_distributions(&self, base_filename: &str) -> Result<(), Box<dyn Error>> {
        self.plot_distribution(
            &self.birth_date_differences,
            &format!("{}_birth.png", base_filename),
            "Birth Date Differences",
            "Difference in Days",
        )?;

        self.plot_distribution(
            &self.mother_age_differences,
            &format!("{}_mother.png", base_filename),
            "Mother Age Differences",
            "Difference in Days",
        )?;

        self.plot_distribution(
            &self.father_age_differences,
            &format!("{}_father.png", base_filename),
            "Father Age Differences",
            "Difference in Days",
        )?;

        Ok(())
    }

    fn plot_distribution(
        &self,
        data: &[i64],
        filename: &str,
        title: &str,
        x_label: &str,
    ) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

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
            .build_cartesian_2d(0..N_BINS, 0f64..max_count * 1.1)?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc("Frequency")
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        chart.draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
            Rectangle::new([(i, 0.0), (i + 1, count as f64)], RED.mix(0.3).filled())
        }))?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        let mean = data.iter().sum::<i64>() as f64 / data.len() as f64;
        chart.draw_series(vec![Text::new(
            format!("Mean: {:.1} days", mean),
            (5, (max_count * 0.9)),
            ("sans-serif", 20).into_font(),
        )])?;

        root.present()?;

        Ok(())
    }

    pub fn format_report(&self) -> String {
        use colored::*;

        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "Matching Quality Report".bold().green()));
        report.push_str(&format!(
            "│ {} {}/{} ({:.1}%)\n",
            "Matching Rate:".bold(),
            self.matched_cases,
            self.total_cases,
            self.matching_rate * 100.0
        ));

        report.push_str(&format!(
            "│ {} {:.2}\n",
            "Average Controls per Case:".bold(),
            self.avg_controls_per_case
        ));

        report.push_str("\nPercentiles (Birth Date Differences):\n");
        report.push_str(&format!(
            "  25th: {} days\n",
            self.birth_date_percentiles[0]
        ));
        report.push_str(&format!(
            "  50th: {} days\n",
            self.birth_date_percentiles[1]
        ));
        report.push_str(&format!(
            "  75th: {} days\n",
            self.birth_date_percentiles[2]
        ));

        report.push_str("\nBalance Metrics:\n");
        report.push_str(&format!(
            "  Birth Date Balance: {:.3}\n",
            self.birth_date_balance
        ));
        report.push_str(&format!(
            "  Parent Age Balance: {:.3}\n",
            self.parent_age_balance
        ));

        report
    }
}
