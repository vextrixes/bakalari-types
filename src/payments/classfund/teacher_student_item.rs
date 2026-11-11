use crate::payments::PaymentsTeacherStudentStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherStudentsItem {
    pub variable_symbol: i64,
    pub name: Option<String>,
    pub registration_number: Option<i64>,
    pub balance: f64,
    pub status: PaymentsTeacherStudentStatus,
}
