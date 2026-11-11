use crate::classbook::{DayType, Record};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Day {
    pub day_of_week: i64,
    pub day_description: Option<String>,
    pub day_type: DayType,
    pub hours: Vec<Record>,
    pub date: DateTime<FixedOffset>,
}
