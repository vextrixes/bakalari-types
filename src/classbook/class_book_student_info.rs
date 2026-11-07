use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookStudentInfo {
    pub id: String,
    pub name: String,
    pub is_adult: bool,
    pub registration_number: i64,
    pub abbrev: Option<String>,
}
