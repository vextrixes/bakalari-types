use std::marker::PhantomData;

use crate::{Endpoint, UserInfo};

pub const USER_INFO_ENDPOINT: Endpoint<UserInfo, ()> = Endpoint::Get {
    url: "/api/3/user",
    auth: true,
    _types: PhantomData,
};

pub const STUDENT_AT_SCHOOL_ENDPOINT: Endpoint<bool, ()> = Endpoint::Get {
    url: "/api/3/user/student-at-school",
    auth: true,
    _types: PhantomData,
};
