use std::marker::PhantomData;

use crate::{Endpoint, SubstitutionCycles, Substitutions};

pub const SUBSTITUTIONS_ENDPOINT: Endpoint<Substitutions, ()> = Endpoint::Get {
    url: "/api/3/substitutions",
    auth: true,
    _types: PhantomData,
};

pub const SUBSTITUTION_CYCLES_ENDPOINT: Endpoint<SubstitutionCycles, ()> = Endpoint::Get {
    url: "/api/3/substitutions/cycles",
    auth: true,
    _types: PhantomData,
};
