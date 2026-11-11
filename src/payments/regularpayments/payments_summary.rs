use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentsSummary {
    pub balance: f64,
    pub paid: f64,
    pub amount_scheduled: f64,
    pub student_name_with_class: Option<String>,
}
