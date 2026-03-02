use std::ops::Range;

use rand::{RngExt, distr::uniform::SampleUniform};

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::conditions::BuilderCondition,
};

/// Condition for allowing the builder call `with_range(1..10)` before apply
/// in order to constrain the random value output on `apply()`.
pub struct RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    range: Option<Range<T>>,
}

impl<T> Default for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    fn default() -> Self {
        Self { range: None }
    }
}

impl<T> RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    /// Specifies the builder should use a range on `apply()`.
    /// 
    /// # Arguments
    /// * `range` - a range of values in `std::Range<T>` format (`(start..end)`),
    ///   where `T` is the base type for the builder.
    /// 
    /// # Examples
    /// `range(1..10)`, `range(..10)`, `range(-10..)`, `range(1.123..5.678)`, etc
    pub fn range(&mut self, range: Range<T>) {
        self.range = Some(range);
    }
}

impl<T> BuilderCondition<T> for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture + PartialOrd + Clone,
{
    fn apply(&self, f: &mut Fixture) -> Option<T> {
        if let Some(range) = &self.range {
            Some(f.rng().random_range(range.clone()))
        }
        else {
            None
        }
    }

    fn clear(&mut self) {
        self.range = None;
    }
}
