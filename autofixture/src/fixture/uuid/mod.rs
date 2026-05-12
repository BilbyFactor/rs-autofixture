//! AutoFixture implementation for `uuid::Uuid`.

use uuid::Uuid;

use crate::fixture::{
    AutoFixture, Fixture,
    builder::FixtureBuilder,
};

pub struct UuidBuilder<'b> {
    fixture: &'b mut Fixture,
}

impl<'b> FixtureBuilder<'b> for UuidBuilder<'b> {
    type F = Uuid;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Uuid::create(self.fixture)
    }
}

impl AutoFixture for Uuid {
    type Builder<'b> = UuidBuilder<'b>;

    fn create(_f: &mut Fixture) -> Self {
        Uuid::new_v4()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        UuidBuilder::new(f)
    }
}
