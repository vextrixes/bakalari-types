use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChangeLessonInfo {
    pub class_abbrev: Option<String>,
    pub group_abbrev: Option<String>,
    pub is_class_group: Option<bool>,
    pub subject_abbrev: Option<String>,
    pub room_abbrev: Option<String>,
}
