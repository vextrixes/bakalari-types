use crate::{
    ClassBookAbsenceResponse, ClassBookLessonPast, ClassBookLessonTag, ClassBookSaveRecordRequest,
    ContentType, Endpoint,
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

pub const LESSON_TAGS_CLASSBOOK_ENDPOINT: Endpoint<ClassBookLessonTag, ()> = Endpoint::Get {
    url: "/api/3/classbook/lessonTags",
    auth: true,
    _types: PhantomData,
};

pub const CLASSBOOK_ABSENCE_ENDPOINT: Endpoint<ClassBookAbsenceResponse, ()> = Endpoint::Get {
    url: "/api/3/lesson/{lessonId}/absence",
    auth: true,
    _types: PhantomData,
};
