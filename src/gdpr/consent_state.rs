use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConsentState {
    Agreed,
    Cancelled,
    Disagreed,
    Expired,
    NewConsent,
    RevokeRequested,
    Revoked,
    Unknown,
}
