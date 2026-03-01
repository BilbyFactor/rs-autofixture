#![allow(non_camel_case_types)]

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

impl AutoFixture for bool {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct boolBuilder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for boolBuilder<'b> {
    type F = isize;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
