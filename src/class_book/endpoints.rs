use crate::{ClassBookSaveAbsenceRequest, ContentType, Endpoint};
use std::marker::PhantomData;

pub const EDIT_CLASSBOOK_ENDPOINT: Endpoint<(), ClassBookSaveAbsenceRequest> = Endpoint::Post {
    url: "/api/3/classbook/{recordId}",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};
