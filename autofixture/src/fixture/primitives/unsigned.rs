use rand::RngExt;

use crate::fixture::{Fixture, auto_fixture::AutoFixture};

impl AutoFixture for u8 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for u16 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for u32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for u64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for u128 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for usize {
    fn create(f: &mut Fixture) -> Self {
        usize::from_ne_bytes(f.rng().random())
    }
}
