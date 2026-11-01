use crate::{Endpoint, Subjects};
use std::marker::PhantomData;

pub const SUBJECTS_ENDPOINT: Endpoint<Subjects, ()> = Endpoint::Get {
    url: "/api/3/subjects",
    auth: true,
    _types: PhantomData,
};
