use chrono::NaiveDate;
use crate::model::pnr::Pnr;

/// Information about a parent
#[derive(Debug, Clone)]
pub struct ParentInfo {
    /// Parent's PNR
    pub pnr: Pnr,
    
    /// Parent's birth date
    pub birth_date: Option<NaiveDate>,
}

/// Represents family relations for a person
#[derive(Debug, Clone)]
pub struct FamilyRelations {
    /// Person's PNR
    pub pnr: Pnr,
    
    /// Person's birth date
    pub birth_date: NaiveDate,
    
    /// Father information
    pub father: Option<ParentInfo>,
    
    /// Mother information
    pub mother: Option<ParentInfo>,
    
    /// Family ID
    pub family_id: Option<String>,
}

impl FamilyRelations {
    /// Create a new family relations record
    pub fn new(pnr: impl Into<Pnr>, birth_date: NaiveDate) -> Self {
        Self {
            pnr: pnr.into(),
            birth_date,
            father: None,
            mother: None,
            family_id: None,
        }
    }

    /// Add father information
    pub fn with_father(mut self, pnr: impl Into<Pnr>, birth_date: Option<NaiveDate>) -> Self {
        self.father = Some(ParentInfo {
            pnr: pnr.into(),
            birth_date,
        });
        self
    }

    /// Add mother information
    pub fn with_mother(mut self, pnr: impl Into<Pnr>, birth_date: Option<NaiveDate>) -> Self {
        self.mother = Some(ParentInfo {
            pnr: pnr.into(),
            birth_date,
        });
        self
    }

    /// Add family ID
    pub fn with_family_id(mut self, id: impl Into<String>) -> Self {
        self.family_id = Some(id.into());
        self
    }
    
    /// Check if the person has both parents
    #[must_use] pub fn has_both_parents(&self) -> bool {
        self.father.is_some() && self.mother.is_some()
    }
    
    /// Check if the person has at least one parent
    #[must_use] pub fn has_any_parent(&self) -> bool {
        self.father.is_some() || self.mother.is_some()
    }
}