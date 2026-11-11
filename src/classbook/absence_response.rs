use crate::classbook::{AbsenceGroup, Hour, SchoolEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceResponse {
    pub lesson_id: String,
    pub absences_groups: Vec<AbsenceGroup>,
    pub hours: Vec<Hour>,
    pub school_events: Vec<SchoolEvent>,
}
