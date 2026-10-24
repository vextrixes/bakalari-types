use crate::StdType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub times: Vec<Time>,
    pub event_type: StdType,
    pub classes: Vec<StdType>,
    //class_sets: unknown
    pub teachers: Vec<StdType>,
    //teacher_sets: unknown
    pub rooms: Vec<StdType>,
    //room_sets: unknown
    pub students: Vec<StdType>,
    pub note: Option<String>,
    pub date_changed: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Time {
    pub whole_day: bool,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
