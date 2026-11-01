use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeDisplayType {
    OriginalTeacher,
    SubstituteTeacher,
}
