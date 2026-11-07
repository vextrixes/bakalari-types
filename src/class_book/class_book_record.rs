use crate::{ClassBookGroupInfo, ClassBookLessonTag, ClassBookLevel, StdTypeAbbrevNullable};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookRecord {
    pub id: String,
    pub lesson_id: String,
    pub tags: Vec<ClassBookLessonTag>,
    pub hour_id: i64,
    pub groups: Vec<ClassBookGroupInfo>,
    pub editable_groups: Vec<String>,
    pub start: i64,
    pub duration: i64,
    pub label: Option<String>,
    pub notice: Option<String>,
    pub private_notice: Option<String>,
    pub theme: Option<String>,
    pub week_theme: Option<String>,
    pub date: DateTime<FixedOffset>,
    pub hour_caption: Option<String>,
    pub subject: Option<StdTypeAbbrevNullable>,
    pub change_text: Option<String>,
    pub supervision: Option<StdTypeAbbrevNullable>,
    pub class_book_level: ClassBookLevel,
}
