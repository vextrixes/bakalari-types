use crate::class_book::ClassBookHour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct ClassBookAbsenceRecordResult {
    pub absences: Vec<ClassBookAbsenceRecordResult>,
    pub hours: Vec<ClassBookHour>,
}
