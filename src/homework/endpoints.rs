use std::marker::PhantomData;

use crate::{Endpoint, HomeworkList};

pub const HOMEWORKS_COUNT_ENDPOINT: Endpoint<i64, ()> = Endpoint::Get {
    url: "/api/3/homeworks/count-actual",
    auth: true,
    _types: PhantomData,
};

/// ID = homework ID (String)
/// finished = homework state (bool)
pub const HOMEWORK_DONE_ENDPOINT: Endpoint<bool, ()> = Endpoint::Get {
    url: "api/3/homeworks/{id}/student-done/{finished}",
    auth: true,
    _types: PhantomData,
};

/// Query 'from' and 'to' (Datetime)
pub const HOMEWORKS_ENDPOINT: Endpoint<HomeworkList, ()> = Endpoint::Get {
    url: "api/3/homeworks",
    auth: true,
    _types: PhantomData,
};
