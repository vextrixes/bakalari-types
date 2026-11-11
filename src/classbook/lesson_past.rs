use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonPast {
    pub lesson_id: String,
    pub last_theme: Option<String>,
    pub next_label: Option<String>,
}
