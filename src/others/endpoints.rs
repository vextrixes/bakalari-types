use std::marker::PhantomData;

use crate::{ApiInfo, Endpoint};

pub const API_V3_INFO_ENDPOINT: Endpoint<(), ApiInfo> = Endpoint::Get {
    url: "/api/3",
    auth: false,
    _types: PhantomData,
};

pub const APIS_INFO_ENDPOINT: Endpoint<(), Vec<ApiInfo>> = Endpoint::Get {
    url: "/api",
    auth: false,
    _types: PhantomData,
};
