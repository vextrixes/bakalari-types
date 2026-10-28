use std::marker::PhantomData;

use crate::{ContentType, Endpoint, LoginSuccess, OpenIdDiscovery, auth::login::LoginRequest};

pub const LOGIN_ENDPOINT: Endpoint<LoginSuccess, LoginRequest> = Endpoint::Post {
    url: "/api/login",
    auth: false,
    content_type: ContentType::Urlencoded,
    _types: PhantomData,
};

pub const OPEN_ID_DISCOVERY_ENDPOINT: Endpoint<OpenIdDiscovery, ()> = Endpoint::Get {
    url: "/.well-known/openid-configuration",
    auth: false,
    _types: PhantomData,
};

pub const LOGIN_TOKEN_ENDPOINT: Endpoint<String, ()> = Endpoint::Get {
    url: "/api/3/logintoken",
    auth: true,
    _types: PhantomData,
};
