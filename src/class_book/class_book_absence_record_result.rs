use crate::ClassBookHour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassBookAbsenceRecordResult {
    pub absences: Vec<ClassBookAbsenceRecordResult>,
    pub hours: Vec<ClassBookHour>,
}
