use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LessonTag {
    pub id: String,
    pub notice: String,
    pub abbreviation: Option<String>,
    pub title: Option<String>,
}
