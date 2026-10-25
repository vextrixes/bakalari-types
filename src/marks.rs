use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Marks {
    pub subjects: Vec<SubjectMarks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubjectMarks {
    pub marks: Vec<Mark>,
    pub subject: StdType,
    pub average_text: String,
    pub temporary_mark: String,
    pub subject_note: String,
    pub temporary_mark_note: String,
    pub points_only: bool,
    pub mark_prediction_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mark {
    pub mark_date: DateTime<Utc>,
    pub edit_date: DateTime<Utc>,
    pub caption: String,
    pub theme: String,
    pub mark_text: String,
    pub teacher_id: String,
    pub r#type: String,
    pub type_note: String,
    pub weight: u8,
    pub subject_id: String,
    pub is_new: bool,
    pub is_points: bool,
    pub calculated_mark_text: String,
    //ClassRankText:null,
    pub id: String,
    pub points_text: String,
    pub max_points: u16,
    pub confirmed_when: Option<DateTime<Utc>>,
    pub confirmed_by: Option<String>,
    pub mark_confirmation_state: String,
}
