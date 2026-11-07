use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commissioner {
    pub id: String,
    pub name: String,
    pub mobile: String,
    pub phone: String,
    pub email: String,
    pub web: Option<String>,
    pub data_box: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commissioners {
    pub commissioners: Vec<Commissioner>,
}
