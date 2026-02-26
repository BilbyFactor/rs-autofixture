use crate::fixture::{AutoFixture, Fixture};

pub trait FixtureBuilder<'f> {
    type F: AutoFixture;
    fn new(f: &'f mut Fixture) -> Self;
    fn create(&mut self) -> Self::F;
}
