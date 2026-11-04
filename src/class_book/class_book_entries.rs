use crate::ClassBookDay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClassBookEntries {
    pub days: Vec<ClassBookDay>,
}
