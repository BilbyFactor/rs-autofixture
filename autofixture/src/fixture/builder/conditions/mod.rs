use crate::fixture::{Fixture, auto_fixture::AutoFixture};

pub mod general;
pub mod numeric;

/// A trait for allowing the application of custom builder conditions
/// by a `FixtureBuilder`.
/// 
/// # Type constraints
/// * `T` - must implement `AutoFixture`.
///   This already includes all Rust primitive types,
///   and can be derived on a struct or enum with `#[derive(AutoFixture)]`.
pub trait BuilderCondition<T>: Default
where
    T: AutoFixture,
{
    /// Returns either Some built instance of `T` following any builder rules specified
    /// before `apply()` was called,
    /// or None if the condiotions are not relevant; for instance none were given,
    /// or a different condition call overwrote them.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn apply(&self, f: &mut Fixture) -> Option<T>;

    /// Clears all specifies conditions.
    fn clear(&mut self);
}
