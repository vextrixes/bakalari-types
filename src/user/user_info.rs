use serde::{Deserialize, Serialize};

use crate::{Module, SchoolClass, user::ModulesSetting};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfo {
    #[serde(rename = "UserUID")]
    pub user_uid: String,
    pub campaign_category_code: String,
    pub class: Option<SchoolClass>,
    pub teacher_classes: Option<Vec<SchoolClass>>,
    pub full_name: Option<String>,
    pub school_organization_name: Option<String>,
    pub school_type: Option<SchoolType>,
    pub user_type: UserType,
    pub user_type_text: String,
    pub study_year: Option<i64>,
    pub enabled_modules: Vec<Module>,
    pub setting_modules: ModulesSetting,
    pub full_user_name: Option<String>,
    pub students: Vec<UserInfoStudent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserType {
    Controller,
    Parents,
    Student,
    Teacher,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserInfoStudent {
    pub full_name: String,
    pub class: SchoolClass,
    pub school_type: SchoolType,
    pub study_year: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchoolType {
    BasicArtSchool,
    BasicPracticalSchool,
    BasicSchool,
    BasicSpecialSchool,
    BoardingHouse,
    ChildrensHome,
    Club,
    Conservatory,
    EconomicSection,
    Gymnasium,
    HigerProfessionalSchool,
    Kindergarten,
    OtherSchool,
    OtherType,
    PracticalSchool,
    SchoolCanteen,
    SchoolClub,
    SchoolService,
    SecondarySchool,
    SecondaryVocationalSchool,
    SportGymnasium,
    SuperiorOrganization,
    TechnicalSchool,
    University,
    Unknown,
    VocationalSchool,
}
