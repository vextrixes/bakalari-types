use serde::{Deserialize, Serialize};

use crate::{WebModuleIcon, WebModuleSubMenu};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebModuleDashboard {
    pub icon_id: WebModuleIcon,
    pub url: String,
    pub name: Option<String>,
    pub sub_menu: Vec<WebModuleSubMenu>,
}
