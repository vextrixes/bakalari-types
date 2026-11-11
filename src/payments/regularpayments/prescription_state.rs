use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PrescriptionState {
    Paid,
    PaidNotActive,
    #[serde(rename = "unknown")]
    Unknown,
    Unpaid,
    UnpaidNotActive,
}
