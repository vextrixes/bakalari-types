use crate::timetable::{AssistanceAtom, Atom};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Day {
    pub atoms: Vec<Atom>,
    pub day_of_week: i64,
    pub date: DateTime<FixedOffset>,
    pub day_description: String,
    pub day_type: DateTime<FixedOffset>,
    pub assistance_atoms: Vec<AssistanceAtom>,
}
