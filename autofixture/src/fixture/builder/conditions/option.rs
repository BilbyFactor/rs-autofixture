use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::conditions::BuilderCondition,
};

// Condition for allowing the builder calls `with({some})` and `without()`
// before `apply()` in order to make the output deterministic.
pub struct OptionCondition<T>
where
    T: AutoFixture,
{
    with: Option<T>,
}

impl<T> Default for OptionCondition<T>
where
    T: AutoFixture,
{
    fn default() -> Self {
        Self { with: None }
    }
}

impl<T> OptionCondition<T>
where
    T: AutoFixture,
{
    /// TODO
    pub fn with(&mut self, some: T) {
        self.with = Some(some);
    }

    /// TODO
    pub fn without(&mut self) {
        self.with = None;
    }
}

impl<T> BuilderCondition<T> for OptionCondition<T>
where
    T: AutoFixture + Clone,
{
    fn apply(&self, _: &mut Fixture) -> Option<T> {
        self.with.clone()
    }

    fn clear(&mut self) {
        self.with = None;
    }
}
