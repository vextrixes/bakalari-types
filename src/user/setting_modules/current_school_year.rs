use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentSchoolYear {
    pub id: String,
    pub name: String,
    pub effective_start_date: DateTime<FixedOffset>,
    pub effective_end_date: DateTime<FixedOffset>,
}
