mod absence;
mod api_info;
mod attachment;
mod error;
mod events;
mod gdpr;
mod homework;
mod lesson;
mod marks;
mod measures;
mod payments;
mod stdtype;
mod subjects;
mod substitutions;
mod themes;
mod timetable;
mod user;
mod webmodules;
mod what_if;

pub use absence::{Absence, AbsenceListItem, SubjectAbsence};
pub use api_info::ApiInfo;
pub use attachment::Attachment;
pub use error::Error;
pub use events::{Event, Events, Time};
pub use gdpr::{GDPRcommissioner, GDPRcommissioners};
pub use homework::{Homework, HomeworkListItem};
pub use lesson::{
    AbsencesGroup, AbsentTypeOption, LessonAbsence, LessonHour, LessonLastTheme,
    LessonStudentAbsence, LessonStudentInfo,
};
pub use marks::{Mark, Marks, SubjectMarks, MarksFinal, CertificateTerm, FinalMark};
pub use measures::{Measures, PedagogicalMeasure};
pub use payments::{AddBalance, ClassfundPayments, ClassfundSummary, Payment, PaymentMonthlyData};
pub use stdtype::{StdClassGroup, StdType};
pub use subjects::{Subject, Subjects};
pub use substitutions::{SubstitutionChange, Substitutions};
pub use themes::{Theme, Themes};
pub use timetable::{Timetable, TimetableAtom, TimetableChange, TimetableDay, TimetableHour};
pub use user::{Module, User};
pub use webmodules::{Webmodule, Webmodules};
pub use what_if::{WhatIfResult, WhatIfResultMark, WhatIfStartingEnvironmentMark};
