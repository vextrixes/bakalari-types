use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeLessonInfo {
    class_abbrev: Option<String>,
    group_abbrev: Option<String>,
    is_class_group: Option<bool>,
    subject_abbrev: Option<String>,
    room_abbrev: Option<String>,
}
