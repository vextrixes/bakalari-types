use crate::{ContentType, Endpoint, MarkingHeaderDefinitions};
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
