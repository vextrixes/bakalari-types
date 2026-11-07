use crate::timetable::TimetableChange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssistanceAtom {
    pub hour_id: i64,
    pub start: i64,
    pub duration: i64,
    pub class_id: Option<String>,
    pub group_id: Option<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub room_id: Option<String>,
    pub student_id: Option<String>,
    pub change: Option<TimetableChange>,
}
