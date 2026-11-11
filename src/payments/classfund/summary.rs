use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Summary {
    pub student_name_with_class: Option<String>,
    pub balance: f64,
    pub spent: f64,
}
