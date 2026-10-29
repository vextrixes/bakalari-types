use crate::{Room, RoomSet, SchoolClass, SchoolClassSet, StdType, Student, Teacher, TeacherSet};
use chrono::{DateTime, FixedOffset};
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
    pub event_type: EventType,
    pub times: Vec<EventTime>,
    pub classes: Vec<SchoolClass>,
    pub class_sets: Vec<SchoolClassSet>,
    pub teachers: Vec<Teacher>,
    pub teacher_sets: Vec<TeacherSet>,
    pub rooms: Vec<Room>,
    pub room_sets: Vec<RoomSet>,
    pub students: Vec<Student>,
    pub note: Option<String>,
    pub date_changed: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventTime {
    pub whole_day: bool,
    pub start_time: Option<DateTime<FixedOffset>>,
    pub end_time: Option<DateTime<FixedOffset>>,
}

pub type EventType = StdType;
