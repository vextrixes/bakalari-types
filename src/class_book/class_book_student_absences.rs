use crate::class_book::{ClassBookAbsence, ClassBookStudentInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookStudentAbsences {
    pub student: ClassBookStudentInfo,
    pub absences: Vec<ClassBookAbsence>,
    pub student_released: bool,
    pub school_event_ids: Vec<String>,
}
