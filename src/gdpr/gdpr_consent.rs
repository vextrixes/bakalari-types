use crate::{GdprConsentRights, GdprConsentState, GdprValidity, UserType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "PascalCase")]
pub struct GdprConsents {
    pub person_consents: Vec<GdprConsent>,
}
