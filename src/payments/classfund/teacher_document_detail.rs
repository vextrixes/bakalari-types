use crate::payments::classfund::TeacherDocumentDetailStudent;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherDocumentDetail {
    pub title: Option<String>,
    pub amount: f64,
    pub date: DateTime<FixedOffset>,
    pub student_count: i64,
    pub students: Option<Vec<TeacherDocumentDetailStudent>>,
}
