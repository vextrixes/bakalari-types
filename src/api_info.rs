use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiInfo {
    pub api_version: String,
    pub application_version: String,
    pub base_url: String,
}
