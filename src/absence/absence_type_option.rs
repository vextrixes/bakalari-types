use crate::{absence::AbsentType, common::StdType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbsenceTypeOption {
    pub absent_type: AbsentType,
    pub absent_kind_enabled: bool,
    pub absent_kind_required: bool,
    pub absent_note_enabled: bool,
    pub next_absent_type: AbsentType,
    pub absent_kinds: Vec<StdType>,
}
