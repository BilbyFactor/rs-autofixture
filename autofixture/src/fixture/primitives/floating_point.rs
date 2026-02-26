#![allow(non_camel_case_types)]

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

impl AutoFixture for f32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct f32Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for f32Builder<'b> {
    type F = f32;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl AutoFixture for f64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct f64Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for f64Builder<'b> {
    type F = f64;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
