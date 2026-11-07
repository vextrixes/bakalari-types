use crate::{events::EventTime, common_types::StdType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: StdType,
    pub times: Vec<EventTime>,
    pub classes: Vec<StdType>,
    pub class_sets: Vec<StdType>,
    pub teachers: Vec<StdType>,
    pub teacher_sets: Vec<StdType>,
    pub rooms: Vec<StdType>,
    pub room_sets: Vec<StdType>,
    pub students: Vec<StdType>,
    pub note: Option<String>,
    pub date_changed: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    pub events: Vec<Event>,
}
