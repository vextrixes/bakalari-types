use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SubstitutionChangeType {
    Absence,
    Added,
    Canceled,
    NoChange,
    Removed,
    RoomChanged,
    Substitution,
}
