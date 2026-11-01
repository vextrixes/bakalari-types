use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprConsentRights {
    pub can_agree: bool,
    pub can_disagree: bool,
    pub can_revoke_request: bool,
}
