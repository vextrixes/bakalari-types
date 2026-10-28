use std::marker::PhantomData;

use crate::{ContentType, Endpoint, Marks};

pub const CONFIRM_MARKS_ENDPOINT: Endpoint<(), Vec<String>> = Endpoint::Post {
    url: "/api/3/marks/SetClassificationConfirmation",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const COUNT_NEW_MARKS_ENDPOINT: Endpoint<i64, ()> = Endpoint::Get {
    url: "/api/3/marks/count-new",
    auth: true,
    _types: PhantomData,
};

pub const MARKS_ENDPOINT: Endpoint<Marks, ()> = Endpoint::Get {
    url: "/api/3/marks",
    auth: true,
    _types: PhantomData,
};
