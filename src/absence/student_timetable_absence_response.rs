use crate::{AbsenceDayResponse, AbsenceTypeOption, StdTypeNullable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentTimetableAbsenceResponse {
    pub student: Option<StdTypeNullable>,
    pub days: Vec<AbsenceDayResponse>,
    pub absent_type_options: Vec<AbsenceTypeOption>,
}
