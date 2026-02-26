pub mod auto_fixture;
pub mod primitives;

use rand::rngs::ThreadRng;

use crate::fixture::auto_fixture::AutoFixture;

pub struct Fixture {
    rng: ThreadRng,
}

impl Fixture {
    pub fn new() -> Self {
        Self { rng: ThreadRng::default() }
    }

    pub fn create<T: AutoFixture>(&mut self) -> T {
        T::create(self)
    }

    pub fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}
