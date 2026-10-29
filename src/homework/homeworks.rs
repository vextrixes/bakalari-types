use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::{AttachmentInfo, StdType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworkList {
    pub homeworks: Vec<HomeworkListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HomeworkListItem {
    #[serde(rename = "ID")]
    pub id: String,
    pub date_start: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
    pub content: String,
    pub notice: Option<String>,
    pub done: bool,
    pub closed: bool,
    pub electronic: bool, //check existance
    pub hour: i64,
    pub class: StdType,   //HomeworkSchoolClass
    pub group: StdType,   //HomeworkSchoolGroup
    pub subject: StdType, //HomeworkSchoolSubject
    pub teacher: StdType, //HomeworkTeacher
    pub attachments: Vec<AttachmentInfo>,
    pub finished: bool,
}
