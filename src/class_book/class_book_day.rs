use crate::class_book::{ClassBookDayType, ClassBookRecord};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookDay {
    pub day_of_week: i64,
    pub day_description: Option<String>,
    pub day_type: ClassBookDayType,
    pub hours: Vec<ClassBookRecord>,
    pub date: DateTime<FixedOffset>,
}
