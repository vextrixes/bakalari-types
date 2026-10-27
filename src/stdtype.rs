use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdType {
    pub id: String,
    pub abbrev: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StdTypeNullable {
    pub id: Option<String>,
    pub abbrev: Option<String>,
    pub name: Option<String>,
}

pub type SchoolClass = StdType;

pub type StdClassGroup = StdType;

pub type SchoolClassSet = StdType;

pub type Teacher = StdType;

pub type TeacherSet = StdType;

pub type Room = StdType;

pub type RoomSet = StdType;

pub type Student = StdType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimetableStudent {
    pub id: String,
    pub abbrev: Option<String>,
    pub name: String,
}
