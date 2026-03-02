pub mod conditions;

use crate::fixture::{AutoFixture, Fixture};

/// A trait for building `AutoFixture` types.
pub trait FixtureBuilder<'f> {
    /// The type that `create()` will return a built implementation of.
    type F: AutoFixture;

    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn new(f: &'f mut Fixture) -> Self;

    /// Returns a built implementation of type `F`, following any builder rules
    /// specified beforehand.
    fn create(&mut self) -> Self::F;
}
