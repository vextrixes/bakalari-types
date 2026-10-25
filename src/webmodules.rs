use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Webmodules {
    pub web_modules: Vec<Webmodule>,
    pub dashboard: Webmodule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Webmodule {
    pub icon_id: Option<String>,
    // "SubMenu":null, unknown type
    pub url: Option<String>,
    pub name: Option<String>,
}
