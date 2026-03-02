#![allow(non_camel_case_types)]

use crate::fixture::primitives::{
    create_general_builder,
    create_numeric_builder,
    impl_autofixture_random,
    impl_autofixture_random_dyn,
};

impl_autofixture_random!(i8, i16, i32, i64, i128);
create_numeric_builder!(i8, i16, i32, i64, i128);

impl_autofixture_random_dyn!(isize);
create_general_builder!(isize);
