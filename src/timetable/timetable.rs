use crate::{
    common_types::StdType,
    timetable::{Day, Group, Hour},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timetable {
    pub hours: Vec<Hour>,
    pub days: Vec<Day>,
    pub classes: Vec<StdType>,
    pub groups: Vec<Group>,
    pub subjects: Vec<StdType>,
    pub rooms: Vec<StdType>,
    pub teachers: Vec<StdType>,
    pub cycles: Vec<StdType>,
    pub students: Vec<StdType>,
}
