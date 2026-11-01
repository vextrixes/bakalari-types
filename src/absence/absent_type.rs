use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AbsentType {
    Absent,
    AbsentLate,
    AbsentMiss,
    AbsentOk,
    AbsentSchool,
    AbsentSoon,
    DistanceTeaching,
    NoAbsent,
}
