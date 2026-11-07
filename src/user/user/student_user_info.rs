use crate::{common_types::StdType, user::SchoolType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StudentUserInfo {
    pub full_name: String,
    pub class: StdType,
    pub school_type: SchoolType,
    pub study_year: Option<i64>,
}
