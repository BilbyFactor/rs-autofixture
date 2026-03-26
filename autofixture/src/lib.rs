//! # RS-AutoFixture
//!
//! This crate does it's best to create a Rust equivelant of the popular .NET
//! [Autofixture]: https://github.com/AutoFixture/AutoFixture/
//! library for quickly automatically generating data fixtures.
//!
//! [LICENSE-MIT]: https://opensource.org/licenses/MIT
//!
#![cfg_attr(feature = "nightly-float", feature(f16, f128))]

pub mod fixture;

// Re-export #[derive(AutoFixture)].
#[cfg(feature = "derive")]
extern crate autofixture_derive;

/// Derive macro available if rs-autofixture is built with `features = ["derive"]`.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use autofixture_derive::{AutoFixture};
