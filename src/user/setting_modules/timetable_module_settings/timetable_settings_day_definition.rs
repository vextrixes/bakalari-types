use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableSettingsDayDefinition {
    pub id: String,
    pub name: String,
    pub caption: String,
}
