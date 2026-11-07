use crate::absence::AbsentType;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookSaveAbsenceRequest {
    pub notice: Option<String>,
    pub absent_kind_id: Option<String>,
    pub student_id: String,
    pub minutes: i64,
    pub absent_type: AbsentType,
    pub hour_id: String,
    pub start: i64,
    pub duration: i64,
    pub date: NaiveDateTime, //Wierd idk why
}
