use crate::payments::PaymentsTeacherStudentStatus;
use crate::payments::regularpayments::{
    PrescriptionDocumentLine, PrescriptionRegulationLine,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrescriptionStudentDetail {
    pub id: i64,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub title: Option<String>,
    pub balance: f64,
    pub total_regulation_amount: f64,
    pub status: PaymentsTeacherStudentStatus,
    pub document_lines: Option<Vec<PrescriptionDocumentLine>>,
    pub regulation_lines: Option<Vec<PrescriptionRegulationLine>>,
}
