use crate::{subjects::Subject, teaching::TeachingTheme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teaching {
    pub subject: Subject,
    pub themes: Vec<TeachingTheme>,
}
