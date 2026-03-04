#![allow(non_camel_case_types)]

use crate::fixture::primitives::{
    create_numeric_builder,
    impl_autofixture_random,
};

create_numeric_builder!(f32, f64);
impl_autofixture_random!(f32 => f32Builder, f64 => f64Builder);

#[cfg(feature = "nightly-float")]
pub mod nightly_float {
    impl_autofixture_random!(f16, f128);
    create_numeric_builder!(f16, f128);
}
