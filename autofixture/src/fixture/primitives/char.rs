#![allow(non_camel_case_types)]

use crate::fixture::{
    auto_fixture::impl_autofixture_random,
    builder::create_general_builder,
};

create_general_builder!(char);
impl_autofixture_random!(char => charBuilder);
