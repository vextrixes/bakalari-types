use std::marker::PhantomData;

use crate::{ContentType, Endpoint, GdprCommissioners, GdprConsents};

pub const SEND_GDPR_OBJECTION_ENDPOINT: Endpoint<(), String> = Endpoint::Post {
    url: "/api/3/gdpr/commissioner/send-objection",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GDPR_CONSENT_REVOKE_ENDPOINT: Endpoint<(), ()> = Endpoint::Put {
    url: "/api/3/gdpr/consent/{id}/revoke-request",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GDPR_CONSENT_AGREE_ENDPOINT: Endpoint<(), ()> = Endpoint::Put {
    url: "/api/3/gdpr/consent/{id}/agree",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GDPR_CONSENT_DISAGREE_ENDPOINT: Endpoint<(), ()> = Endpoint::Put {
    url: "/api/3/gdpr/consent/{id}/disagree",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GDPR_CONSENTS_COUNT_ENDPOINT: Endpoint<i64, ()> = Endpoint::Get {
    url: "/api/3/gdpr/consents/person/new",
    auth: true,
    _types: PhantomData,
};

pub const SEND_GDPR_REPORT_ENDPOINT: Endpoint<(), ()> = Endpoint::Post {
    // unknown request type
    url: "/api/3/gdpr/commissioner/send-report",
    auth: true,
    content_type: ContentType::Json,
    _types: PhantomData,
};

pub const GET_GDPR_COMMISSIONERS_ENDPOINT: Endpoint<GdprCommissioners, ()> = Endpoint::Get {
    url: "/api/3/gdpr/commissioners",
    auth: true,
    _types: PhantomData,
};

pub const GET_GDPR_CHILD_CONSENTS_ENDPOINT: Endpoint<GdprConsents, ()> = Endpoint::Get {
    url: "/api/3/gdpr/consents/person/child",
    auth: true,
    _types: PhantomData,
};

pub const GET_GDPR_CONSENTS_ENDPOINT: Endpoint<GdprConsents, ()> = Endpoint::Get {
    url: "/api/3/gdpr/consents/person",
    auth: true,
    _types: PhantomData,
};
