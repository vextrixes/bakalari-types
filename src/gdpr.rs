use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprCommissioners {
    pub commissioners: Vec<GdprCommissioner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprCommissioner {
    pub id: String,
    pub name: String,
    pub mobile: String,
    pub phone: String,
    pub email: String,
    pub web: Option<String>,
    pub data_box: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprConsent {
    pub id: i64,
    pub person_type: String, //UserType Enum "controller","parents","student","teacher"
    pub person_id: String,
    pub created: DateTime<FixedOffset>,
    pub validity: String, //enum "endOfStudy","endOfStudyPlusYears","toAppeal","unknown", "validity"
    pub valid_from: DateTime<FixedOffset>,
    pub state: String, //Enum   ConsentState "agreed", "cancelled", "disagreed", "expired", "newConsent", "revokeRequested", "revoked", "unknown"
    pub title: Option<String>,
    pub personal_data: Option<String>,
    pub usage_purpose: Option<String>,
    pub valid_to: Option<DateTime<FixedOffset>>,
    pub valid_years: Option<i64>,
    pub rights: Option<GdprConsentRights>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprConsentRights {
    pub can_agree: bool,
    pub can_disagree: bool,
    pub can_revoke_request: bool,
}
