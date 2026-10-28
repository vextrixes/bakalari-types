use std::marker::PhantomData;

pub enum Endpoint<Response, Request> {
    Get {
        url: &'static str,
        auth: bool,
        _types: PhantomData<fn() -> (Response, Request)>,
    },
    Post {
        url: &'static str,
        auth: bool,
        content_type: ContentType,
        _types: PhantomData<fn() -> (Response, Request)>,
    },
}

pub enum ContentType {
    Urlencoded,
    Json,
}
