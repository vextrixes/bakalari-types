use crate::payments::classfund::TeacherDocumentsItem;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherDocumentsMonthly {
    pub date: DateTime<FixedOffset>,
    pub monthly_balance: f64,
    pub items: Vec<TeacherDocumentsItem>,
}
