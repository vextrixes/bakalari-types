use crate::user::{
    SettingModuleType, TimetableDisplayType, TimetableHeaderType, TimetableSettingsDayDefinition,
    TimetableSettingsHourDefinition,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub display_type: TimetableDisplayType,
    pub header_type: TimetableHeaderType,
    pub days_definition: Vec<TimetableSettingsDayDefinition>,
    pub hours_definition: Vec<TimetableSettingsHourDefinition>,
}
