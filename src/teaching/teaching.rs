use crate::{Subject, TeachingTheme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teaching {
    pub subject: Subject,
    pub themes: Vec<TeachingTheme>,
}
