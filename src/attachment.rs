use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub size: u64,
}
