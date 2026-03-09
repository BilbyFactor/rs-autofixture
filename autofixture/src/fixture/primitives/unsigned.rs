#![allow(non_camel_case_types)]

use crate::fixture::{
    auto_fixture::{impl_autofixture_random, impl_autofixture_random_dyn},
    builder::{create_general_builder, create_numeric_builder},
};

create_numeric_builder!(u8, u16, u32, u64, u128);

impl_autofixture_random!(
    u8 => u8Builder,
    u16 => u16Builder,
    u32 => u32Builder,
    u64 => u64Builder,
    u128 => u128Builder);

create_general_builder!(usize);
impl_autofixture_random_dyn!(usize => usizeBuilder);
