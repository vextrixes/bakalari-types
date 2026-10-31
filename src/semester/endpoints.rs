use std::marker::PhantomData;

use crate::{CertificateTerms, Endpoint, PedagogicalMeasures};

pub const GET_CERTIFICATE_TERMS_ENDPOINT: Endpoint<CertificateTerms, ()> = Endpoint::Get {
    url: "/api/3/marks/final",
    auth: true,
    _types: PhantomData,
};

pub const GET_PEDAGOGICAL_MEASURES_ENDPOINT: Endpoint<PedagogicalMeasures, ()> = Endpoint::Get {
    url: "/api/3/marks/measures",
    auth: true,
    _types: PhantomData,
};
