use crate::payments::regularpayments::PaymentStatus;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    pub payment_id: i64,
    pub title: Option<String>,
    pub due_date: DateTime<FixedOffset>,
    pub amount: f64,
    pub status: PaymentStatus,
    pub original_amount: f64,
}
