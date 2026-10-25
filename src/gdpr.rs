use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GDPRcommissioners {
    pub commissioners: Vec<GDPRcommissioner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GDPRcommissioner {
    pub id: String,
    pub name: String,
    pub mobile: String,
    pub phone: String,
    pub email: String,
    pub web: String,
    pub data_box: String,
}
