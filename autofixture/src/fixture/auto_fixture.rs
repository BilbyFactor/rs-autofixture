use crate::fixture::Fixture;

pub trait AutoFixture {
    fn create(f: &mut Fixture) -> Self;
}
