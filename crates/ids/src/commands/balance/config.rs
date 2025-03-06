/// Configuration for the balance check command
#[derive(Debug, Clone)]
pub struct BalanceCheckConfig<'a> {
    /// Path to the CSV file containing matched pairs
    pub matches_file: &'a str,
    
    /// Base directory containing register data
    pub covariate_dir: Option<&'a str>,
    
    /// Output directory for results
    pub output_dir: &'a str,
    
    /// Path to the family relationships file
    pub family_file: Option<&'a str>,
    
    /// Path to the AKM register directory
    pub akm_dir: Option<&'a str>,
    
    /// Path to the BEF register directory
    pub bef_dir: Option<&'a str>,
    
    /// Path to the IND register directory
    pub ind_dir: Option<&'a str>,
    
    /// Path to the UDDF register directory
    pub uddf_dir: Option<&'a str>,
    
    /// Whether to generate structured HTML reports
    pub generate_structured_output: bool,
}