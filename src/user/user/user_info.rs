use crate::{
    common_types::StdType,
    user::{Module, ModulesSettings, SchoolType, StudentUserInfo, UserType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    #[serde(rename = "UserUID")]
    pub user_uid: String,
    pub campaign_category_code: String,
    pub class: Option<StdType>,
    pub teacher_classes: Option<Vec<StdType>>,
    pub full_name: Option<String>,
    pub school_organization_name: Option<String>,
    pub school_type: Option<SchoolType>,
    pub user_type: UserType,
    pub user_type_text: String,
    pub study_year: Option<i64>,
    pub enabled_modules: Vec<Module>,
    pub setting_modules: ModulesSettings,
    pub full_user_name: Option<String>,
    pub students: Vec<StudentUserInfo>,
}
