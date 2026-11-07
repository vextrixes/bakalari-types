use crate::{endpoints::Endpoint, events::Events};
use std::marker::PhantomData;

pub const EVENTS_ENDPOINT: Endpoint<Events, ()> = Endpoint::Get {
    url: "/api/3/events",
    auth: true,
    _types: PhantomData,
};

pub const MY_EVENTS_ENDPOINT: Endpoint<Events, ()> = Endpoint::Get {
    url: "/api/3/events/my",
    auth: true,
    _types: PhantomData,
};
