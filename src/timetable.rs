use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    pub hours: Vec<TimetableHour>,
    pub days: Vec<TimetableDay>,
    pub classes: Vec<StdType>,
    pub groups: Vec<TimetableGroup>,
    pub subjects: Vec<StdType>,
    pub teachers: Vec<StdType>,
    pub rooms: Vec<StdType>,
    pub cycles: Vec<StdType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableHour {
    pub id: u8,
    pub caption: String,
    pub begin_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableDay {
    pub atoms: Vec<TimetableAtom>,
    pub day_of_week: u8,
    pub date: DateTime<Utc>,
    pub day_description: String,
    pub day_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableAtom {
    pub hour_id: u8,
    pub group_ids: Vec<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub room_id: Option<String>,
    pub cycle_ids: Vec<String>,
    pub change: Option<TimetableChange>,
    pub homework_ids: Vec<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableChange {
    pub change_subject: Option<String>,
    pub day: DateTime<Utc>,
    pub hours: String,
    pub change_type: String,
    pub description: String,
    pub time: String,
    pub type_abbrev: Option<String>,
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableGroup {
    pub class_id: String,
    pub id: String,
    pub abbrev: String,
    pub name: String,
}
