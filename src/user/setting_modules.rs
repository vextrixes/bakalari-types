use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesSetting {
    pub common: CommonModuleSettings,
    pub komens: Option<KomensModuleSettings>,
    pub marking: Option<MarkinModuleSettings>,
    pub timetable: Option<TimetableModuleSettings>,
    pub class_book: Option<ClassbookModuleSettings>,
    pub emergency: EmergencyModuleSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub actual_semester: ActualSemester,
    pub actual_school_year: Option<ActualSchoolYear>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActualSemester {
    pub semester_id: Option<String>,
    pub from: DateTime<FixedOffset>,
    pub to: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActualSchoolYear {
    pub id: String,
    pub name: String,
    pub effective_start_date: DateTime<FixedOffset>,
    pub effective_end_date: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KomensModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub upload_folder_verified: bool,
    pub current_school_year_start_date: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkinModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub required_classification_confirmation: bool,
    pub classification_confirmation_classes_list: Vec<String>,
    pub classification_confirmation_notification: bool,
    pub number_of_days_classification_confirmation_notifications: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub display_type: TimetableDisplayType,
    pub header_type: TimetableHeaderType,
    pub days_definition: Vec<TimetableSettingsDayDefinition>,
    pub hours_definition: Vec<TimetableSettingsHourDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TimetableDisplayType {
    Classic,
    Flexi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TimetableHeaderType {
    HourIndex,
    TimeLine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableSettingsDayDefinition {
    pub id: String,
    pub name: String,
    pub caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableSettingsHourDefinition {
    pub id: i64,
    pub caption: String,
    pub start: i64,
    pub duration: i64,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassbookModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub absence_calculation_type: AbsenceCalculationType,
    pub absence_calculation_period: ActualSemester,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AbsenceCalculationType {
    Duration,
    Lessons,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmergencyModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: ModulesSettingType,
    pub firestore_db_name: String,
    pub emergency_server_url: String,
    pub is_sms_licensed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ModulesSettingType {
    KomensModuleSettings,
    MarksModuleSettings,
    TimetableModuleSettings,
    HomeworksModuleSettings,
    ClassBookModuleSettings,
    CommonModuleSettings,
    EmergencyModuleSettings,
}
