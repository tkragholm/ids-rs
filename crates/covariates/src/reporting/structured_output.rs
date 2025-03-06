use crate::balance::results::BalanceResults;
use crate::data::matched_pairs::record::{CaseWithControls, MatchedPairRecord};
use chrono::Local;
use hashbrown::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use types::error::IdsError;

/// A centralized manager for structured output organization
pub struct StructuredOutputManager {
    /// Root output directory
    base_dir: PathBuf,
    /// Report directory for HTML and human-readable output
    report_dir: PathBuf,
    /// Data directory for raw CSV files
    data_dir: PathBuf,
    /// Plots directory for all visualizations
    plots_dir: PathBuf,
    /// Log directory for all logs
    logs_dir: PathBuf,
    /// Flag to enable detailed debug information in reports
    debug_mode: bool,
    /// Runtime information to include in reports
    runtime_info: HashMap<String, String>,
}

impl StructuredOutputManager {
    /// Create a new structured output manager with the given base directory
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, IdsError> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        
        // Create the main directory structure
        let report_dir = base_dir.join("report");
        let data_dir = base_dir.join("data");
        let plots_dir = base_dir.join("plots");
        let logs_dir = base_dir.join("logs");
        
        // Create additional subdirectories
        let dirs = [
            &report_dir,
            &data_dir,
            &data_dir.join("balance"),
            &data_dir.join("matching"),
            &data_dir.join("register"),
            &plots_dir,
            &plots_dir.join("balance"),
            &plots_dir.join("matching"),
            &plots_dir.join("data_quality"),
            &logs_dir,
        ];
        
        for dir in &dirs {
            fs::create_dir_all(dir).map_err(|e| {
                IdsError::io_error(format!("Failed to create directory {:?}: {}", dir, e))
            })?;
        }
        
        // Initialize runtime information
        let mut runtime_info = HashMap::new();
        runtime_info.insert("generated_at".to_string(), timestamp);
        runtime_info.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        
        Ok(Self {
            base_dir,
            report_dir,
            data_dir,
            plots_dir,
            logs_dir,
            debug_mode: false,
            runtime_info,
        })
    }
    
    /// Enable debug mode with more detailed output
    pub fn with_debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }
    
    /// Add custom runtime information to be included in reports
    pub fn with_runtime_info<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.runtime_info.insert(key.into(), value.into());
        self
    }
    
    /// Get the path to the specified output directory
    pub fn get_dir_path(&self, dir_type: OutputDirType) -> PathBuf {
        match dir_type {
            OutputDirType::Base => self.base_dir.clone(),
            OutputDirType::Report => self.report_dir.clone(),
            OutputDirType::Data => self.data_dir.clone(),
            OutputDirType::Plots => self.plots_dir.clone(),
            OutputDirType::Logs => self.logs_dir.clone(),
            OutputDirType::BalanceData => self.data_dir.join("balance"),
            OutputDirType::MatchingData => self.data_dir.join("matching"),
            OutputDirType::RegisterData => self.data_dir.join("register"),
            OutputDirType::BalancePlots => self.plots_dir.join("balance"),
            OutputDirType::MatchingPlots => self.plots_dir.join("matching"),
            OutputDirType::DataQualityPlots => self.plots_dir.join("data_quality"),
        }
    }
    
    /// Output balance results in structured format
    pub fn output_balance_results(
        &self, 
        results: &BalanceResults,
        filename_prefix: Option<&str>,
    ) -> Result<(), IdsError> {
        // Get the base balance data directory
        let balance_dir = self.get_dir_path(OutputDirType::BalanceData);
        let prefix = filename_prefix.unwrap_or("balance");
        
        // Output covariate balance
        let covariate_path = balance_dir.join(format!("{}_covariate_balance.csv", prefix));
        self.write_csv_data(&covariate_path, "Variable,Mean (Cases),Mean (Controls),Standardized Difference,Variance Ratio", 
            &results.summaries.iter().map(|s| {
                format!("{},{},{},{},{}",
                    s.variable,
                    s.mean_cases,
                    s.mean_controls,
                    s.std_diff,
                    s.variance_ratio
                )
            }).collect::<Vec<_>>()
        )?;
        
        // Output missing data rates
        let missing_path = balance_dir.join(format!("{}_missing_data_rates.csv", prefix));
        let missing_header = "Variable,Case Missing Rate,Control Missing Rate";
        let missing_rates: Vec<String> = results.missing_data_rates.iter()
            .map(|(var, (case_rate, ctrl_rate))| {
                format!("{},{},{}", var, case_rate, ctrl_rate)
            })
            .collect();
        self.write_csv_data(&missing_path, missing_header, &missing_rates)?;
        
        // Generate standardized difference statistics
        let std_diff_path = balance_dir.join(format!("{}_std_differences.csv", prefix));
        let std_diff_header = "Variable,Min,Max,Mean,StdDev,AbsMean";
        let mut var_stats: HashMap<String, Vec<f64>> = HashMap::new();
        
        // Collect by variable
        for detail in &results.matched_pair_details {
            var_stats.entry(detail.variable.clone())
                .or_default()
                .push(detail.std_diff);
        }
        
        // Calculate statistics for each variable
        let std_diff_data: Vec<String> = var_stats.iter()
            .map(|(var, values)| {
                if values.is_empty() {
                    return format!("{},0.0,0.0,0.0,0.0,0.0", var);
                }
                
                let sum: f64 = values.iter().sum();
                let mean = sum / values.len() as f64;
                let sum_squared: f64 = values.iter().map(|v| (v - mean).powi(2)).sum();
                let std_dev = (sum_squared / values.len() as f64).sqrt();
                let abs_mean = values.iter().map(|v| v.abs()).sum::<f64>() / values.len() as f64;
                
                format!("{},{},{},{},{},{}",
                    var,
                    values.iter().cloned().fold(f64::INFINITY, f64::min),
                    values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    mean,
                    std_dev,
                    abs_mean
                )
            })
            .collect();
        self.write_csv_data(&std_diff_path, std_diff_header, &std_diff_data)?;
        
        // Generate HTML report
        self.generate_balance_html_report(results, filename_prefix)?;
        
        Ok(())
    }
    
    /// Output matched pairs data in structured format
    pub fn output_matched_pairs(
        &self,
        matched_pair_records: &[MatchedPairRecord],
        filename_prefix: Option<&str>,
    ) -> Result<(), IdsError> {
        // Convert MatchedPairRecord to CaseWithControls
        let matched_pairs = CaseWithControls::from_matched_pair_records(matched_pair_records);
        let matching_dir = self.get_dir_path(OutputDirType::MatchingData);
        let prefix = filename_prefix.unwrap_or("matching");
        
        // Main matched pairs CSV
        let pairs_path = matching_dir.join(format!("{}_pairs.csv", prefix));
        let pairs_header = "case_id,case_pnr,case_birth_date,case_treatment_date,control_id,control_pnr,control_birth_date,birth_date_diff_days,mother_age_diff_days,father_age_diff_days";
        
        let pairs_data: Vec<String> = matched_pairs.iter()
            .flat_map(|record| {
                record.controls.iter().map(move |control| {
                    format!("{},{},{},{},{},{},{},{},{},{}",
                        record.case_id,
                        record.case_pnr,
                        record.case_birth_date,
                        record.case_treatment_date,
                        control.id,
                        control.pnr,
                        control.birth_date,
                        control.birth_date_diff,
                        control.mother_age_diff.unwrap_or(-1),
                        control.father_age_diff.unwrap_or(-1)
                    )
                })
            })
            .collect();
        
        self.write_csv_data(&pairs_path, pairs_header, &pairs_data)?;
        
        // Matching statistics
        let stats_path = matching_dir.join(format!("{}_stats.csv", prefix));
        let stats_header = "case_id,n_controls,avg_birth_diff,max_birth_diff,avg_mother_diff,avg_father_diff";
        
        let stats_data: Vec<String> = matched_pairs.iter()
            .map(|record| {
                let n_controls = record.controls.len();
                
                if n_controls == 0 {
                    return format!("{},0,0,0,0,0", record.case_id);
                }
                
                let avg_birth_diff: f64 = record.controls.iter()
                    .map(|c| c.birth_date_diff as f64)
                    .sum::<f64>() / n_controls as f64;
                
                let max_birth_diff = record.controls.iter()
                    .map(|c| c.birth_date_diff)
                    .max()
                    .unwrap_or(0);
                
                let avg_mother_diff: f64 = record.controls.iter()
                    .filter_map(|c| c.mother_age_diff)
                    .map(|diff| diff as f64)
                    .sum::<f64>() / n_controls as f64;
                
                let avg_father_diff: f64 = record.controls.iter()
                    .filter_map(|c| c.father_age_diff)
                    .map(|diff| diff as f64)
                    .sum::<f64>() / n_controls as f64;
                
                format!("{},{},{:.2},{},{:.2},{:.2}",
                    record.case_id,
                    n_controls,
                    avg_birth_diff,
                    max_birth_diff,
                    avg_mother_diff,
                    avg_father_diff
                )
            })
            .collect();
        
        self.write_csv_data(&stats_path, stats_header, &stats_data)?;
        
        // Generate HTML report
        self.generate_matching_html_report(&matched_pairs, filename_prefix)?;
        
        Ok(())
    }
    
    /// Generate a comprehensive index.html report
    pub fn generate_index_html(&self) -> Result<(), IdsError> {
        let report_dir = self.get_dir_path(OutputDirType::Report);
        let index_path = report_dir.join("index.html");
        
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>IDS-RS Analysis Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        header {{
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 5px;
        }}
        h1 {{
            color: #2c3e50;
            margin-top: 0;
        }}
        .card {{
            background-color: #fff;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }}
        .info-bar {{
            background-color: #f8f9fa;
            padding: 10px;
            border-radius: 5px;
            margin-bottom: 20px;
            font-size: 0.9em;
        }}
        .section {{
            margin-bottom: 30px;
        }}
        a.report-link {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 10px 15px;
            text-decoration: none;
            border-radius: 5px;
            margin-right: 10px;
            margin-bottom: 10px;
        }}
        a.report-link:hover {{
            background-color: #2980b9;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        th, td {{
            padding: 12px 15px;
            border-bottom: 1px solid #ddd;
            text-align: left;
        }}
        th {{
            background-color: #f2f2f2;
        }}
        tr:hover {{
            background-color: #f5f5f5;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>IDS-RS Analysis Report</h1>
            <div class="info-bar">
                <strong>Generated:</strong> {generated_at} | 
                <strong>Version:</strong> {version}
            </div>
        </header>

        <div class="section">
            <div class="card">
                <h2>Reports</h2>
                <p>Access detailed analysis reports:</p>
                <div>
                    <a href="balance_report.html" class="report-link">Balance Analysis</a>
                    <a href="matching_report.html" class="report-link">Matching Analysis</a>
                    <a href="data_quality_report.html" class="report-link">Data Quality</a>
                </div>
            </div>
        </div>

        <div class="section">
            <div class="card">
                <h2>Analysis Summary</h2>
                <p>Key findings from the analysis:</p>
                <ul>
                    <li>Balance analysis assessed covariate distributions across treatment and control groups</li>
                    <li>Matching analysis evaluated the quality of matched pairs</li>
                    <li>Data quality metrics identified potential issues in the dataset</li>
                </ul>
            </div>
        </div>

        <div class="section">
            <div class="card">
                <h2>Data Files</h2>
                <p>Raw data files available for download:</p>
                <table>
                    <tr>
                        <th>Category</th>
                        <th>Filename</th>
                        <th>Description</th>
                    </tr>
                    <tr>
                        <td>Balance</td>
                        <td><a href="../data/balance/balance_covariate_balance.csv">balance_covariate_balance.csv</a></td>
                        <td>Covariate balance statistics</td>
                    </tr>
                    <tr>
                        <td>Balance</td>
                        <td><a href="../data/balance/balance_missing_data_rates.csv">balance_missing_data_rates.csv</a></td>
                        <td>Missing data rates by variable</td>
                    </tr>
                    <tr>
                        <td>Balance</td>
                        <td><a href="../data/balance/balance_std_differences.csv">balance_std_differences.csv</a></td>
                        <td>Standardized difference statistics</td>
                    </tr>
                    <tr>
                        <td>Matching</td>
                        <td><a href="../data/matching/matching_pairs.csv">matching_pairs.csv</a></td>
                        <td>Matched pair records</td>
                    </tr>
                    <tr>
                        <td>Matching</td>
                        <td><a href="../data/matching/matching_stats.csv">matching_stats.csv</a></td>
                        <td>Matching quality statistics</td>
                    </tr>
                </table>
            </div>
        </div>
    </div>
</body>
</html>"#,
            generated_at = self.runtime_info.get("generated_at").unwrap_or(&String::from("Unknown")),
            version = self.runtime_info.get("version").unwrap_or(&String::from("Unknown"))
        );
        
        fs::write(&index_path, html_content).map_err(|e| {
            IdsError::io_error(format!("Failed to write index.html: {}", e))
        })?;
        
        Ok(())
    }
    
    /// Utility method to write CSV data to a file
    fn write_csv_data<P: AsRef<Path>>(
        &self,
        path: P,
        header: &str,
        data: &[String],
    ) -> Result<(), IdsError> {
        let content = format!("{}\n{}", header, data.join("\n"));
        fs::write(path, content).map_err(|e| {
            IdsError::io_error(format!("Failed to write CSV data: {}", e))
        })
    }
    
    /// Generate a detailed HTML report for balance analysis
    fn generate_balance_html_report(
        &self,
        results: &BalanceResults,
        filename_prefix: Option<&str>,
    ) -> Result<(), IdsError> {
        let report_dir = self.get_dir_path(OutputDirType::Report);
        let prefix = filename_prefix.unwrap_or("balance");
        let report_path = report_dir.join(format!("{}_report.html", prefix));
        
        // Group variables by category
        let mut demographic_rows = String::new();
        let mut income_rows = String::new();
        let mut education_rows = String::new(); 
        let mut occupation_rows = String::new();
        let mut other_rows = String::new();
        
        // Variable tooltips for better explanation
        let variable_tooltips: HashMap<&str, &str> = [
            // Demographics
            ("Family Size", "Number of people in the family unit (ANTPERSF/ANTPERSH)"),
            ("Municipality", "Municipality code (KOM)"),
            ("Family Type", "Type of family unit (FAMILIE_TYPE)"),
            ("Civil Status", "Civil/marital status (CIVST)"),
            ("Gender", "Gender of the individual (KOEN)"),
            ("Citizenship", "Citizenship/nationality (STATSB)"),
            ("Age", "Age of the individual (ALDER)"),
            ("Children Count", "Number of children in the family (ANTBOERNF/ANTBOERNH)"),
            
            // Income
            ("Income", "Total personal income (PERINDKIALT_13)"),
            ("Wage Income", "Income from wages (LOENMV_13)"),
            ("Employment Status", "Employment status code (BESKST13)"),
            ("Employment Status Category", "Employment status category (BESKST13)"),
            
            // Education
            ("Education Level", "Highest education level attained"),
            ("ISCED Level", "International Standard Classification of Education level"),
            ("Education Years", "Years of education completed"),
            
            // Occupation
            ("SOCIO13 Code", "Socioeconomic classification code (SOCIO13)"),
            ("SOCIO13 Value", "Socioeconomic classification numeric value (SOCIO13)"),
            ("Classification System", "Classification system used for occupational coding"),
            ("SOCIO", "Previous socioeconomic classification code"),
            ("SOCIO Category", "Previous socioeconomic classification category"),
            ("SOCIO02", "Alternative socioeconomic classification from 2002"),
            ("SOCIO02 Category", "Alternative socioeconomic classification category from 2002"),
            ("Previous Socioeconomic Status", "Previous socioeconomic status (PRE_SOCIO)"),
            ("Previous Socioeconomic Category", "Previous socioeconomic status category (PRE_SOCIO)"),
        ].iter().cloned().collect();
        
        for summary in &results.summaries {
            // Determine if there's an imbalance (std_diff > 0.1)
            let row_class = if summary.std_diff.abs() > 0.1 {
                "imbalanced"
            } else {
                ""
            };
            
            // Get tooltip for this variable
            let tooltip = variable_tooltips.get(summary.variable.as_str())
                .map_or("", |&s| s);
            
            let row_html = format!(
                r#"<tr class="{}">
                    <td title="{}">{}</td>
                    <td>{:.4}</td>
                    <td>{:.4}</td>
                    <td>{:.4}</td>
                    <td>{:.4}</td>
                </tr>"#,
                row_class,
                tooltip,
                summary.variable,
                summary.mean_cases,
                summary.mean_controls,
                summary.std_diff,
                summary.variance_ratio
            );
            
            // Categorize the row based on variable name
            if summary.variable.contains("Family") || 
               summary.variable.contains("Municipality") ||
               summary.variable.contains("Civil Status") ||
               summary.variable.contains("Gender") ||
               summary.variable.contains("Citizenship") ||
               summary.variable.contains("Age") ||
               summary.variable.contains("Children") {
                demographic_rows.push_str(&row_html);
            } else if summary.variable.contains("Income") ||
                     summary.variable.contains("Employment") {
                income_rows.push_str(&row_html);
            } else if summary.variable.contains("Education") ||
                     summary.variable.contains("ISCED") {
                education_rows.push_str(&row_html);
            } else if summary.variable.contains("SOCIO") ||
                     summary.variable.contains("Classification") ||
                     summary.variable.contains("Socioeconomic") {
                occupation_rows.push_str(&row_html);
            } else {
                other_rows.push_str(&row_html);
            }
        }
        
        // Combine all rows with section headers
        let mut variable_rows = String::new();
        
        if !demographic_rows.is_empty() {
            variable_rows.push_str("<tr class=\"section-header\"><th colspan=\"5\">Demographics Variables</th></tr>");
            variable_rows.push_str(&demographic_rows);
        }
        
        if !income_rows.is_empty() {
            variable_rows.push_str("<tr class=\"section-header\"><th colspan=\"5\">Income Variables</th></tr>");
            variable_rows.push_str(&income_rows);
        }
        
        if !education_rows.is_empty() {
            variable_rows.push_str("<tr class=\"section-header\"><th colspan=\"5\">Education Variables</th></tr>");
            variable_rows.push_str(&education_rows);
        }
        
        if !occupation_rows.is_empty() {
            variable_rows.push_str("<tr class=\"section-header\"><th colspan=\"5\">Occupation Variables</th></tr>");
            variable_rows.push_str(&occupation_rows);
        }
        
        if !other_rows.is_empty() {
            variable_rows.push_str("<tr class=\"section-header\"><th colspan=\"5\">Other Variables</th></tr>");
            variable_rows.push_str(&other_rows);
        }
        
        // Calculate missing data for report
        let mut missing_data_rows = String::new();
        for (var, (case_rate, ctrl_rate)) in &results.missing_data_rates {
            missing_data_rows.push_str(&format!(
                r#"<tr>
                    <td>{}</td>
                    <td>{:.2}%</td>
                    <td>{:.2}%</td>
                </tr>"#,
                var,
                case_rate * 100.0,
                ctrl_rate * 100.0
            ));
        }
        
        // Generate the HTML content
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Balance Analysis Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        header {{
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 5px;
        }}
        h1, h2, h3 {{
            color: #2c3e50;
        }}
        h1 {{
            margin-top: 0;
        }}
        .card {{
            background-color: #fff;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }}
        .info-bar {{
            background-color: #f8f9fa;
            padding: 10px;
            border-radius: 5px;
            margin-bottom: 20px;
            font-size: 0.9em;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 20px;
        }}
        th, td {{
            padding: 12px 15px;
            border-bottom: 1px solid #ddd;
            text-align: left;
        }}
        th {{
            background-color: #f2f2f2;
        }}
        tr:hover {{
            background-color: #f5f5f5;
        }}
        tr.imbalanced {{
            background-color: #fff9c4;
        }}
        tr.imbalanced:hover {{
            background-color: #fff59d;
        }}
        .summary-box {{
            background-color: #f1f8e9;
            border-left: 4px solid #7cb342;
            padding: 10px;
            margin-bottom: 20px;
        }}
        .warning-box {{
            background-color: #ffebee;
            border-left: 4px solid #e57373;
            padding: 10px;
            margin-bottom: 20px;
        }}
        a.nav-link {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 8px 12px;
            text-decoration: none;
            border-radius: 5px;
            margin-right: 10px;
            font-size: 0.9em;
        }}
        a.nav-link:hover {{
            background-color: #2980b9;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Balance Analysis Report</h1>
            <div class="info-bar">
                <strong>Generated:</strong> {generated_at} | 
                <strong>Version:</strong> {version}
                <div style="float: right;">
                    <a href="index.html" class="nav-link">Back to Dashboard</a>
                </div>
            </div>
        </header>

        <div class="card">
            <h2>Covariate Balance Summary</h2>
            <div class="summary-box">
                <p><strong>Analysis Overview:</strong> This report shows the balance of covariates between case and control groups.</p>
                <p><strong>Imbalance Threshold:</strong> Variables with standardized difference > 0.1 are highlighted as potentially imbalanced.</p>
                <p><strong>Register Variables:</strong> The report includes variables from BEF (demographics), IND (income), and AKM (occupation) registers.</p>
            </div>
            
            <div style="background-color: #f8f9fa; padding: 15px; margin: 15px 0; border-radius: 5px;">
                <h3 style="margin-top: 0;">Register Variables Legend</h3>
                <p><strong>Demographics (BEF):</strong> Family Size (ANTPERSF/ANTPERSH), Municipality (KOM), Family Type (FAMILIE_TYPE), Civil Status (CIVST), Gender (KOEN), Citizenship (STATSB), Age (ALDER), Children Count (ANTBOERNF/ANTBOERNH)</p>
                <p><strong>Income (IND):</strong> Total Income (PERINDKIALT_13), Wage Income (LOENMV_13), Employment Status (BESKST13)</p>
                <p><strong>Occupation (AKM):</strong> SOCIO13, SOCIO, SOCIO02, PRE_SOCIO</p>
                <p><em>Hover over variable names in the table below to see details about the register variables they represent.</em></p>
            </div>
            
            <table>
                <thead>
                    <tr>
                        <th>Variable</th>
                        <th>Mean (Cases)</th>
                        <th>Mean (Controls)</th>
                        <th>Standardized Difference</th>
                        <th>Variance Ratio</th>
                    </tr>
                </thead>
                <tbody>
                    {variable_rows}
                </tbody>
            </table>
        </div>

        <div class="card">
            <h2>Missing Data Analysis</h2>
            <table>
                <thead>
                    <tr>
                        <th>Variable</th>
                        <th>Cases Missing</th>
                        <th>Controls Missing</th>
                    </tr>
                </thead>
                <tbody>
                    {missing_data_rows}
                </tbody>
            </table>
        </div>
        
        <div class="card">
            <h2>Data Files</h2>
            <p>Raw data files for further analysis:</p>
            <ul>
                <li><a href="../data/balance/balance_covariate_balance.csv">Covariate Balance Data (CSV)</a></li>
                <li><a href="../data/balance/balance_missing_data_rates.csv">Missing Data Rates (CSV)</a></li>
                <li><a href="../data/balance/balance_std_differences.csv">Standardized Differences (CSV)</a></li>
            </ul>
        </div>
    </div>
</body>
</html>"#,
            generated_at = self.runtime_info.get("generated_at").unwrap_or(&String::from("Unknown")),
            version = self.runtime_info.get("version").unwrap_or(&String::from("Unknown")),
            variable_rows = variable_rows,
            missing_data_rows = missing_data_rows
        );
        
        fs::write(&report_path, html_content).map_err(|e| {
            IdsError::io_error(format!("Failed to write balance HTML report: {}", e))
        })?;
        
        Ok(())
    }
    
    /// Generate a detailed HTML report for matching analysis
    fn generate_matching_html_report(
        &self,
        matched_pairs: &[CaseWithControls],
        filename_prefix: Option<&str>,
    ) -> Result<(), IdsError> {
        let report_dir = self.get_dir_path(OutputDirType::Report);
        let prefix = filename_prefix.unwrap_or("matching");
        let report_path = report_dir.join(format!("{}_report.html", prefix));
        
        // Generate summary statistics
        let total_cases = matched_pairs.len();
        let cases_with_controls = matched_pairs.iter()
            .filter(|r| !r.controls.is_empty())
            .count();
        let total_controls: usize = matched_pairs.iter()
            .map(|r| r.controls.len())
            .sum();
        let avg_controls_per_case = if total_cases > 0 {
            total_controls as f64 / total_cases as f64
        } else {
            0.0
        };
        
        // Calculate birth date difference statistics
        let mut birth_diffs: Vec<i64> = Vec::new();
        for record in matched_pairs.iter() {
            for control in &record.controls {
                birth_diffs.push(control.birth_date_diff);
            }
        }
        
        let avg_birth_diff = if !birth_diffs.is_empty() {
            birth_diffs.iter().sum::<i64>() as f64 / birth_diffs.len() as f64
        } else {
            0.0
        };
        
        let max_birth_diff = birth_diffs.iter().cloned().max().unwrap_or(0);
        
        // Generate case summary rows
        let mut case_summary_rows = String::new();
        for record in matched_pairs.iter().take(100) { // Limit to first 100 for performance
            let controls_count = record.controls.len();
            let control_info = if controls_count > 0 {
                let first_control = &record.controls[0];
                format!(
                    "{} controls, first: {} (diff: {} days)", 
                    controls_count,
                    first_control.pnr,
                    first_control.birth_date_diff
                )
            } else {
                "No controls found".to_string()
            };
            
            case_summary_rows.push_str(&format!(
                r#"<tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>"#,
                record.case_id,
                record.case_pnr,
                record.case_birth_date,
                record.case_treatment_date,
                control_info
            ));
        }
        
        // Generate the HTML content
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Matching Analysis Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        header {{
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 5px;
        }}
        h1, h2, h3 {{
            color: #2c3e50;
        }}
        h1 {{
            margin-top: 0;
        }}
        .card {{
            background-color: #fff;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }}
        .info-bar {{
            background-color: #f8f9fa;
            padding: 10px;
            border-radius: 5px;
            margin-bottom: 20px;
            font-size: 0.9em;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }}
        .stat-box {{
            background-color: #e3f2fd;
            border-radius: 5px;
            padding: 15px;
            text-align: center;
        }}
        .stat-value {{
            font-size: 24px;
            font-weight: bold;
            margin: 10px 0;
        }}
        .stat-label {{
            font-size: 14px;
            color: #555;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 20px;
        }}
        th, td {{
            padding: 12px 15px;
            border-bottom: 1px solid #ddd;
            text-align: left;
        }}
        th {{
            background-color: #f2f2f2;
        }}
        tr:hover {{
            background-color: #f5f5f5;
        }}
        .summary-box {{
            background-color: #f1f8e9;
            border-left: 4px solid #7cb342;
            padding: 10px;
            margin-bottom: 20px;
        }}
        a.nav-link {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 8px 12px;
            text-decoration: none;
            border-radius: 5px;
            margin-right: 10px;
            font-size: 0.9em;
        }}
        a.nav-link:hover {{
            background-color: #2980b9;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Matching Analysis Report</h1>
            <div class="info-bar">
                <strong>Generated:</strong> {generated_at} | 
                <strong>Version:</strong> {version}
                <div style="float: right;">
                    <a href="index.html" class="nav-link">Back to Dashboard</a>
                </div>
            </div>
        </header>

        <div class="card">
            <h2>Matching Statistics</h2>
            
            <div class="stats-grid">
                <div class="stat-box">
                    <div class="stat-label">Total Cases</div>
                    <div class="stat-value">{total_cases}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Cases With Controls</div>
                    <div class="stat-value">{cases_with_controls}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Total Controls</div>
                    <div class="stat-value">{total_controls}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Avg Controls per Case</div>
                    <div class="stat-value">{avg_controls:.2}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Avg Birth Date Diff (days)</div>
                    <div class="stat-value">{avg_birth_diff:.1}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Max Birth Date Diff (days)</div>
                    <div class="stat-value">{max_birth_diff}</div>
                </div>
            </div>
            
            <div class="summary-box">
                <p><strong>Matching Overview:</strong> This report shows the results of matching cases to controls.</p>
                <p><strong>Note:</strong> Only the first 100 cases are shown in the table below.</p>
            </div>
        </div>

        <div class="card">
            <h2>Case Summary</h2>
            <table>
                <thead>
                    <tr>
                        <th>Case ID</th>
                        <th>Case PNR</th>
                        <th>Birth Date</th>
                        <th>Treatment Date</th>
                        <th>Controls</th>
                    </tr>
                </thead>
                <tbody>
                    {case_summary_rows}
                </tbody>
            </table>
        </div>
        
        <div class="card">
            <h2>Data Files</h2>
            <p>Raw data files for further analysis:</p>
            <ul>
                <li><a href="../data/matching/matching_pairs.csv">Matched Pairs Data (CSV)</a></li>
                <li><a href="../data/matching/matching_stats.csv">Matching Statistics (CSV)</a></li>
            </ul>
        </div>
    </div>
</body>
</html>"#,
            generated_at = self.runtime_info.get("generated_at").unwrap_or(&String::from("Unknown")),
            version = self.runtime_info.get("version").unwrap_or(&String::from("Unknown")),
            total_cases = total_cases,
            cases_with_controls = cases_with_controls,
            total_controls = total_controls,
            avg_controls = avg_controls_per_case,
            avg_birth_diff = avg_birth_diff,
            max_birth_diff = max_birth_diff,
            case_summary_rows = case_summary_rows
        );
        
        fs::write(&report_path, html_content).map_err(|e| {
            IdsError::io_error(format!("Failed to write matching HTML report: {}", e))
        })?;
        
        Ok(())
    }
    
    /// Generate a basic data quality report
    pub fn generate_data_quality_report(&self) -> Result<(), IdsError> {
        let report_dir = self.get_dir_path(OutputDirType::Report);
        let report_path = report_dir.join("data_quality_report.html");
        
        // Generate placeholder HTML
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Data Quality Report</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        header {{
            background-color: #f5f5f5;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 5px;
        }}
        h1, h2, h3 {{
            color: #2c3e50;
        }}
        h1 {{
            margin-top: 0;
        }}
        .card {{
            background-color: #fff;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            padding: 20px;
            margin-bottom: 20px;
        }}
        .info-bar {{
            background-color: #f8f9fa;
            padding: 10px;
            border-radius: 5px;
            margin-bottom: 20px;
            font-size: 0.9em;
        }}
        a.nav-link {{
            display: inline-block;
            background-color: #3498db;
            color: white;
            padding: 8px 12px;
            text-decoration: none;
            border-radius: 5px;
            margin-right: 10px;
            font-size: 0.9em;
        }}
        a.nav-link:hover {{
            background-color: #2980b9;
        }}
        .placeholder {{
            background-color: #f8f9fa;
            border: 1px dashed #ccc;
            padding: 20px;
            text-align: center;
            color: #777;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Data Quality Report</h1>
            <div class="info-bar">
                <strong>Generated:</strong> {generated_at} | 
                <strong>Version:</strong> {version}
                <div style="float: right;">
                    <a href="index.html" class="nav-link">Back to Dashboard</a>
                </div>
            </div>
        </header>

        <div class="card">
            <h2>Data Quality Overview</h2>
            <p>This report provides an overview of data quality metrics across the datasets used for analysis.</p>
            
            <div class="placeholder">
                <h3>Data Quality Analysis</h3>
                <p>The detailed data quality analysis will appear here in future versions.</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
            generated_at = self.runtime_info.get("generated_at").unwrap_or(&String::from("Unknown")),
            version = self.runtime_info.get("version").unwrap_or(&String::from("Unknown"))
        );
        
        fs::write(&report_path, html_content).map_err(|e| {
            IdsError::io_error(format!("Failed to write data quality HTML report: {}", e))
        })?;
        
        Ok(())
    }
}

/// Enum for different output directory types
#[derive(Debug, Clone, Copy)]
pub enum OutputDirType {
    Base,
    Report,
    Data,
    Plots,
    Logs,
    BalanceData,
    MatchingData,
    RegisterData,
    BalancePlots,
    MatchingPlots,
    DataQualityPlots,
}