use crate::{MarkCalculationType, MarkingType};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct MarkingHeaderDefinitions {
    pub header_definitions: Vec<MarkingHeaderDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct MarkingHeaderDefinition {
    pub theme: String,
    pub caption: String,
    pub marking_type: MarkingType,
    pub mark_calculation_type: MarkCalculationType,
    pub id: Option<String>,
    pub color: Option<String>,
    pub order: Option<i64>,
    pub mark_date: Option<DateTime<FixedOffset>>,
    pub publish_date: Option<DateTime<FixedOffset>>,
    pub points_max: Option<i64>,
    pub mark_weight_id: Option<String>,
    pub mark_calculation_boundaries: Option<Vec<i64>>,
}
