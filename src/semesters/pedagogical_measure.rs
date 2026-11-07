use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasures {
    pub pedagogical_measures: Vec<PedagogicalMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasure {
    pub school_year: String,
    pub semester: String,
    pub type_label: String,
    pub date: DateTime<FixedOffset>,
    pub type_id: String,
    pub text: String,
    pub actual_grade: Option<i64>,
    pub actual_grade_name: Option<String>,
}
