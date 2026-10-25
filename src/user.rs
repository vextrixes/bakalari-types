use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(rename = "UserUID")]
    pub user_uid: String,
    pub campaign_category_code: String,
    pub class: StdType,
    // TeacherClasses: Option<> TODO unknown
    pub full_name: String,
    pub school_organization_name: String,
    pub school_type: Option<String>,
    pub user_type: String,
    pub user_type_text: String,
    pub study_year: u8,
    pub enabled_modules: Vec<Module>,
    pub setting_modules: HashMap<String, Value>,
    pub full_user_name: String,
    pub students: Vec<Student>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Module {
    pub module: String,
    pub rights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    pub full_name: String,
    pub class: Vec<StdType>,
    pub school_type: String,
    pub study_year: u8,
}
