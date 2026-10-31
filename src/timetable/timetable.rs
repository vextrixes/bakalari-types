use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{Room, StdType, Student, Teacher};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    pub hours: Vec<Hour>,
    pub days: Vec<Day>,
    pub classes: Vec<StdType>,
    pub groups: Vec<Group>,
    pub subjects: Vec<StdType>,
    pub rooms: Vec<Room>,
    pub teachers: Vec<Teacher>,
    pub cycles: Vec<StdType>,
    pub students: Vec<Student>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Changetype {
    Absence,
    Added,
    Canceled,
    NoChange,
    Removed,
    RoomChanged,
    Substitution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AtomType {
    Hour,
    Pause,
    TimeBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlannedClassification {
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworkInfo {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    pub class_id: Option<String>,
    pub id: String,
    pub abbre: Option<String>,
    pub name: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Atom {
    pub hour_id: i64,
    pub group_ids: Vec<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub room_id: Option<String>,
    pub is_last_room_lesson: bool,
    pub cycle_ids: Vec<String>,
    pub homeworks: Vec<HomeworkInfo>,
    pub homework_ids: Vec<String>,
    pub change: Option<TimetableChange>,
    pub theme: Option<String>,
    pub assistants: Vec<Assistant>,
    pub notice: Option<String>,
    pub planned_clasification: Option<PlannedClasification>,
    pub week_theme: Option<String>,
    pub lesson_release: LessonRelease,
    pub start: Option<i64>,
    pub duration: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LessonRelease {
    NoChange,
    Released,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Assistant {
    pub id: String,
    pub is_present: bool,
    pub student_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssistanceAtom {
    pub hour_id: i64,
    pub start: i64,
    pub duration: i64,
    pub class_id: Option<String>,
    pub group_id: Option<String>,
    pub subject_id: Option<String>,
    pub teacher_id: Option<String>,
    pub room_id: Option<String>,
    pub student_id: Option<String>,
    pub change: Option<TimetableChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlannedClasification {
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hour {
    pub id: i64,
    pub caption: String,
    pub begin_time: String,
    pub end_time: String,
    pub start: Option<i64>,
    pub duration: Option<i64>,
}
