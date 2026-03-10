use std::marker::PhantomData;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

trait TupleFixture: Sized {
    fn create_tuple(f: &mut Fixture) -> Self;
}

pub struct TupleBuilder<'b, T> {
    fixture: &'b mut Fixture,
    _phantom: PhantomData<T>,
}

impl<'b, T> FixtureBuilder<'b> for TupleBuilder<'b, T>
where
    T: TupleFixture + AutoFixture,
{
    type F = T;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        T::create_tuple(self.fixture)
    }
}

impl<T> AutoFixture for T
where
    T: TupleFixture,
{
    type Builder<'b> = TupleBuilder<'b, T>;

    fn create(f: &mut Fixture) -> Self {
        T::create_tuple(f)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        TupleBuilder::new(f)
    }
}

macro_rules! impl_tuple_fixture {
    ($($t:ident),+ $(,)?) => {
        impl<$($t),+> TupleFixture for ($($t,)+)
        where
            $($t: AutoFixture),+
        {
            fn create_tuple(f: &mut Fixture) -> Self {
                ($($t::create(f),)+)
            }
        }
    };
}

macro_rules! impl_tuple_fixtures {
    ($($( $t:ident ),+);+ $(;)?) => {
        $(impl_tuple_fixture!($($t),+);)+
    };
}

// By default we can support up to 8 tuple arities.
// This can be increased by enabling the `double_tuples` or `tripple_tuples` features...
impl_tuple_fixtures!(
    A, B;
    A, B, C;
    A, B, C, D;
    A, B, C, D, E;
    A, B, C, D, E, F;
    A, B, C, D, E, F, G;
    A, B, C, D, E, F, G, H;
);

// 😈 Extends to up to 16 tuple arities.
#[cfg(feature = "double_tuples")]
impl_tuple_fixtures!(
    A, B, C, D, E, F, G, H, I;
    A, B, C, D, E, F, G, H, I, J;
    A, B, C, D, E, F, G, H, I, J, K;
    A, B, C, D, E, F, G, H, I, J, K, L;
    A, B, C, D, E, F, G, H, I, J, K, L, M;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P;
);

// 😈😈 Extends to up to 32 tuple arities.
#[cfg(feature = "tripple_tuples")]
impl_tuple_fixtures!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, A2, A3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, A2, A3, A4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, A2, A3, A4, A5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, A2, A3, A4, A5, A6;
);
