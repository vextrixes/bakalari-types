use crate::payments::PaymentsTeacherStudentStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherDocumentDetailStudent {
    pub full_name_with_class: Option<String>,
    pub amount: f64,
    pub status: Option<PaymentsTeacherStudentStatus>,
}
