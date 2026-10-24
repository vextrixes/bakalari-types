use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
    message: String,
    model_state: ModelState,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelState {
    #[serde(rename = "$type")]
    type_name: String,
    #[serde(rename = "from")]
    from: StringArray,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StringArray {
    #[serde(rename = "$type")]
    type_name: String,
    #[serde(rename = "$values")]
    values: Vec<String>,
}