use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::Subject;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teaching {
    pub subject: Subject,
    pub themes: Vec<TeachingTheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeachingTheme {
    pub date: DateTime<FixedOffset>,
    pub theme: String,
    pub note: String,
    pub lesson_label: String,
    pub hour_caption: Option<String>,
    pub start: Option<u16>,
    pub duration: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Lesson {
    pub subject_id: String,
    pub subject_name: String,
    pub subject_abbrev: String,
    pub teacher_id: String,
    pub teacher_name: String,
    pub teacher_abbrev: String,
    pub teacher_email: Option<String>,
    pub teacher_web: Option<String>,
    pub teacher_school_phone: Option<String>,
    pub teacher_home_phone: Option<String>,
    pub teacher_mobile_phone: Option<String>,
}
