use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableSettingsHourDefinition {
    pub id: i64,
    pub caption: String,
    pub start: i64,
    pub duration: i64,
    pub is_visible: bool,
}
