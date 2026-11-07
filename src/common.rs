use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdType {
    pub id: String,
    pub abbrev: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdTypeNullable {
    pub id: Option<String>,
    pub abbrev: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdTypeAbbrevNullable {
    pub id: String,
    pub abbrev: Option<String>,
    pub name: String,
}
