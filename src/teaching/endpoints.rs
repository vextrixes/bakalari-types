use crate::{endpoints::Endpoint, teaching::Teaching};
use std::marker::PhantomData;

/// Subject ID = String
pub const TEACHING_ENDPOINT: Endpoint<Teaching, ()> = Endpoint::Get {
    url: "/api/3/subjects/themes/{subjectId}",
    auth: true,
    _types: PhantomData,
};
