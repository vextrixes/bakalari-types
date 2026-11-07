use crate::class_book::ClassBookTimetableHour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookStudentTimetable {
    pub id: String,
    pub abbrev: Option<String>,
    pub name: String,
    pub hours: Vec<ClassBookTimetableHour>,
}
