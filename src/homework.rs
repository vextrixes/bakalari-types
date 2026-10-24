use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Attachment, StdType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Homework {
    pub homeworks: Vec<HomeworkListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworkListItem {
    #[serde(rename = "ID")]
    pub id: String,
    pub date_start: DateTime<Utc>,
    pub date_end: DateTime<Utc>,
    pub content: String,
    pub notice: String,
    pub done: bool,
    pub closed: bool,
    pub electronic: bool,
    pub hour: u8,
    pub class: StdType,
    pub group: StdType,
    pub subject: StdType,
    pub teacher: StdType,
    pub attachments: Vec<Attachment>,
    pub finished: bool,
}
