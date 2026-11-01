use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WebModuleIcon {
    Akademie,
    Ankety,
    Dokumenty,
    Druzina,
    Potvrzeni,
    VyukoveZdroje,
    WebovyModul,
}
