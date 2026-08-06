//! AutoFixture implementations for Rust primitive types.
//!
//! Numeric types (`u8`..`u128`, `i8`..`i128`, `f32`, `f64`) support the
//! `with_range()` and `with_options()` builder conditions.
//!
//! For `f16` and `f128` support, you can use the `nightly-float` feature with
//! nightly Rust builds.
//!
//! `bool` and `char` support `with_options()` only.

pub mod bool;
pub mod char;
pub mod floating_point;
pub mod option;
pub mod result;
pub mod signed;
pub mod unit_type;
pub mod unsigned;
