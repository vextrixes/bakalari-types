use crate::user::{AbsenceCalculationType, CurrentSemester, SettingModuleType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassbookModuleSettings {
    #[serde(rename = "$type")]
    pub r#type: SettingModuleType,
    pub absence_calculation_type: AbsenceCalculationType,
    pub absence_calculation_period: Vec<CurrentSemester>,
}
