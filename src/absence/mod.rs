#[allow(clippy::module_inception)]
mod absence;
mod absence_day;
mod absence_day_response;
mod absence_overview_response;
mod absence_record;
mod absence_type_option;
mod absent_type;
mod endpoints;
mod excuse_absence_response;
mod student_timetable_absence_response;
mod subject_absence;

pub use absence::*;
pub use absence_day::*;
pub use absence_day_response::*;
pub use absence_overview_response::*;
pub use absence_record::*;
pub use absence_type_option::*;
pub use absent_type::*;
pub use endpoints::*;
pub use excuse_absence_response::*;
pub use student_timetable_absence_response::*;
pub use subject_absence::*;
