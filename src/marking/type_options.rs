use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct TypeOptions {
    pub points: bool,
    pub percentage: bool,
}
