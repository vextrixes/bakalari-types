use crate::{
    endpoints::{ContentType, Endpoint},
    marking::{Atom, AtomMarks, HeaderDefinitions, HeaderOptions, SaveMarkRequest},
};
use std::marker::PhantomData;

pub const SAVE_CLASSIFICATION_HEADER_ENDPOINT: Endpoint<HeaderDefinitions, HeaderDefinitions> =
    Endpoint::Post {
        url: "/api/3/marking/atoms/{atomId}/headers",
        auth: true,
        content_type: ContentType::Json,
        _types: PhantomData,
    };

pub const GET_HEADER_OPTIONS_ENDPOINT: Endpoint<HeaderOptions, ()> = Endpoint::Get {
    url: "/api/3/marking/atom/{atomId}/headerOptions",
    auth: true,
    _types: PhantomData,
};

pub const GET_MARKING_HEADER_DEFINITIONS_ENDPOINT: Endpoint<HeaderDefinitions, ()> =
    Endpoint::Get {
        url: "/api/3/marking/atoms/{atomId}/headers",
        auth: true,
        _types: PhantomData,
    };

pub const GET_ATOM_MARKS_ENDPOINT: Endpoint<AtomMarks, ()> = Endpoint::Get {
    url: "/api/3/marking/marks/{atomId}",
    auth: true,
    _types: PhantomData,
};

pub const SAVE_MARK_ENDPOINT: Endpoint<AtomMarks, SaveMarkRequest> = Endpoint::Post {
    url: "/api/3/marking/marks/{atomId}",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GET_MARK_ATOMS_ENDPOINT: Endpoint<Vec<Atom>, ()> = Endpoint::Get {
    url: "/api/3/marking/atoms",
    auth: true,
    _types: PhantomData,
};
