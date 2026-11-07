use crate::classbook::ClassBookStudentTimetable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookStudentsTimetableResponse {
    pub student_timetables: Vec<ClassBookStudentTimetable>,
}
