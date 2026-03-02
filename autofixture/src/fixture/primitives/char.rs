#![allow(non_camel_case_types)]

use crate::fixture::primitives::{
    create_general_builder,
    impl_autofixture_random,
};

impl_autofixture_random!(char);
create_general_builder!(char);
