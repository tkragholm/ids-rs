use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AkmRecord {
    pub pnr: String,
    pub socio: Option<i32>,
    pub socio02: Option<i32>,
    pub socio13: i32,
    pub cprtjek: Option<i32>,
    pub cprtype: Option<i32>,
    pub version: Option<String>,
    pub senr: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BefRecord {
    pub pnr: String,
    pub aegte_id: Option<String>,
    pub alder: String,
    pub antboernf: Option<i32>,
    pub antboernh: Option<i32>,
    pub antpersf: Option<i32>,
    pub antpersh: Option<i32>,
    pub bop_vfra: Option<NaiveDate>,
    pub civst: Option<String>,
    pub familie_id: Option<String>,
    pub familie_type: Option<String>,
    pub far_id: Option<String>,
    pub foed_dag: NaiveDate,
    pub koen: String,
    pub kom: Option<i32>,
    pub mor_id: Option<String>,
    pub statsb: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct IndRecord {
    pub pnr: String,
    pub beskst13: Option<i32>,
    pub loenmv_13: Option<f64>,
    pub perindkialt_13: Option<f64>,
    pub pre_socio: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UddfRecord {
    pub pnr: String,
    pub hfaudd: Option<String>,
    pub hf_vfra: Option<NaiveDate>,
    pub hf_vtil: Option<NaiveDate>,
    pub instnr: Option<i32>,
}
