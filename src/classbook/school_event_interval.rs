use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolEventInterval {
    pub event_begin: DateTime<FixedOffset>,
    pub event_end: DateTime<FixedOffset>,
    pub all_day: bool,
    pub affected_hours: Vec<i64>,
}
