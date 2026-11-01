use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubjectAbsence {
    pub subject_name: String,
    pub lessons_count: u16,
    pub base: u16,
    pub late: u16,
    pub soon: u16,
    pub school: u16,
    pub distance_teaching: u16,
}
