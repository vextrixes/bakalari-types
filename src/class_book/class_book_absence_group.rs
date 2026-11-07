use crate::{ClassBookAbsenceTypeOption, ClassBookGroupInfo, ClassBookStudentAbsences};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookAbsenceGroup {
    pub students_absences: Vec<ClassBookStudentAbsences>,
    pub absent_type_options: Vec<ClassBookAbsenceTypeOption>,
    pub group: ClassBookGroupInfo,
}
