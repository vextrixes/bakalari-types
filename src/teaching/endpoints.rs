use std::marker::PhantomData;

use crate::{Endpoint, Teaching};

/// Subject ID = String
pub const TEACHING_ENDPOINT: Endpoint<(), Teaching> = Endpoint::Get {
    url: "/api/3/subjects/themes/{subjectId}",
    auth: true,
    _types: PhantomData,
};
