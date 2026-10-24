use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
    pub message: String,
    pub model_state: ModelState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelState {
    #[serde(rename = "$type")]
    pub type_name: String,
    #[serde(rename = "from")]
    pub from: StringArray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringArray {
    #[serde(rename = "$type")]
    pub type_name: String,
    #[serde(rename = "$values")]
    pub values: Vec<String>,
}
