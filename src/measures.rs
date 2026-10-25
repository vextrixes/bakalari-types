use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Measures {
    pedagogical_measures: Vec<PedagogicalMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PedagogicalMeasure {
    school_year: String,
    semester: String,
    type_label: String,
    date: DateTime<Utc>,
    type_id: String,
    text: String,
}
