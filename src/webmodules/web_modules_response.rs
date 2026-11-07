use crate::webmodules::{WebModule, WebModuleDashboard};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebModulesResponse {
    pub web_modules: Vec<WebModule>,
    pub dashboard: WebModuleDashboard,
}
