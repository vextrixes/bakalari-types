use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GdprValidity {
    EndOfStudy,
    EndOfStudyPlusYears,
    ToAppeal,
    Unknown,
    Validity,
}
