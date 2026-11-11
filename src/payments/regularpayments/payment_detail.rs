use crate::payments::regularpayments::PaymentStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentDetail {
    pub title: Option<String>,
    pub due_date: DateTime<FixedOffset>,
    pub amount: f64,
    pub original_amount: f64,
    pub status: PaymentStatus,
    pub bank_account: Option<String>,
    pub variable_symbol: Option<i64>,
    pub specific_symbol: Option<i64>,
    pub message: Option<String>,
}
