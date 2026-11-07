use serde::{Deserialize, Serialize};

use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct MarkingAtom {
    pub id: String,
    pub disabled: bool,
    pub subject: StdType,
    pub name: Option<String>,
    pub disabled_text: Option<String>,
}
