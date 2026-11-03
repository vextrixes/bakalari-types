use crate::{ClassBookLessonPast, ClassBookSaveAbsenceRequest, ContentType, Endpoint};
use std::marker::PhantomData;

pub const EDIT_CLASSBOOK_ENDPOINT: Endpoint<(), ClassBookSaveAbsenceRequest> = Endpoint::Post {
    url: "/api/3/classbook/{recordId}",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const PREVIOUS_LESSON_INFO_CLASSBOOK_ENDPOINT: Endpoint<ClassBookLessonPast, ()> = Endpoint::Get {
    url: "/api/3/classbook/{recordId}",
    auth: true,
    _types: PhantomData,
};
