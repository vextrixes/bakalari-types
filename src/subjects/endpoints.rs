use std::marker::PhantomData;

use crate::{Endpoint, Subjects};

pub const SUBJECTS_ENDPOINT: Endpoint<(), Subjects> = Endpoint::Get {
    url: "/api/3/subjects",
    auth: true,
    _types: PhantomData,
};
