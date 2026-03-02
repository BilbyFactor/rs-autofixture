#![allow(non_camel_case_types)]

use crate::fixture::primitives::{
    create_general_builder,
    create_numeric_builder,
    impl_autofixture_random,
    impl_autofixture_random_dyn,
};

impl_autofixture_random!(u8, u16, u32, u64, u128);
create_numeric_builder!(u8, u16, u32, u64, u128);

impl_autofixture_random_dyn!(usize);
create_general_builder!(usize);
