use crate::{others::AttachmentInfo, stdtype::StdType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Homework {
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
    pub class: StdType,
    pub group: StdType,
    pub subject: StdType,
    pub teacher: StdType,
    pub attachments: Vec<AttachmentInfo>,
    pub finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Homeworks {
    pub homeworks: Vec<Homework>,
}
