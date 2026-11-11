use crate::classbook::StudentTimetable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentsTimetableResponse {
    pub student_timetables: Vec<StudentTimetable>,
}
