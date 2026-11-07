use crate::{
    subjects::Subject,
    substitutions::{ChangeDisplayType, ChangeLessonInfo, SubstitutionChangeType},
    timetable::AtomType,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

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
