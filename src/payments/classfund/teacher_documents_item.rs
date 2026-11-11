use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherDocumentsItem {
    pub id: i64,
    pub title: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub total_amount: f64,
    pub amount_per_student: f64,
    pub student_count: i64,
}
