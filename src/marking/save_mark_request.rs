use crate::marks::MarkConfirmationStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct SaveMarkRequest {
    pub mark_date: DateTime<FixedOffset>,
    pub publish_date: Option<DateTime<FixedOffset>>,
    pub teacher_id: Option<String>, //check nullability
    pub header_id: String,
    pub mark_value: String,
    pub theme: Option<String>, //check nullability
    pub student_id: String,
    pub mark_confirmation_state: Option<MarkConfirmationStatus>, //check nullability
    pub verbal_evaluation: Option<String>,                       //check nullability
    pub internal_note: Option<String>,                           //check nullability
    pub id: String,
}
