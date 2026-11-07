use crate::user::SettingModuleType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmergencyModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub firestore_db_name: String,
    pub emergency_server_url: String,
    pub is_sms_licensed: bool,
}
