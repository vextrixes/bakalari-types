use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassfundSummary {
    pub student_name_with_class: String,
    pub balance: f64,
    pub spent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassfundPayments {
    pub monthly_data: Vec<PaymentMonthlyData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentMonthlyData {
    pub date: DateTime<Utc>,
    pub monthly_balance: i64,
    pub items: Vec<Payment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    pub description: String,
    pub date: DateTime<Utc>,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddBalance {
    pub instructions: String,
    pub bank_account: String,
    pub variable_symbol: u32,
    pub specific_symbol: u32,
    pub amount: i64,
    pub message: String,
}
