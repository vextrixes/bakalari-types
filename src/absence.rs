use chrono::FixedOffset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentAbsenceOverviewResponse {
    pub percentage_threshold: f64, //TODO: cutom type (u8)
    pub absences: Vec<AbsenceDay>,
    pub absences_per_subject: Option<Vec<SubjectAbsence>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceDay {
    pub date: chrono::DateTime<FixedOffset>,
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
