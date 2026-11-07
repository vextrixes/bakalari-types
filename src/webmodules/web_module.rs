use crate::webmodules::{WebModuleIcon, WebModuleSubMenu};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebModule {
    pub icon_id: WebModuleIcon,
    pub url: Option<String>,
    pub name: String,
    pub sub_menu: Vec<WebModuleSubMenu>,
}
