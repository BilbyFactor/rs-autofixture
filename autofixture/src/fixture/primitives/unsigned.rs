#![allow(non_camel_case_types)]

use std::ops::Range;

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{conditions::BuilderCondition, FixtureBuilder, conditions::numeric::RandomRangeCondition},
};
/*
impl AutoFixture for u8 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u8Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<u8>,
}

impl<'b> FixtureBuilder<'b> for u8Builder<'b> {
    type F = u8;

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

impl<'b> u8Builder<'b> {
    pub fn with_range(&mut self, range: Range<u8>) {
        self.condition.range(range)
    }
}

impl AutoFixture for u16 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u16Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<u16>,
}

impl<'b> FixtureBuilder<'b> for u16Builder<'b> {
    type F = u16;

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

impl<'b> u16Builder<'b> {
    pub fn with_range(&mut self, range: Range<u16>) {
        self.condition.range(range)
    }
}

impl AutoFixture for u32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u32Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<u32>,
}

impl<'b> FixtureBuilder<'b> for u32Builder<'b> {
    type F = u32;

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

impl<'b> u32Builder<'b> {
    pub fn with_range(&mut self, range: Range<u32>) {
        self.condition.range(range)
    }
}

impl AutoFixture for u64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u64Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<u64>,
}

impl<'b> FixtureBuilder<'b> for u64Builder<'b> {
    type F = u64;

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

impl<'b> u64Builder<'b> {
    pub fn with_range(&mut self, range: Range<u64>) {
        self.condition.range(range)
    }
}

impl AutoFixture for u128 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u128Builder<'b> {
    fixture: &'b mut Fixture,
    condition: RandomRangeCondition<u128>,
}

impl<'b> FixtureBuilder<'b> for u128Builder<'b> {
    type F = u128;

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

impl<'b> u128Builder<'b> {
    pub fn with_range(&mut self, range: Range<u128>) {
        self.condition.range(range)
    }
}

impl AutoFixture for usize {
    fn create(f: &mut Fixture) -> Self {
        usize::from_ne_bytes(f.rng().random())
    }
}

pub struct usizeBuilder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for usizeBuilder<'b> {
    type F = usize;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
*/