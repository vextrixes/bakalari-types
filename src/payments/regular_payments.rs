use crate::payments::PaymentsTeacherStudentStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPaymentDetail {
    pub title: Option<String>,
    pub due_date: DateTime<FixedOffset>,
    pub amount: f64,
    pub original_amount: f64,
    pub status: RegularPaymentStatus,
    pub bank_account: Option<String>,
    pub variable_symbol: Option<i64>,
    pub specific_symbol: Option<i64>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPayment {
    pub payment_id: i64,
    pub title: Option<String>,
    pub due_date: DateTime<FixedOffset>,
    pub amount: f64,
    pub status: RegularPaymentStatus,
    pub original_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPaymentsSummary {
    pub balance: f64,
    pub paid: f64,
    pub amount_scheduled: f64,
    pub student_name_with_class: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionDetailItem {
    pub due_date: DateTime<FixedOffset>,
    pub title: Option<String>,
    pub amount: f64,
    pub advance_notice_date: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionDetail {
    pub id: i64,
    pub title: Option<String>,
    pub payment_type_id: i64,
    pub payment_type_name: Option<String>,
    pub amount: f64,
    pub state: RegularPrescriptionState,
    pub due_date_max: DateTime<FixedOffset>,
    pub bank_account: Option<String>,
    pub specific_symbol: Option<i64>,
    pub items: Option<Vec<RegularPrescriptionDetailItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionDocumentLine {
    pub date: DateTime<FixedOffset>,
    pub title: Option<String>,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionRegulationLine {
    pub due_date: DateTime<FixedOffset>,
    pub title: Option<String>,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionStudentDetail {
    pub id: i64,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub title: Option<String>,
    pub balance: f64,
    pub total_regulation_amount: f64,
    pub status: PaymentsTeacherStudentStatus,
    pub document_lines: Option<Vec<RegularPrescriptionDocumentLine>>,
    pub regulation_lines: Option<Vec<RegularPrescriptionRegulationLine>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescriptionStudent {
    pub id: i64,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub balance: f64,
    pub status: PaymentsTeacherStudentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescription {
    pub id: i64,
    pub title: Option<String>,
    pub payment_type_id: i64,
    pub payment_type_name: Option<String>,
    pub amount: f64,
    pub state: PaymentsTeacherStudentStatus,
    pub due_date_max: Option<DateTime<FixedOffset>>,
    pub students_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegularPaymentStatus {
    BeforeDueDate,
    NotPaid,
    Paid,
    PartiallyPaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegularPrescriptionState {
    Paid,
    PaidNotActive,
    #[serde(rename = "unknown")]
    Unknown,
    Unpaid,
    UnpaidNotActive,
}
