use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherSummaryClass {
    pub abbrev: Option<String>,
    pub class_id: Option<String>,
    pub balance: f64,
    pub amount_to_pay: f64,
    pub debtors: i64,
    pub spent: f64,
}
