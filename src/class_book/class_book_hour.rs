use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookHour {
    pub id: i64,
    pub caption: String,
    pub begin_time: String,
    pub end_time: String,
    pub start: i64,
    pub duration: i64,
    pub is_backed_by_system: bool,
}
