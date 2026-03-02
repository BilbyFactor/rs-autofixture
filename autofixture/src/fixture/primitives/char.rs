#![allow(non_camel_case_types)]

use crate::fixture::primitives::impl_autofixture_random;

impl_autofixture_random!(char);

/*
pub struct charBuilder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<char>,
}

impl<'b> FixtureBuilder<'b> for charBuilder<'b> {
    type F = char;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            condition: RandomRangeCondition::default(),
        }
    }

    fn create(&mut self) -> Self::F {
        self.condition.apply(self.fixture)
    }
}

impl<'b> charBuilder<'b> {
    pub fn with_range(&mut self, range: Range<char>) {
        self.condition.range(range)
    }
}
*/