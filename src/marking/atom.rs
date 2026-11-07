use crate::common_types::StdType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct Atom {
    pub id: String,
    pub disabled: bool,
    pub subject: StdType,
    pub name: Option<String>,
    pub disabled_text: Option<String>,
}
