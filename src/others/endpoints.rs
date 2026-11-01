use crate::{ApiInfo, Endpoint};
use std::marker::PhantomData;

pub const API_V3_INFO_ENDPOINT: Endpoint<ApiInfo, ()> = Endpoint::Get {
    url: "/api/3",
    auth: false,
    _types: PhantomData,
};

pub const APIS_INFO_ENDPOINT: Endpoint<Vec<ApiInfo>, ()> = Endpoint::Get {
    url: "/api",
    auth: false,
    _types: PhantomData,
};
