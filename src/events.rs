use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    events: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    id: String,
    title: String,
    description: String,
    times: Vec<Time>,
    event_type: StdType,
    classes: Vec<StdType>,
    //class_sets: unknown
    teachers: Vec<StdType>,
    //teacher_sets: unknown
    rooms: Vec<StdType>,
    //room_sets: unknown
    students: Vec<StdType>,
    note: Option<String>,
    date_changed: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Time {
    whole_day: bool,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}
