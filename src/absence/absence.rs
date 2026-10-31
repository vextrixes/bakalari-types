use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{Hour, StdType, StdTypeNullable};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubjectAbsenceOverviewResponse {
    pub percentage_threshold: f64,
    pub absences: Option<Vec<AbsenceDay>>,
    pub absences_per_subject: Option<Vec<SubjectAbsence>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentTimetableAbsenceResponse {
    pub student: Option<StudentInfo>,
    pub days: Vec<AbsenceDayResponse>,
    pub absent_type_options: Vec<AbsenceTypeOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceTypeOption {
    pub absent_type: AbsentType,
    pub absent_kind_enabled: bool,
    pub absent_kind_required: bool,
    pub absent_note_enabled: bool,
    pub next_absent_type: AbsentType,
    pub absent_kinds: Vec<AbsenceKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AbsentType {
    Absent,
    AbsentLate,
    AbsentMiss,
    AbsentOk,
    AbsentSchool,
    AbsentSoon,
    DistanceTeaching,
    NoAbsent,
}

type StudentInfo = StdTypeNullable;
type AbsenceKind = StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceSubject {
    pub id: String,
    pub abbrev: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceDayResponse {
    pub date: DateTime<FixedOffset>,
    pub absences: Vec<Absence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub minutes: i64,
    pub absent_type: AbsentType,
    pub hour_id: String,
    pub start: i64,
    pub duration: i64,
    pub hour_index: Option<i64>,
    pub absent_kind_id: Option<String>,
    pub notice: Option<String>,
    pub student_id: Option<String>,
    pub subject: Option<AbsenceSubject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExcuseAbsenceResponse {
    pub absences: Vec<AbsenceRecord>,
    pub hours: Vec<Hour>,
}

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
