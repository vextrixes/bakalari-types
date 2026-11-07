use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookGroupInfo {
    pub id: String,
    pub class_id: String,
    pub abbrev: Option<String>,
    pub name: Option<String>,
}
