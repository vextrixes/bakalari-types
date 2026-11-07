use crate::substitutions::SubstitutionChange;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Substitutions {
    pub from: DateTime<FixedOffset>,
    pub to: DateTime<FixedOffset>,
    pub changes: Vec<SubstitutionChange>,
}
