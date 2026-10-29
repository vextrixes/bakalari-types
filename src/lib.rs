mod absence;
mod attachment;
mod others;
//campaign
mod auth;
mod campaign;
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
pub use attachment::AttachmentInfo;
pub use auth::*;
pub use campaign::Campaign;
pub use endpoints::*;
pub use error::{Error, ModelState, StringArray};
pub use events::*;
pub use gdpr::{
    GdprCommissioner, GdprCommissioners, GdprConsent, GdprConsentRights, GdprConsentState,
    GdprValidity,
};
pub use homework::*;
pub use marks::*;
pub use others::*;
pub use payments::*;
pub use semester::{
    CertificateTerm, CertificateTerms, FinalMark, PedagogicalMeasure, PedagogicalMeasures,
};
pub use stdtype::{
    Room, RoomSet, SchoolClass, SchoolClassSet, StdClassGroup, StdType, StdTypeNullable, Student,
    Teacher, TeacherSet, TimetableStudent,
};
pub use subjects::*;
pub use substitutions::*;
pub use teaching::*;
pub use timetable::*;
pub use user::*;
