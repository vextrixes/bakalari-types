use crate::payments::regularpayments::{PrescriptionDetailItem, PrescriptionState};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrescriptionDetail {
    pub id: i64,
    pub title: Option<String>,
    pub payment_type_id: i64,
    pub payment_type_name: Option<String>,
    pub amount: f64,
    pub state: PrescriptionState,
    pub due_date_max: DateTime<FixedOffset>,
    pub bank_account: Option<String>,
    pub specific_symbol: Option<i64>,
    pub items: Option<Vec<PrescriptionDetailItem>>,
}
