use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub enum MarkCalculationType {
    CalculateMarks,
    CalculateMarksWithMinus,
    NoCalculation,
}
