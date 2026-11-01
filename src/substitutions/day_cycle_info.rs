use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubstitutionCycles {
    pub cycles_info: Option<Vec<DayCycleInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DayCycleInfo {
    pub date: DateTime<FixedOffset>,
    pub code: Option<String>,
    pub abbrev: Option<String>,
    pub name: Option<String>,
}
