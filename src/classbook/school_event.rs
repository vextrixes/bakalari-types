use crate::classbook::SchoolEventInterval;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchoolEvent {
    pub student_id_set: Vec<String>,
    pub event_title: String,
    pub event_id: String,
    pub school_event_interval_set: Vec<SchoolEventInterval>,
}
