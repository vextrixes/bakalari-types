use crate::{marking::TypeOptions, common::StdType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "PascalCase")]
pub struct HeaderOptions {
    pub mark_weight_options: Vec<StdType>,
    pub marking_type_options: TypeOptions,
}
