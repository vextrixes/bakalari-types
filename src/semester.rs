use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::StdTypeNullable;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CertificateTerms {
    pub certificate_terms: Vec<CertificateTerm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CertificateTerm {
    pub final_marks: Option<Vec<FinalMark>>,
    pub subjects: Option<Vec<StdTypeNullable>>,
    pub grade_name: String,
    pub grade: i64,
    pub year_in_school: i64,
    pub school_year: String,
    pub semester: String,
    pub semester_name: String,
    pub repeated: bool,
    pub closed: bool,
    pub achievement_text: Option<String>,
    pub absent_hours: Option<i64>,
    pub not_excused_hours: Option<i64>,
    pub marks_average: Option<f64>,
    pub certificate_date: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinalMark {
    pub mark_date: DateTime<FixedOffset>,
    pub edit_date: DateTime<FixedOffset>,
    pub mark_text: String,
    pub subject_id: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasure {
    pub pedagogical_measures: Vec<PedagogicalMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasures {
    pub school_year: String,
    pub semester: String,
    pub type_label: String,
    pub date: DateTime<FixedOffset>,
    pub type_id: String,
    pub text: String,
    pub actual_grade: Option<i64>,
    pub actual_grade_name: Option<String>,
}
