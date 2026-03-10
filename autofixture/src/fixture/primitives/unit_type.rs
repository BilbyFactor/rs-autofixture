use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

/// Required for implementing enum types with () fed as a templated type.
impl AutoFixture for () {
    type Builder<'b> = UnitTypeBuilder<'b>;

    fn create(_: &mut crate::fixture::Fixture) -> Self {}

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        UnitTypeBuilder::new(f)
    }
}

pub struct UnitTypeBuilder<'b> {
    fixture: &'b mut Fixture,
}

impl<'b> FixtureBuilder<'b> for UnitTypeBuilder<'b> {
    type F = ();

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
