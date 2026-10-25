use crate::StdType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Themes {
    pub subject: StdType,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Theme {
    pub date: DateTime<Utc>,
    pub theme: String,
    pub note: String,
    pub hour_caption: String,
    pub lesson_label: String,
}
