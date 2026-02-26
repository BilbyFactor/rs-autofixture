use rand::RngExt;

use crate::fixture::{Fixture, auto_fixture::AutoFixture};

impl AutoFixture for f32 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}

impl AutoFixture for f64 {
    fn create(f: &mut Fixture) -> Self {
        f.rng().random()
    }
}
