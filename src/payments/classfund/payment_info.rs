use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentInfo {
    pub instructions: Option<String>,
    pub bank_account: Option<String>,
    pub variable_symbol: Option<i64>,
    pub specific_symbol: i64,
    pub amount: f64,
    pub message: Option<String>,
}
