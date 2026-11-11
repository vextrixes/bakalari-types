use crate::payments::classfund::TeacherSummaryClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeacherSummary {
    pub teacher_name: Option<String>,
    pub allowed_classes: Option<Vec<TeacherSummaryClass>>,
}
