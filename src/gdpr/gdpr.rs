use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::UserType;

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
    pub person_type: UserType,
    pub person_id: String,
    pub created: DateTime<FixedOffset>,
    pub validity: GdprValidity,
    pub valid_from: DateTime<FixedOffset>,
    pub state: GdprConsentState,
    pub title: Option<String>,
    pub personal_data: Option<String>,
    pub usage_purpose: Option<String>,
    pub valid_to: Option<DateTime<FixedOffset>>,
    pub valid_years: Option<i64>,
    pub rights: Option<GdprConsentRights>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GdprValidity {
    EndOfStudy,
    EndOfStudyPlusYears,
    ToAppeal,
    Unknown,
    Validity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GdprConsentState {
    Agreed,
    Cancelled,
    Disagreed,
    Expired,
    NewConsent,
    RevokeRequested,
    Revoked,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprConsentRights {
    pub can_agree: bool,
    pub can_disagree: bool,
    pub can_revoke_request: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GdprConsents {
    pub person_consents: Vec<GdprConsent>,
}
