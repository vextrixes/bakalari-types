use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::PaymentsTeacherStudentStatus;

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
