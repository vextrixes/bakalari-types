use crate::{Endpoint, WebModulesResponse};
use std::marker::PhantomData;

pub const WEBMODULES_ENDPOINT: Endpoint<WebModulesResponse, ()> = Endpoint::Get {
    url: "/api/3/webmodule",
    auth: true,
    _types: PhantomData,
};
