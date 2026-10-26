use serde::{Deserialize, Serialize};

use crate::StdClassGroup;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonAbsence {
    pub lesson_id: String,
    //ClassBookRecordId: unknown probably Option<String>,
    pub absences_groups: Vec<AbsencesGroup>,
    pub hour_id: u8,
    pub start: u16,
    pub duration: u8,
    pub hours: Vec<LessonHour>,
    //   "SchoolEvents": []
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonHour {
    pub id: u8,
    pub caption: String,
    pub begin_time: String,
    pub end_time: String,
    pub start: u16,
    pub duration: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsencesGroup {
    pub students_absences: Vec<LessonStudentAbsence>,
    pub absent_type_options: Vec<AbsentTypeOption>,
    pub group: StdClassGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonStudentAbsence {
    pub student: LessonStudentInfo,
    //    "Absences": [], unknown
    pub student_released: bool,
    pub school_event_ids: Option<Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonStudentInfo {
    pub is_adult: bool,
    pub registration_number: u32,
    pub id: String,
    pub abbrev: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsentTypeOption {
    pub absent_type: String,
    pub absent_kind_enabled: bool,
    pub absent_kind_required: bool,
    pub absent_note_enabled: bool,
    pub absent_kinds: Vec<StdType>
    pub next_absent_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonLastTheme {
    pub lesson_id: String,
    pub last_theme: String,
    pub next_label: String,
}
