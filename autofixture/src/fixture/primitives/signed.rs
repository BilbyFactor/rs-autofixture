#![allow(non_camel_case_types)]

use std::ops::Range;

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{BuilderCondition, FixtureBuilder, RandomRangeCondition},
};

impl AutoFixture for i8 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct i8Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<i8>,
}

impl<'b> FixtureBuilder<'b> for i8Builder<'b> {
    type F = i8;

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

impl<'b> i8Builder<'b> {
    pub fn with_range(&mut self, range: Range<i8>) {
        self.condition.range(range)
    }
}

impl AutoFixture for i16 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct i16Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<i16>,
}

impl<'b> FixtureBuilder<'b> for i16Builder<'b> {
    type F = i16;

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

impl<'b> i16Builder<'b> {
    pub fn with_range(&mut self, range: Range<i16>) {
        self.condition.range(range)
    }
}

impl AutoFixture for i32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct i32Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<i32>,
}

impl<'b> FixtureBuilder<'b> for i32Builder<'b> {
    type F = i32;

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

impl<'b> i32Builder<'b> {
    pub fn with_range(&mut self, range: Range<i32>) {
        self.condition.range(range)
    }
}

impl AutoFixture for i64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct i64Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<i64>,
}

impl<'b> FixtureBuilder<'b> for i64Builder<'b> {
    type F = i64;

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

impl<'b> i64Builder<'b> {
    pub fn with_range(&mut self, range: Range<i64>) {
        self.condition.range(range)
    }
}

impl AutoFixture for i128 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct i128Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<i128>,
}

impl<'b> FixtureBuilder<'b> for i128Builder<'b> {
    type F = i128;

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

impl<'b> i128Builder<'b> {
    pub fn with_range(&mut self, range: Range<i128>) {
        self.condition.range(range)
    }
}

impl AutoFixture for isize {
    fn create(f: &mut Fixture) -> Self {
        isize::from_ne_bytes(f.rng().random())
    }
}

pub struct isizeBuilder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for isizeBuilder<'b> {
    type F = isize;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
