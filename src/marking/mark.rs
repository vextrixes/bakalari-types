use crate::marks::MarkConfirmationStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct Mark {
    pub id: String,
    pub edit_date: DateTime<FixedOffset>,
    pub read_only: bool,
    pub mark_date: DateTime<FixedOffset>,
    pub header_id: String,
    pub mark_value: String,
    pub student_id: String,
    pub read_only_description: Option<String>,
    pub publish_date: Option<DateTime<FixedOffset>>,
    pub teacher_id: Option<String>,
    pub mark_confirmation_state: Option<MarkConfirmationStatus>,
    pub theme: Option<String>,
    pub verbal_evaluation: Option<String>,
    pub internal_note: Option<String>,
    pub confirmed_when: Option<DateTime<FixedOffset>>,
    pub confirmed_by: Option<String>,
}
