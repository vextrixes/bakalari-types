use serde::{Deserialize, Serialize};

use crate::StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(rename = "UserUID")]
    pub user_uid: String,
    pub campaign_category_code: String,
    pub class: StdType,
    // TeacherClasses: Option TODO
    pub full_name: String,
    pub school_organization_name: String,
    pub school_type: Option<String>,
    pub user_type: String,
    pub user_type_text: String,
    pub study_year: u8,
    pub enabled_modules: Vec<Module>, //TODO: user SettingModules
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
    full_name: String,
    class: Vec<StdType>,
    school_type: String,
    study_year: u8,
}
