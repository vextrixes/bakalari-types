use crate::{
    stdtype::StdType,
    timetable::{AtomType, Changetype},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableChange {
    pub day: DateTime<FixedOffset>,
    pub hours: String,
    pub changetype: Changetype,
    pub description: String,
    pub time: String,
    pub change_subject: Option<StdType>,
    pub type_abbrev: Option<String>,
    pub type_name: Option<String>,
    pub atom_type: Option<AtomType>,
    pub start: Option<i64>,
    pub duration: Option<i64>,
}
