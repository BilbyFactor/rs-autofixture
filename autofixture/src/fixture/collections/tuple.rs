use std::marker::PhantomData;

use crate::fixture::{Fixture, auto_fixture::AutoFixture, builder::FixtureBuilder};

pub struct Tuple2Builder<'b, A, B> {
    fixture: &'b mut Fixture,
    _phantom: PhantomData<(A, B)>,
}

impl<'b, A, B> FixtureBuilder<'b> for Tuple2Builder<'b, A, B>
where
    A: AutoFixture,
    B: AutoFixture,
{
    type F = (A, B);

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        (A::create(self.fixture), B::create(self.fixture))
    }
}

impl<A, B> AutoFixture for (A, B)
where
    A: AutoFixture,
    B: AutoFixture,
{
    type Builder<'b> = Tuple2Builder<'b, A, B>;

    fn create(f: &mut Fixture) -> Self {
        Tuple2Builder::<A, B>::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        Tuple2Builder::new(f)
    }
}
