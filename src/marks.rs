use chrono::{DateTime, FixedOffset};
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
    pub points_only: bool,
    pub mark_prediction_enabled: bool,
    pub subject: StdType,
    pub average_text: Option<String>,
    pub temporary_mark: Option<String>,
    pub subject_note: Option<String>,
    pub temporary_mark_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mark {
    pub mark_date: DateTime<FixedOffset>,
    pub edit_date: DateTime<FixedOffset>,
    pub is_invalid_date: Option<bool>,
    pub teacher_id: Option<String>,
    pub subject_id: String,
    pub is_new: bool,
    pub is_points: bool,
    pub calculated_mark_text: String,
    pub max_points: i64,
    pub is_predictor_mark: bool,
    pub mark_confirmation_state: String, // Option<MarkConfirmationStatus> enum "Confirmed", "DoesNotRequireConfirmation","Unconfirmed", "UnconfirmedAfterChange"
    pub caption: Option<String>,
    pub theme: Option<String>,
    pub mark_text: Option<String>,
    pub r#type: Option<String>,
    pub type_note: Option<String>,
    pub weight: Option<i64>,
    pub class_rank_text: Option<String>,
    pub id: Option<String>,
    pub points_text: Option<String>,
    pub confirmed_when: Option<DateTime<FixedOffset>>,
    pub confirmed_by: Option<String>,
    pub verbal_evaluation: Option<String>,
}
