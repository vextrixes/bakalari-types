use crate::user::SettingModuleType;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KomensModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub upload_folder_verified: bool,
    pub current_school_year_start_date: DateTime<FixedOffset>,
}
