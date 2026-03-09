#![allow(non_camel_case_types)]

use crate::fixture::{
    auto_fixture::impl_autofixture_random,
    builder::create_numeric_builder,
};

create_numeric_builder!(f32, f64);
impl_autofixture_random!(f32 => f32Builder, f64 => f64Builder);

#[cfg(feature = "nightly-float")]
pub mod nightly_float {
    create_numeric_builder!(f16, f128);
    impl_autofixture_random!(f16 => f16Builder, f128 => f128Builder);
}
