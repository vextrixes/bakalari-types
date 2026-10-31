use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Campaign {
    pub message_id: i64,
    pub priority: i64,
    pub authorized: bool,
    pub consent_required: bool,
    pub delayed_show: i64,
    pub image_url: String,
    pub link_url: Option<String>,
    pub salt: Option<String>,
}
