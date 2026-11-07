use crate::user::{CurrentSchoolYear, CurrentSemester, SettingModuleType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub actual_semester: CurrentSemester,
    pub actual_school_year: Option<CurrentSchoolYear>,
}
