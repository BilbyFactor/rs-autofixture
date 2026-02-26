use rand::RngExt;

use crate::fixture::{Fixture, auto_fixture::AutoFixture};

impl AutoFixture for i8 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for i16 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for i32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for i64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for i128 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for isize {
    fn create(f: &mut Fixture) -> Self {
        isize::from_ne_bytes(f.rng().random())
    }
}
