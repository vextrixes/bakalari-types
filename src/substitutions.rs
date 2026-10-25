use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Substitutions {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub changes: Vec<SubstitutionChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubstitutionChange {
    pub change_subject: Option<String>, //Unknown if string is the right type
    pub day: DateTime<Utc>,
    pub hours: String,
    pub change_type: String,
    pub description: String,
    pub time: String,
    pub type_abbrev: Option<String>, //Unknown if string is the right type,
    pub type_name: Option<String>,   //Unknown if string is the right type
}
