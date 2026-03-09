use std::ops::{
    Bound,
    Range,
    RangeBounds,
    RangeInclusive,
};

use num_traits::Bounded;
use rand::{RngExt, distr::uniform::SampleUniform};

use crate::fixture::{
    Fixture,
    FixtureExt,
    auto_fixture::AutoFixture,
    builder::conditions::BuilderCondition,
};

enum StdRangeType<T> {
    Range(Range<T>),
    RangeInclusive(RangeInclusive<T>),
}

/// Condition for allowing the builder call `with_range(1..10)` before apply
/// in order to constrain the random value output on `apply()`.
pub struct RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    range: Option<StdRangeType<T>>
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
    T: SampleUniform + AutoFixture + Bounded + Clone,
{
    /// Specifies the builder should use a range on `apply()`.
    /// 
    /// # Arguments
    /// * `range` - a range of values in `std::ops::Range<T>` format (`(start..end)`),
    ///   where `T` is the base type for the builder.
    /// 
    /// # Examples
    /// `range('a'..'Z')`, `range(..=10)`, `range(-10..)`, `range(1.123..5.678)`, etc
    /// 
    /// # Conditions
    /// In the case of `..` provided, a `std::ops::RangeFull<T>` will be used.
    /// 
    /// # Panics
    /// This function will panic if somehow a `Bound::Excluded<T>` is ever provided
    /// for a `start` range. (This is impossible with std syntax)
    pub fn range<R: RangeBounds<T>>(&mut self, range: R) {
        self.range = Some(match (
            range.start_bound().cloned(),
            range.end_bound().cloned(),
        )
        {
            (Bound::Included(s), Bound::Included(e)) =>
                StdRangeType::RangeInclusive(s..=e),
            (Bound::Included(s), Bound::Excluded(e)) => 
                StdRangeType::Range(s..e),
            (Bound::Included(s), Bound::Unbounded) =>
                StdRangeType::RangeInclusive(s..=T::max_value()),
            (Bound::Unbounded, Bound::Included(e)) =>
                StdRangeType::RangeInclusive(T::min_value()..=e),
            (Bound::Unbounded, Bound::Excluded(e)) =>
                StdRangeType::Range(T::min_value()..e),
            (Bound::Unbounded, Bound::Unbounded) =>
                StdRangeType::RangeInclusive(T::min_value()..=T::max_value()),
            _ =>
                panic!("a starting `Bound::Excluded` cannot be parsed..."),
        });
    }
}

impl<T> BuilderCondition<T> for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture + PartialOrd + Clone,
{
    fn apply(&self, f: &mut Fixture) -> Option<T> {
        if let Some(range) = &self.range {
            Some(match range {
                StdRangeType::Range(r) => f.rng().random_range(r.clone()),
                StdRangeType::RangeInclusive(ri) => f.rng().random_range(ri.clone()),
            })
        }
        else {
            None
        }
    }

    fn clear(&mut self) {
        self.range = None;
    }
}
