use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchoolType {
    BasicArtSchool,
    BasicPracticalSchool,
    BasicSchool,
    BasicSpecialSchool,
    BoardingHouse,
    ChildrensHome,
    Club,
    Conservatory,
    EconomicSection,
    Gymnasium,
    HigerProfessionalSchool,
    Kindergarten,
    OtherSchool,
    OtherType,
    PracticalSchool,
    SchoolCanteen,
    SchoolClub,
    SchoolService,
    SecondarySchool,
    SecondaryVocationalSchool,
    SportGymnasium,
    SuperiorOrganization,
    TechnicalSchool,
    University,
    Unknown,
    VocationalSchool,
}
