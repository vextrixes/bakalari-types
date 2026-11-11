use crate::classbook::TimetableHour;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentTimetable {
    pub id: String,
    pub abbrev: Option<String>,
    pub name: String,
    pub hours: Vec<TimetableHour>,
}
