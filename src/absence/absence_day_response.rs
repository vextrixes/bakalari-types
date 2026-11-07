use crate::absence::Absence;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceDayResponse {
    pub date: DateTime<FixedOffset>,
    pub absences: Vec<Absence>,
}
