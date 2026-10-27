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
   instructions: Option<String>,
   bank_account: Option<String>,
   variable_symbol: Option<i64>,
   specific_symbol: i64,
   amount: f64,
   message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundRecord {
    description: Option<String>,
    date: DateTime<FixedOffset>,
    amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundSummary {
    student_name_with_class: Option<String>,
    balance: f64,
    spent: f64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentDetailStudent {
    full_name_with_class: Option<String>,
    amount: f64,
    status: Option<String> //Option<PaymentsTeacherStudentStatus>, enum "Active" "Canceled"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentDetail{
    title: Option<String>,
    amount: f64,
    date: DateTime<FixedOffset>,
    student_count: i64,
    students: Option<Vec<ClassFundTeacherDocumentDetailStudent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentsItem{
    id: i64,
    title: Option<String>,
    date: DateTime<FixedOffset>,
    total_amount: f64,
    amount_per_student: f64,
    student_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherDocumentsMonthly{
    date: DateTime<FixedOffset>,
    monthly_balance: f64,
    items: Vec<ClassFundTeacherDocumentsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherStudentsItem{
    variable_symbol: i64,
    name: Option<String>,
    registration_number: Option<i64>,
    balance: f64,
    status: String//PaymentsTeacherStudentStatus, enum "Active" "Canceled
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherSummaryClass{
    abbrev: Option<String>,
    class_id: Option<String>,
    balance: f64,
    amount_to_pay: f64,
    debtors: i64,
    spent: f64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassFundTeacherSummary{
    teacher_name: Option<String>,
    allowed_classes: Option<Vec<ClassFundTeacherSummaryClass>>,
}

//------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPaymentDetail{
    title: Option<String>,
    due_date: DateTime<FixedOffset>,
    amount: f64,
    original_amount: f64,
    status: String, //r16 = <RegularPaymentStatus>  "BeforeDueDate", "NotPaid", "Paid", "PartiallyPaid"
    bank_account: Option<String>,
    variable_symbol: Option<i64>,
    specific_symbol: Option<i64>,
    message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPayment{
    payment_id: i64,
    title: Option<String>,
    due_date: DateTime<FixedOffset>,
    amount: f64,
    status: String, //r16 = <RegularPaymentStatus>  "BeforeDueDate", "NotPaid", "Paid", "PartiallyPaid"
    original_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPaymentsSummary{
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
    pub state: String, //r16 = <RegularPrescriptionState>  "Paid", "PaidNotActive","unknown","Unpaid", "UnpaidNotActive"
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
    pub status: String, //PaymentsTeacherStudentStatus, enum "Active" "Canceled"
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
    pub status: String, //PaymentsTeacherStudentStatus, enum "Active" "Canceled"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularPrescription {
    pub id: i64,
    pub title: Option<String>,
    pub payment_type_id: i64,
    pub payment_type_name: Option<String>,
    pub amount: f64,
    pub state: String, //PaymentsTeacherStudentStatus, enum "Active" "Canceled"
    pub due_date_max: Option<DateTime<FixedOffset>>,
    pub students_count: i64,
}