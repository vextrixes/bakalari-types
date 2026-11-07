use crate::{endpoints::Endpoint, timetable::Timetable};
use std::marker::PhantomData;

pub const PERMAMENT_TIMETABLE_ENDPOINT: Endpoint<Timetable, ()> = Endpoint::Get {
    url: "/api/3/timetable/permanent",
    auth: true,
    _types: PhantomData,
};

pub const PERMAMENT_TIMETABLE_CLASS_ENDPOINT: Endpoint<Timetable, ()> = Endpoint::Get {
    url: "/api/3/timetable/permanent/class/{classId}",
    auth: true,
    _types: PhantomData,
};

pub const WEEK_TIMETABLE_ENDPOINT: Endpoint<Timetable, ()> = Endpoint::Get {
    url: "/api/3/timetable/actual",
    auth: true,
    _types: PhantomData,
};

pub const WEEK_TIMETABLE_CLASS_ENDPOINT: Endpoint<Timetable, ()> = Endpoint::Get {
    url: "/api/3/timetable/actual/class/{classId}",
    auth: true,
    _types: PhantomData,
};
