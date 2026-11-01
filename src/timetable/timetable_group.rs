use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    pub class_id: Option<String>,
    pub id: String,
    pub abbre: Option<String>,
    pub name: Option<String>,
}
