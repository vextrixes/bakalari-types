use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SettingModuleType {
    KomensModuleSettings,
    MarksModuleSettings,
    TimetableModuleSettings,
    HomeworksModuleSettings,
    ClassBookModuleSettings,
    CommonModuleSettings,
    EmergencyModuleSettings,
}
