use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdType {
    pub id: String,
    pub abbrev: String,
    pub name: String,
}
