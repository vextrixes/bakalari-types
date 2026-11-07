use crate::{
    marks::Mark,
    common_types::{StdType, StdTypeAbbrevNullable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubjectMarks {
    pub marks: Vec<Mark>,
    pub points_only: bool,
    pub mark_prediction_enabled: bool,
    pub subject: StdType,
    pub average_text: Option<String>,
    pub temporary_mark: Option<String>,
    pub subject_note: Option<String>,
    pub temporary_mark_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Marks {
    pub subjects: Vec<SubjectMarks>,
    pub mark_options: Vec<StdTypeAbbrevNullable>,
}
