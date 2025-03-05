use chrono::NaiveDate;

/// Family relationships for a person
#[derive(Clone, Debug)]
pub struct FamilyRelations {
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub father_id: Option<String>,
    pub father_birth_date: Option<NaiveDate>,
    pub mother_id: Option<String>,
    pub mother_birth_date: Option<NaiveDate>,
    pub family_id: Option<String>,
}

impl FamilyRelations {
    /// Create a new family relations object
    pub fn new(pnr: impl Into<String>, birth_date: NaiveDate) -> Self {
        Self {
            pnr: pnr.into(),
            birth_date,
            father_id: None,
            father_birth_date: None,
            mother_id: None,
            mother_birth_date: None,
            family_id: None,
        }
    }

    /// Add father information
    pub fn with_father(mut self, id: impl Into<String>, birth_date: Option<NaiveDate>) -> Self {
        self.father_id = Some(id.into());
        self.father_birth_date = birth_date;
        self
    }

    /// Add mother information
    pub fn with_mother(mut self, id: impl Into<String>, birth_date: Option<NaiveDate>) -> Self {
        self.mother_id = Some(id.into());
        self.mother_birth_date = birth_date;
        self
    }

    /// Add family ID
    pub fn with_family_id(mut self, id: impl Into<String>) -> Self {
        self.family_id = Some(id.into());
        self
    }
}