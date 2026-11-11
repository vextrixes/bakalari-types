use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrescriptionDetailItem {
    pub due_date: DateTime<FixedOffset>,
    pub title: Option<String>,
    pub amount: f64,
    pub advance_notice_date: DateTime<FixedOffset>,
}
