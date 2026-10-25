use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdType {
    pub id: String,
    pub abbrev: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdClassGroup {
    pub class_id: String,
    pub id: String,
    pub abbrev: String,
    pub name: String,
}
