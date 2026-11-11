use crate::classbook::{Absence, StudentInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentAbsences {
    pub student: StudentInfo,
    pub absences: Vec<Absence>,
    pub student_released: bool,
    pub school_event_ids: Vec<String>,
}
