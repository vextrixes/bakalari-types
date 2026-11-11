use crate::classbook::Day;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Entries {
    pub days: Vec<Day>,
}
