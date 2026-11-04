use crate::{ClassBookAbsenceGroup, ClassBookHour, ClassBookSchoolEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookAbsenceResponse {
    pub lesson_id: String,
    pub absences_groups: Vec<ClassBookAbsenceGroup>,
    pub hours: Vec<ClassBookHour>,
    pub school_events: Vec<ClassBookSchoolEvent>,
}
