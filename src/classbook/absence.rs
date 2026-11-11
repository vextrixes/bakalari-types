use crate::{absence::AbsentType, common_types::StdTypeAbbrevNullable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Absence {
    pub minutes: i64,
    pub absent_type: AbsentType,
    pub hour_id: String,
    pub start: i64,
    pub duration: i64,
    pub hour_index: Option<i64>,
    pub absent_kind_id: Option<String>,
    pub notice: Option<String>,
    pub subject: Option<StdTypeAbbrevNullable>,
}
