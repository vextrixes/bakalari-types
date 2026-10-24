use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    id: String,
    name: String,
    r#type: String,
    size: u64,
}
