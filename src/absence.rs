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
    pub unsolved: u16,
    pub ok: u16,
    pub missed: u16,
    pub late: u16,
    pub soon: u16,
    pub school: u16,
    pub distance_teaching: u16,
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
