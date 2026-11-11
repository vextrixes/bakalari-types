use crate::classbook::{GroupInfo, LessonTag, Level};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaveRecordRequest {
    id: String,
    lesson_id: String,
    class_book_level: Level,
    label: Option<String>,
    notice: Option<String>,
    private_notice: Option<String>,
    tags: Vec<LessonTag>,
    theme: Option<String>,
    week_theme: Option<String>,
    groups: Vec<GroupInfo>,
}
