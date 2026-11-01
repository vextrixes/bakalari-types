use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Assistant {
    pub id: String,
    pub is_present: bool,
    pub student_id: String,
}
