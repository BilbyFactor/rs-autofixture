pub mod auto_fixture;
pub mod builder;
pub mod primitives;

use rand::rngs::ThreadRng;

use crate::fixture::{
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct Fixture {
    rng: ThreadRng,
}

impl Fixture {
    pub fn new() -> Self {
        Self { rng: ThreadRng::default() }
    }

    pub fn build<'b, B: FixtureBuilder<'b>>(&'b mut self) -> B {
        B::new(self)
    }

    pub fn create<F: AutoFixture>(&mut self) -> F {
        F::create(self)
    }

    pub fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}
