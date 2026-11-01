use crate::{AbsenceDay, SubjectAbsence};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceOverviewResponse {
    pub percentage_threshold: f64, //TODO: cutom type (u8)
    pub absences: Vec<AbsenceDay>,
    pub absences_per_subject: Option<Vec<SubjectAbsence>>,
}
