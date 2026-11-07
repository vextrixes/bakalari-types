use crate::{
    marking::{Header, Mark},
    common::StdType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct AtomMarks {
    pub read_only: bool,
    pub mark_options: Vec<StdType>,
    pub marks: Vec<Mark>,
    pub headers: Vec<Header>,
    pub students: Vec<StdType>,
    pub teachers: Vec<StdType>,
    pub read_only_description: Option<String>,
}
