#![allow(non_camel_case_types)]

use crate::fixture::{
    Fixture,
    auto_fixture::{AutoFixture, impl_autofixture_random},
    builder::FixtureBuilder,
};


// Requires a very basic builder...
pub struct boolBuilder<'b> {
    fixture: &'b mut Fixture,
}

impl<'b> FixtureBuilder<'b> for boolBuilder<'b> {
    type F = bool;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl_autofixture_random!(bool => boolBuilder);
