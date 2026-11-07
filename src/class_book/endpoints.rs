use crate::{
    class_book::{
        ClassBookAbsenceRecordResult, ClassBookAbsenceResponse, ClassBookEntries,
        ClassBookLessonPast, ClassBookLessonTag, ClassBookSaveAbsenceRequest,
        ClassBookSaveRecordRequest, ClassBookStudentsTimetableResponse,
    },
    endpoints::{ContentType, Endpoint},
};
use std::marker::PhantomData;

pub const EDIT_CLASSBOOK_ENDPOINT: Endpoint<(), ClassBookSaveRecordRequest> = Endpoint::Post {
    url: "/api/3/classbook/{recordId}",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const PREVIOUS_LESSON_INFO_CLASSBOOK_ENDPOINT: Endpoint<ClassBookLessonPast, ()> =
    Endpoint::Get {
        url: "/api/3/lesson/{lessonId}/past",
        auth: true,
        _types: PhantomData,
    };

pub const LESSON_TAGS_CLASSBOOK_ENDPOINT: Endpoint<Vec<ClassBookLessonTag>, ()> = Endpoint::Get {
    url: "/api/3/classbook/lessonTags",
    auth: true,
    _types: PhantomData,
};

pub const CLASSBOOK_ABSENCE_ENDPOINT: Endpoint<ClassBookAbsenceResponse, ()> = Endpoint::Get {
    url: "/api/3/lesson/{lessonId}/absence",
    auth: true,
    _types: PhantomData,
};

pub const GET_CLASSBOOK_ENDPOINT: Endpoint<ClassBookEntries, ()> = Endpoint::Get {
    url: "/api/3/classbook",
    auth: true,
    _types: PhantomData,
};

pub const CLASSBOOK_STUDENT_TIMETABLE_ENDPOINT: Endpoint<ClassBookStudentsTimetableResponse, ()> =
    Endpoint::Get {
        url: "/api/3/lesson/{lessonId}/studentsTimetable",
        auth: true,
        _types: PhantomData,
    };

pub const EDIT_CLASSBOOK_ABSENCE_ENDPOINT: Endpoint<
    ClassBookAbsenceRecordResult,
    Vec<ClassBookSaveAbsenceRequest>,
> = Endpoint::Put {
    url: "/api/3/lesson/{lessonId}/absence",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};
