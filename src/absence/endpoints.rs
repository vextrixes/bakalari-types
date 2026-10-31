use std::marker::PhantomData;

use crate::{
    ContentType, Endpoint, ExcuseAbsenceResponse, StudentAbsenceOverviewResponse,
    StudentTimetableAbsenceResponse,
};

pub const CAN_STUDENT_EXCUSE_ABSENCE_ENDPOINT: Endpoint<bool, ()> = Endpoint::Get {
    url: "/api/3/absence/canexcuse/{studentId}",
    auth: true,
    _types: PhantomData,
};

pub const GET_STUDENT_ABSENCE_ENDPOINT: Endpoint<StudentAbsenceOverviewResponse, ()> =
    Endpoint::Get {
        url: "/api/3/absence/student",
        auth: true,
        _types: PhantomData,
    };

pub const GET_STUDENT_TIMETABLE_WITH_ABSENCES_ENDPOINT: Endpoint<
    StudentTimetableAbsenceResponse,
    (),
> = Endpoint::Get {
    url: "/api/3/timetable/student/{studentId}",
    auth: true,
    _types: PhantomData,
};

pub const EXCUSE_KOMENS_ABSENCE_ENDPOINT: Endpoint<ExcuseAbsenceResponse, ()> = Endpoint::Put {
    //TODO unknown behaviour (and request type) // probably bsenceRecord or AbsenceRecordWithSubject
    url: "/api/3/absence",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};
