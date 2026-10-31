mod absence;
mod others;
//campaign
mod auth;
mod endpoints;
mod error;
mod events;
mod gdpr;
//komens
mod homework; //TDOS
//marking
mod marks;
mod payments;
mod semester;
mod stdtype;
mod subjects;
mod substitutions;
mod teaching;
mod timetable;
mod user; //TODOS
//webmodules
//predictor

pub use absence::*;
pub use auth::*;
pub use endpoints::*;
pub use error::{Error, ModelState, StringArray};
pub use events::*;
pub use gdpr::*;
pub use homework::*;
pub use marks::*;
pub use others::*;
pub use payments::*;
pub use semester::*;
pub use stdtype::{
    Room, RoomSet, SchoolClass, SchoolClassSet, StdClassGroup, StdType, StdTypeNullable, Student,
    Teacher, TeacherSet, TimetableStudent,
};
pub use subjects::*;
pub use substitutions::*;
pub use teaching::*;
pub use timetable::*;
pub use user::*;
