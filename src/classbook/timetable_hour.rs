use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableHour {
    pub id: i64,
    pub start: i64,
    pub duration: i64,
}
