use rand::seq::IndexedRandom;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::conditions::BuilderCondition,
};

/// Condition for allowing the builder call `with_options(vec![1, 2, 3])` before apply
/// in order to constrain the output of `apply()` to be selected from one of the
/// given options.
pub struct OptionsCondition<T>
where
    T: AutoFixture,
{
    options: Vec<T>,
}

impl<T> Default for OptionsCondition<T>
where
    T: AutoFixture,
{
    fn default() -> Self {
        Self { options: vec![] }
    }
}

impl<T> OptionsCondition<T>
where
    T: AutoFixture + Clone,
{
    /// Specifies the builder pick from a set of given options on `apply()`.
    /// 
    /// # Arguments
    /// * `options` - a mutable `Vec` of `T` values to add as options,
    ///   where `T` is the base type for the builder.
    /// 
    /// # Examples
    /// `options(vec![1, 3, 5, 9])`
    pub fn options(&mut self, options: &mut Vec<T>) {
        self.options.append(options);
    }
}

impl<T> BuilderCondition<T> for OptionsCondition<T>
where
    T: AutoFixture + Clone,
{
    fn apply(&self, f: &mut Fixture) -> Option<T> {
        self.options
            .choose(f.rng())
            .map(|i| i.clone())
    }

    fn clear(&mut self) {
        self.options.clear();
    }
}
