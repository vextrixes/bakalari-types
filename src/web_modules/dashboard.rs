use crate::web_modules::{Icon, SubMenu};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dashboard {
    pub icon_id: Icon,
    pub url: String,
    pub name: Option<String>,
    pub sub_menu: Vec<SubMenu>,
}
