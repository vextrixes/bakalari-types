use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeachingTheme {
    pub date: DateTime<FixedOffset>,
    pub theme: String,
    pub note: String,
    pub lesson_label: String,
    pub hour_caption: Option<String>,
    pub start: Option<u16>,
    pub duration: Option<u16>,
}
