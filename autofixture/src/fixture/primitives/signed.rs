#![allow(non_camel_case_types)]

use crate::fixture::{
    auto_fixture::{impl_autofixture_random, impl_autofixture_random_dyn},
    builder::{create_general_builder, create_numeric_builder},
};

create_numeric_builder!(i8, i16, i32, i64, i128);

impl_autofixture_random!(
    i8 => i8Builder,
    i16 => i16Builder,
    i32 => i32Builder,
    i64 => i64Builder,
    i128 => i128Builder);

create_general_builder!(isize);
impl_autofixture_random_dyn!(isize => isizeBuilder);
