use crate::{
    gdpr::{ConsentRights, ConsentState, Validity},
    user::UserType,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Consent {
    pub id: i64,
    pub person_type: UserType,
    pub person_id: String,
    pub created: DateTime<FixedOffset>,
    pub validity: Validity,
    pub valid_from: DateTime<FixedOffset>,
    pub state: ConsentState,
    pub title: Option<String>,
    pub personal_data: Option<String>,
    pub usage_purpose: Option<String>,
    pub valid_to: Option<DateTime<FixedOffset>>,
    pub valid_years: Option<i64>,
    pub rights: Option<ConsentRights>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Consents {
    pub person_consents: Vec<Consent>,
}
