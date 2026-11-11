use crate::payments::PaymentsTeacherStudentStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Prescription {
    pub id: i64,
    pub title: Option<String>,
    pub payment_type_id: i64,
    pub payment_type_name: Option<String>,
    pub amount: f64,
    pub state: PaymentsTeacherStudentStatus,
    pub due_date_max: Option<DateTime<FixedOffset>>,
    pub students_count: i64,
}
