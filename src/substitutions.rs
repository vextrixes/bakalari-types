use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{AtomType, Subject};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Substitutions {
    pub from: DateTime<FixedOffset>,
    pub to: DateTime<FixedOffset>,
    pub changes: Vec<SubstitutionChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubstitutionChange {
    pub change_subject: Option<Subject>,
    pub day: DateTime<FixedOffset>,
    pub hours: String,
    pub change_type: SubstitutionChangeType,
    pub description: String,
    pub time: String,
    pub change_lesson_info: Option<ChangeLessonInfo>,
    pub change_display_type: Option<ChangeDisplayType>,
    pub atom_type: Option<AtomType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SubstitutionChangeType {
    Absence,
    Added,
    Canceled,
    NoChange,
    Removed,
    RoomChanged,
    Substitution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeDisplayType {
    OriginalTeacher,
    SubstituteTeacher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeLessonInfo {
    class_abbrev: Option<String>,
    group_abbrev: Option<String>,
    is_class_group: Option<bool>,
    subject_abbrev: Option<String>,
    room_abbrev: Option<String>,
}
