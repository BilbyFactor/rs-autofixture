use std::ops::Range;

use rand::{RngExt, distr::uniform::SampleUniform};

use crate::fixture::{AutoFixture, Fixture};

pub trait FixtureBuilder<'f> {
    type F: AutoFixture;
    fn new(f: &'f mut Fixture) -> Self;
    fn create(&mut self) -> Self::F;
}

pub trait BuilderCondition<T>: Default
where
    T: AutoFixture,
{
    fn apply(&self, f: &mut Fixture) -> T;
}

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

impl<T: AutoFixture> RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    pub fn range(&mut self, range: Range<T>) {
        self.range = Some(range);
    }
}

impl<T> BuilderCondition<T> for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture + PartialOrd + Clone,
{
    fn apply(&self, f: &mut Fixture) -> T {
        if let Some(range) = &self.range {
            f.rng().random_range(range.clone())
        }
        else {
            T::create(f)
        }
    }
}
