use crate::{ClassBookGroupInfo, ClassBookLessonTag, ClassBookLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookSaveRecordRequest {
    id: String,
    lesson_id: String,
    class_book_level: ClassBookLevel,
    label: Option<String>,
    notice: Option<String>,
    private_notice: Option<String>,
    tags: Vec<ClassBookLessonTag>,
    theme: Option<String>,
    week_theme: Option<String>, //TODO: check nullability
    groups: Vec<ClassBookGroupInfo>,
}
