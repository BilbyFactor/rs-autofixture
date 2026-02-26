#![allow(non_camel_case_types)]

use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

impl AutoFixture for u8 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u8Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for u8Builder<'b> {
    type F = u8;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl AutoFixture for u16 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u16Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for u16Builder<'b> {
    type F = u16;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl AutoFixture for u32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u32Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for u32Builder<'b> {
    type F = u32;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl AutoFixture for u64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u64Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for u64Builder<'b> {
    type F = u64;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}

impl AutoFixture for u128 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

pub struct u128Builder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for u128Builder<'b> {
    type F = u128;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
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
