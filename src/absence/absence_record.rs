use crate::absence::AbsentType;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceRecord {
    pub minutes: i64,
    pub absent_type: AbsentType,
    pub hour_id: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub start: i64,
    pub duration: i64,
    pub absent_kind_id: Option<String>,
    pub notice: Option<String>,
    pub student_id: Option<String>,
    pub apology_id: Option<String>,
}
