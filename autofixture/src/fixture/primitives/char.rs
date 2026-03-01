#![allow(non_camel_case_types)]

use std::ops::Range;

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{BuilderCondition, FixtureBuilder, RandomRangeCondition},
};

impl AutoFixture for char {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

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
