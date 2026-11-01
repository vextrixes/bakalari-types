use crate::{AbsenceRecord, Hour};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExcuseAbsenceResponse {
    pub absences: Vec<AbsenceRecord>,
    pub hours: Vec<Hour>,
}
