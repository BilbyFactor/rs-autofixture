use crate::fixture::{Fixture, builder::FixtureBuilder};

pub trait AutoFixture {
    type Builder<'b>: FixtureBuilder<'b, F = Self>;

    /// Creates a new randomly populated implementation of Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn create(f: &mut Fixture) -> Self;

    /// Creates the FixtureBuilder implementation for Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b>;
}
