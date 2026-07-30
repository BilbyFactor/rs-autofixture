use std::marker::PhantomData;

use crate::fixture::{Fixture, auto_fixture::AutoFixture, builder::FixtureBuilder};

/// Constructs a wrapper type directly from its inner value, used to build
/// simple pass-through wrappers (`Box<T>`, `Arc<T>`, `Mutex<T>`, etc.) on
/// top of `T`'s own `AutoFixture` implementation.
pub trait FromInner<T> {
    fn from_inner(value: T) -> Self;
}

/// A `FixtureBuilder` for any wrapper `W` that can be constructed from an
/// inner `T` via `FromInner`. Populates the inner value with `T::create`
/// and wraps it with no further configuration.
pub struct WrapperBuilder<'b, W, T> {
    fixture: &'b mut Fixture,
    _phantom: PhantomData<(W, T)>,
}

impl<'b, W, T> FixtureBuilder<'b> for WrapperBuilder<'b, W, T>
where
    W: FromInner<T> + AutoFixture,
    T: AutoFixture,
{
    type F = W;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        W::from_inner(T::create(self.fixture))
    }
}
