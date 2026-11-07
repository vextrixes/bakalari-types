use crate::user::SettingModuleType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkingModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub required_classification_confirmation: bool,
    pub classification_confirmation_classes_list: Vec<String>,
    pub classification_confirmation_notification: bool,
    pub number_of_days_classification_confirmation_notifications: i64,
}
