use super::BalanceChecker;
use crate::balance::results::BalanceResults;
use chrono::NaiveDate;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use types::error::IdsError;
use types::models::CovariateType;

impl BalanceChecker {
    /// Calculate balance metrics between case and control groups
    pub fn calculate_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<BalanceResults, IdsError> {
        debug!(
            "Starting balance calculation for {} cases and {} controls",
            cases.len(),
            controls.len()
        );

        let multi_progress = indicatif::MultiProgress::new();
        let overall_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .expect("Failed to create progress bar template - this is a static template that should never fail");

        let overall_pb = multi_progress.add(ProgressBar::new(3)); // 3 steps: demographics, income, education
        overall_pb.set_style(overall_style);
        overall_pb.set_message("Calculating balance...");

        let mut results = BalanceResults::new();

        // Calculate overall balance
        self.add_all_balances(&mut results, cases, controls, &overall_pb)?;

        // Calculate matched pair details
        overall_pb.set_message("Processing matched pairs...");
        self.add_matched_pair_details(&mut results, cases, controls)?;
        overall_pb.finish_with_message("Balance calculation complete");

        self.log_balance_statistics(&results);
        Ok(results)
    }
    
    /// Calculate balance metrics for all covariate types
    fn add_all_balances(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        overall_pb: &ProgressBar,
    ) -> Result<(), IdsError> {
        // Update progress bar to account for occupation processing
        let total_steps = 4; // demographics, income, education, occupation
        overall_pb.set_length(total_steps);
        
        overall_pb.set_message("Processing demographics...");
        self.calculate_demographic_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing income...");
        self.calculate_income_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing education...");
        self.calculate_education_balance(results, cases, controls)?;
        overall_pb.inc(1);
        
        overall_pb.set_message("Processing occupation...");
        self.calculate_occupation_balance(results, cases, controls)?;
        overall_pb.inc(1);

        Ok(())
    }

    fn calculate_demographic_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // Original variables
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Size",
            |covariate| covariate.family_size().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Family Size".to_string(), missing_rates.0, missing_rates.1);

        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Municipality",
            |covariate| covariate.municipality().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Municipality".to_string(), missing_rates.0, missing_rates.1);

        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Type",
            |covariate| covariate.family_type().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Family Type".to_string(), missing_rates.0, missing_rates.1);

        // New variables from TROUBLE.md (BEF register)
        
        // Civil status (CIVST) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Civil Status",
            |covariate| covariate.civil_status().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Civil Status".to_string(), missing_rates.0, missing_rates.1);
        
        // Gender (KOEN) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Gender",
            |covariate| covariate.gender().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Gender".to_string(), missing_rates.0, missing_rates.1);
        
        // Citizenship (STATSB) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Citizenship",
            |covariate| covariate.citizenship().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Citizenship".to_string(), missing_rates.0, missing_rates.1);
        
        // Age (ALDER) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Age",
            |covariate| covariate.age().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Age".to_string(), missing_rates.0, missing_rates.1);
        
        // Children count (ANTBOERNF/ANTBOERNH) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Children Count",
            |covariate| covariate.children_count().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Children Count".to_string(), missing_rates.0, missing_rates.1);

        Ok(())
    }

    fn calculate_income_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // Original income variable
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Income",
            |covariate| covariate.income_amount(),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Income".to_string(), missing_rates.0, missing_rates.1);

        // New variables from TROUBLE.md (IND register)
        
        // Wage income (LOENMV_13) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Wage Income",
            |covariate| covariate.wage_income(),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Wage Income".to_string(), missing_rates.0, missing_rates.1);
        
        // Employment status (BESKST13) - Numeric categorical
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Employment Status",
            |covariate| covariate.employment_status().map(|val| val as f64),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Employment Status".to_string(), missing_rates.0, missing_rates.1);
        
        // Also add as categorical for better representation
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Employment Status Category",
            |covariate| covariate.employment_status().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Employment Status Category".to_string(), missing_rates.0, missing_rates.1);

        Ok(())
    }

    fn calculate_education_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // 1. Process education levels as categorical variables
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "Education Level",
            |covariate| covariate.education_level().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Education Level".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // 2. Process ISCED codes as a separate categorical variable
        // Only if ISCED codes are available in the data
        let (isced_summaries, isced_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "ISCED Level",
            |covariate| covariate.isced_code().map(|s| s.to_string()),
        )?;

        for summary in isced_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "ISCED Level".to_string(),
            isced_missing_rates.0,
            isced_missing_rates.1,
        );
        
        // 3. Process education years as a numeric variable (if available)
        let (years_summary, years_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "Education Years",
            |covariate| covariate.education_years().map(|y| y as f64),
        )?;
        
        results.add_summary(years_summary);
        results.add_missing_rate(
            "Education Years".to_string(),
            years_missing_rates.0,
            years_missing_rates.1,
        );

        Ok(())
    }
    
    fn calculate_occupation_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // 1. Process SOCIO13 codes as categorical variables
        let (code_summaries, code_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO13 Code",
            |covariate| covariate.occupation_code().map(|s| s.to_string()),
        )?;

        for summary in code_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO13 Code".to_string(),
            code_missing_rates.0,
            code_missing_rates.1,
        );
        
        // 2. Process SOCIO13 codes as a numeric variable for standardized difference calculation
        let (socio_summary, socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO13 Value",
            |covariate| {
                covariate.occupation_code().clone()
                    .and_then(|code| code.parse::<f64>().ok())
            },
        )?;
        
        results.add_summary(socio_summary);
        results.add_missing_rate(
            "SOCIO13 Value".to_string(),
            socio_missing_rates.0,
            socio_missing_rates.1,
        );
        
        // 3. Process occupation classification system as a separate categorical variable
        // This might be used for different versions or systems (DISCO, ISCO, etc.)
        let (class_summaries, class_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Classification System",
            |covariate| covariate.classification().map(|s| s.to_string()),
        )?;

        for summary in class_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Classification System".to_string(),
            class_missing_rates.0,
            class_missing_rates.1,
        );
        
        // New variables from TROUBLE.md (AKM register)
        
        // SOCIO - older socioeconomic classification
        let (socio_summary, socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO",
            |covariate| covariate.socio().map(|val| val as f64),
        )?;
        
        results.add_summary(socio_summary);
        results.add_missing_rate(
            "SOCIO".to_string(),
            socio_missing_rates.0,
            socio_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO Category",
            |covariate| covariate.socio().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // SOCIO02 - another socioeconomic classification
        let (socio02_summary, socio02_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO02",
            |covariate| covariate.socio02().map(|val| val as f64),
        )?;
        
        results.add_summary(socio02_summary);
        results.add_missing_rate(
            "SOCIO02".to_string(),
            socio02_missing_rates.0,
            socio02_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO02 Category",
            |covariate| covariate.socio02().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO02 Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // PRE_SOCIO - previous socioeconomic status
        let (pre_socio_summary, pre_socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Previous Socioeconomic Status",
            |covariate| covariate.pre_socio().map(|val| val as f64),
        )?;
        
        results.add_summary(pre_socio_summary);
        results.add_missing_rate(
            "Previous Socioeconomic Status".to_string(),
            pre_socio_missing_rates.0,
            pre_socio_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Previous Socioeconomic Category",
            |covariate| covariate.pre_socio().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Previous Socioeconomic Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );

        Ok(())
    }
    
    /// Log statistics about the balance calculation results
    fn log_balance_statistics(&self, results: &BalanceResults) {
        debug!("Balance calculation completed:");
        debug!("Total summaries: {}", results.summaries.len());
        debug!(
            "Total matched pair details: {}",
            results.matched_pair_details.len()
        );

        for summary in &results.summaries {
            if summary.std_diff.abs() > 0.1 {
                debug!(
                    "Large imbalance detected for {}: std_diff = {:.3}",
                    summary.variable, summary.std_diff
                );
            }
        }
    }
}