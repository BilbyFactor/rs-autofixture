#![allow(non_camel_case_types)]

use crate::fixture::primitives::{
    create_numeric_builder,
    impl_autofixture_random,
};

impl_autofixture_random!(f32, f64);
create_numeric_builder!(f32, f64);

#[cfg(feature = "nightly-float")]
pub mod nightly_float {
    impl_autofixture_random!(f16, f128);
    create_numeric_builder!(f16, f128);
}
