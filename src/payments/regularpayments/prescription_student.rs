use crate::payments::PaymentsTeacherStudentStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrescriptionStudent {
    pub id: i64,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub balance: f64,
    pub status: PaymentsTeacherStudentStatus,
}
