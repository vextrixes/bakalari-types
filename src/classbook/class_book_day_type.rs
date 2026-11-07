use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClassBookDayType {
    Celebration,
    DirectorDay,
    Holiday,
    Undefined,
    Weekend,
    WorkDay,
}
