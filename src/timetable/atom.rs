use crate::timetable::{
    Assistant, HomeworkInfo, LessonRelease, PlannedClassification, Change,
};
use serde::{Deserialize, Serialize};

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
    pub change: Option<Change>,
    pub theme: Option<String>,
    pub assistants: Vec<Assistant>,
    pub notice: Option<String>,
    #[serde(rename = "PlannedClasification")]
    pub planned_classification: Option<PlannedClassification>,
    pub week_theme: Option<String>,
    pub lesson_release: LessonRelease,
    pub start: Option<i64>,
    pub duration: Option<i64>,
}
