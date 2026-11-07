use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinalMark {
    pub mark_date: DateTime<FixedOffset>,
    pub edit_date: DateTime<FixedOffset>,
    pub mark_text: String,
    pub subject_id: String,
    pub id: String,
}
