use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subjects {
    pub subjects: Vec<Subject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subject {
    pub subject_id: String,
    pub subject_name: String,
    pub subject_abbrev: String,
    pub teacher_id: String,
    pub teacher_name: String,
    pub teacher_abbrev: String,
    pub teacher_email: Option<String>,
    pub teacher_web: Option<String>,
    pub teacher_school_phone: Option<String>,
    pub teacher_home_phone: Option<String>,
    pub teacher_mobile_phone: Option<String>,
}
