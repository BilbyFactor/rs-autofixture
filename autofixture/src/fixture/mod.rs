pub mod auto_fixture;
pub mod builder;
pub mod collections;
pub mod primitives;

use rand::rngs::ThreadRng;

use crate::fixture::{
    auto_fixture::AutoFixture,
};

/// The main `Fixture` structure that maintains any state and the RNG engine.
pub struct Fixture {
    rng: ThreadRng,
}

impl Fixture {
    /// Creates a new `Fixture` with a default RNG engine.
    pub fn new() -> Self {
        Self { rng: ThreadRng::default() }
    }

    /// Creates a new `FixtureBuilder` for type `F`, where `F` is a
    /// type that implements `AutoFixture`. This includes all Rust primitive
    /// types, and can be derived on a struct or enum with `#[derive(AutoFixture)]`.
    pub fn build<'b, F: AutoFixture>(&'b mut self) -> F::Builder<'b> {
        F::build(self)
    }

    /// Creates a new randomly populated implementation of the given type `F`
    /// where `F` implements `AutoFixture`. This includes all Rust primitive
    /// types, and can be derived on a struct or enum with `#[derive(AutoFixture)]`
    pub fn create<F: AutoFixture>(&mut self) -> F {
        F::create(self)
    }

    /// Return the `Fixture` implementations `ThreadRng` engine.
    pub fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}
