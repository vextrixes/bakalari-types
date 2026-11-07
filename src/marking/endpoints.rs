use crate::{ContentType, Endpoint, MarkingHeaderDefinitions, MarkingHeaderOptions};
use std::marker::PhantomData;

pub const SAVE_CLASSIFICATION_HEADER_ENDPOINT: Endpoint<
    MarkingHeaderDefinitions,
    MarkingHeaderDefinitions,
> = Endpoint::Post {
    url: "/api/3/marking/atoms/{atomId}/headers",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GET_HEADER_OPTIONS_ENDPOINT: Endpoint<MarkingHeaderOptions, ()> = Endpoint::Get {
    url: "/api/3/marking/atom/{atomId}/headerOptions",
    auth: true,
    _types: PhantomData,
};
