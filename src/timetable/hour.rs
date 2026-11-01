use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hour {
    pub id: i64,
    pub caption: String,
    pub begin_time: String,
    pub end_time: String,
    pub start: Option<i64>,
    pub duration: Option<i64>,
}
