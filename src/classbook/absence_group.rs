use crate::classbook::{AbsenceTypeOption, GroupInfo, StudentAbsences};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceGroup {
    pub students_absences: Vec<StudentAbsences>,
    pub absent_type_options: Vec<AbsenceTypeOption>,
    pub group: GroupInfo,
}
