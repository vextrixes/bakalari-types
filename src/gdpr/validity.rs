use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Validity {
    EndOfStudy,
    EndOfStudyPlusYears,
    ToAppeal,
    Unknown,
    Validity,
}
