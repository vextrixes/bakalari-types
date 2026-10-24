use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub percentage_threshold: f64, //TODO: cutom type (u8)
    pub absences: Vec<AbsenceListItem>,
    pub absences_per_subject: Vec<SubjectAbsence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceListItem {
    pub date: chrono::DateTime<Utc>,
    pub unsolved: i8,
    pub ok: i8,
    pub missed: i8,
    pub late: i8,
    pub soon: i8,
    pub school: i8,
    pub distance_teaching: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubjectAbsence {
    pub subject_name: String,
    pub lessons_count: u16,
    pub base: u16,
    pub late: u16,
    pub soon: u16,
    pub school: u16,
    pub distance_teaching: u16,
}
