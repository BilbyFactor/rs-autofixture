use crate::fixture::{Fixture, builder::FixtureBuilder};

pub trait AutoFixture {
    /// Creates a new randomly populated implementation of Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn create(f: &mut Fixture) -> Self;

    /// Creates the FixtureBuilder implementation for Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn build<'b>(f: &'b mut Fixture) -> impl FixtureBuilder<'b>;
}
