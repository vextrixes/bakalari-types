use crate::user::{
    ClassbookModuleSettings, CommonModuleSettings, EmergencyModuleSettings, KomensModuleSettings,
    MarkingModuleSettings, TimetableModuleSettings,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesSettings {
    pub common: CommonModuleSettings,
    pub komens: Option<KomensModuleSettings>,
    pub marking: Option<MarkingModuleSettings>,
    pub timetable: Option<TimetableModuleSettings>,
    pub class_book: Option<ClassbookModuleSettings>,
    pub emergency: EmergencyModuleSettings,
}
