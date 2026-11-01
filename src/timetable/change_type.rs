use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Changetype {
    Absence,
    Added,
    Canceled,
    NoChange,
    Removed,
    RoomChanged,
    Substitution,
}
