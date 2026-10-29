use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassfundSummary {
    pub student_name_with_class: Option<String>,
    pub balance: f64,
    pub spent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundPaymentInfo {
    pub instructions: Option<String>,
    pub bank_account: Option<String>,
    pub variable_symbol: Option<i64>,
    pub specific_symbol: i64,
    pub amount: f64,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundMonthly {
    pub date: DateTime<FixedOffset>,
    pub monthly_balance: f64,
    pub items: Vec<ClassFundRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundRecord {
    pub description: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundSummary {
    pub student_name_with_class: Option<String>,
    pub balance: f64,
    pub spent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentDetailStudent {
    pub full_name_with_class: Option<String>,
    pub amount: f64,
    pub status: Option<PaymentsTeacherStudentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentDetail {
    pub title: Option<String>,
    pub amount: f64,
    pub date: DateTime<FixedOffset>,
    pub student_count: i64,
    pub students: Option<Vec<ClassFundTeacherDocumentDetailStudent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentsItem {
    pub id: i64,
    pub title: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub total_amount: f64,
    pub amount_per_student: f64,
    pub student_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentsMonthly {
    pub date: DateTime<FixedOffset>,
    pub monthly_balance: f64,
    pub items: Vec<ClassFundTeacherDocumentsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherStudentsItem {
    pub variable_symbol: i64,
    pub name: Option<String>,
    pub registration_number: Option<i64>,
    pub balance: f64,
    pub status: PaymentsTeacherStudentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherSummaryClass {
    pub abbrev: Option<String>,
    pub class_id: Option<String>,
    pub balance: f64,
    pub amount_to_pay: f64,
    pub debtors: i64,
    pub spent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherSummary {
    pub teacher_name: Option<String>,
    pub allowed_classes: Option<Vec<ClassFundTeacherSummaryClass>>,
}

//------------------------------------------------------------------------------------------------------------------------------------------------------------

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
    pub state: RegularPrescription,
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
pub enum PaymentsTeacherStudentStatus {
    Active,
    Canceled,
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
