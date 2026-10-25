use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Measures {
    pub pedagogical_measures: Vec<PedagogicalMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasure {
    pub school_year: String,
    pub semester: String,
    pub type_label: String,
    pub date: DateTime<Utc>,
    pub type_id: String,
    pub text: String,
}
