use crate::MarkingType;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct MarkingHeader {
    pub id: String,
    pub order: i64,
    pub points_max: i64,
    pub theme: String,
    pub caption: String,
    pub average_text: String,
    pub weight: i64,
    pub fill_mark_date: bool,
    pub fill_theme: bool,
    pub marking_type: MarkingType,
    pub color: String,
    pub mark_weight_name: String,
    pub mark_date: Option<DateTime<FixedOffset>>,
    pub publish_date: Option<DateTime<FixedOffset>>,
}
