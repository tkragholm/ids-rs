//! ICD-10 chapter classification system
//!
//! This module implements the ICD-10 chapter classification system
//! for categorizing diagnosis codes according to WHO standards.

pub mod diagnosis_pattern;

/// Represents the 22 main chapters in the ICD-10 classification system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icd10Chapter {
    InfectiousParasitic,
    Neoplasms,
    BloodImmuneDisorders,
    EndocrineMetabolicNutritional,
    MentalBehavioral,
    NervousSystem,
    EyeAdnexa,
    EarMastoid,
    CirculatorySystem,
    RespiratorySystem,
    DigestiveSystem,
    SkinSubcutaneous,
    MusculoskeletalConnective,
    GenitourinarySystem,
    PregnancyChildbirth,
    PerinatalPeriod,
    CongenitalMalformations,
    SymptomsSignsAbnormalities,
    InjuryPoisoning,
    ExternalCauses,
    FactorsHealthStatus,
    SpecialPurposes,
}

impl Icd10Chapter {
    /// Determine the ICD-10 chapter from a diagnosis code
    #[must_use] pub fn from_code(code: &str) -> Option<Self> {
        // Return None for empty or too short codes
        if code.is_empty() || code.is_empty() {
            return None;
        }

        // Extract the first character (must be a letter)
        let first_char = code.chars().next().unwrap().to_ascii_uppercase();
        
        match first_char {
            'A' | 'B' => Some(Self::InfectiousParasitic),
            'C' => Some(Self::Neoplasms),
            'D' => {
                // D00-D48: Neoplasms
                // D50-D89: Blood and immune system disorders
                if code.len() >= 2 {
                    let second = code.chars().nth(1).unwrap();
                    if let Some(d) = second.to_digit(10) {
                        if d <= 4 {
                            Some(Self::Neoplasms)
                        } else {
                            Some(Self::BloodImmuneDisorders)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            'E' => Some(Self::EndocrineMetabolicNutritional),
            'F' => Some(Self::MentalBehavioral),
            'G' => Some(Self::NervousSystem),
            'H' => {
                // H00-H59: Eye and adnexa
                // H60-H95: Ear and mastoid process
                if code.len() >= 2 {
                    let second = code.chars().nth(1).unwrap();
                    if let Some(d) = second.to_digit(10) {
                        if d <= 5 {
                            Some(Self::EyeAdnexa)
                        } else {
                            Some(Self::EarMastoid)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            'I' => Some(Self::CirculatorySystem),
            'J' => Some(Self::RespiratorySystem),
            'K' => Some(Self::DigestiveSystem),
            'L' => Some(Self::SkinSubcutaneous),
            'M' => Some(Self::MusculoskeletalConnective),
            'N' => Some(Self::GenitourinarySystem),
            'O' => Some(Self::PregnancyChildbirth),
            'P' => Some(Self::PerinatalPeriod),
            'Q' => Some(Self::CongenitalMalformations),
            'R' => Some(Self::SymptomsSignsAbnormalities),
            'S' | 'T' => Some(Self::InjuryPoisoning),
            'V' | 'W' | 'X' | 'Y' => Some(Self::ExternalCauses),
            'Z' => Some(Self::FactorsHealthStatus),
            'U' => Some(Self::SpecialPurposes),
            _ => None,
        }
    }

    /// Get the description of the ICD-10 chapter
    #[must_use] pub fn description(&self) -> &'static str {
        match self {
            Self::InfectiousParasitic => "I: Infectious and parasitic diseases",
            Self::Neoplasms => "II: Neoplasms",
            Self::BloodImmuneDisorders => "III: Blood and immune system disorders",
            Self::EndocrineMetabolicNutritional => "IV: Endocrine, nutritional and metabolic diseases",
            Self::MentalBehavioral => "V: Mental and behavioral disorders",
            Self::NervousSystem => "VI: Diseases of the nervous system",
            Self::EyeAdnexa => "VII: Diseases of the eye and adnexa",
            Self::EarMastoid => "VIII: Diseases of the ear and mastoid process",
            Self::CirculatorySystem => "IX: Diseases of the circulatory system",
            Self::RespiratorySystem => "X: Diseases of the respiratory system",
            Self::DigestiveSystem => "XI: Diseases of the digestive system",
            Self::SkinSubcutaneous => "XII: Diseases of the skin and subcutaneous tissue",
            Self::MusculoskeletalConnective => "XIII: Diseases of the musculoskeletal system and connective tissue",
            Self::GenitourinarySystem => "XIV: Diseases of the genitourinary system",
            Self::PregnancyChildbirth => "XV: Pregnancy, childbirth and the puerperium",
            Self::PerinatalPeriod => "XVI: Certain conditions originating in the perinatal period",
            Self::CongenitalMalformations => "XVII: Congenital malformations, deformations and chromosomal abnormalities",
            Self::SymptomsSignsAbnormalities => "XVIII: Symptoms, signs and abnormal clinical and laboratory findings",
            Self::InjuryPoisoning => "XIX: Injury, poisoning and certain other consequences of external causes",
            Self::ExternalCauses => "XX: External causes of morbidity and mortality",
            Self::FactorsHealthStatus => "XXI: Factors influencing health status and contact with health services",
            Self::SpecialPurposes => "XXII: Codes for special purposes",
        }
    }
    
    /// Get the code range for this chapter
    #[must_use] pub fn code_range(&self) -> &'static str {
        match self {
            Self::InfectiousParasitic => "A00-B99",
            Self::Neoplasms => "C00-D48",
            Self::BloodImmuneDisorders => "D50-D89",
            Self::EndocrineMetabolicNutritional => "E00-E90",
            Self::MentalBehavioral => "F00-F99",
            Self::NervousSystem => "G00-G99",
            Self::EyeAdnexa => "H00-H59",
            Self::EarMastoid => "H60-H95",
            Self::CirculatorySystem => "I00-I99",
            Self::RespiratorySystem => "J00-J99",
            Self::DigestiveSystem => "K00-K93",
            Self::SkinSubcutaneous => "L00-L99",
            Self::MusculoskeletalConnective => "M00-M99",
            Self::GenitourinarySystem => "N00-N99",
            Self::PregnancyChildbirth => "O00-O99",
            Self::PerinatalPeriod => "P00-P96",
            Self::CongenitalMalformations => "Q00-Q99",
            Self::SymptomsSignsAbnormalities => "R00-R99",
            Self::InjuryPoisoning => "S00-T98",
            Self::ExternalCauses => "V01-Y98",
            Self::FactorsHealthStatus => "Z00-Z99",
            Self::SpecialPurposes => "U00-U99",
        }
    }
}