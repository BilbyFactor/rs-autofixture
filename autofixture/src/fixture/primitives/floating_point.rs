#![allow(non_camel_case_types)]

use std::ops::Range;

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{BuilderCondition, FixtureBuilder, RandomRangeCondition},
};

impl AutoFixture for f32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct f32Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<f32>,
}

impl<'b> FixtureBuilder<'b> for f32Builder<'b> {
    type F = f32;

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

impl<'b> f32Builder<'b> {
    pub fn with_range(&mut self, range: Range<f32>) {
        self.condition.range(range)
    }
}

impl AutoFixture for f64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct f64Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<f64>,
}

impl<'b> FixtureBuilder<'b> for f64Builder<'b> {
    type F = f64;

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

impl<'b> f64Builder<'b> {
    pub fn with_range(&mut self, range: Range<f64>) {
        self.condition.range(range)
    }
}
