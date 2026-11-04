use crate::AbsentType;
use chrono::{DateTime, FixedOffset};
pub struct ClassBookSaveAbsenceRequest {
    //check nullability on all types
    pub absent_kind_id: Option<String>,
    pub minutes: i64,
    pub notice: Option<String>,
    pub absent_type: AbsentType,
    pub hour_id: i64,
    pub start: i64,
    pub duration: i64,
    pub student_id: String,
    pub date: DateTime<FixedOffset>,
}
