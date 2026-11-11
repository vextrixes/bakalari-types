use crate::classbook::Hour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct AbsenceRecordResult {
    pub absences: Vec<AbsenceRecordResult>,
    pub hours: Vec<Hour>,
}
