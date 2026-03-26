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
#[cfg(feature = "double-tuples")]
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
#[cfg(feature = "tripple-tuples")]
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
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5;
);

// 😈😈 Extends to up to 128 tuple arities.
#[cfg(feature = "way-too-many-tuples")]
impl_tuple_fixtures!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6, K7;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6, K7, K8;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, L0;
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, D0, D1, D2, D3, D4, D5, D6, D7, D8, D9, E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, H0, H1, H2, H3, H4, H5, H6, H7, H8, H9, I0, I1, I2, I3, I4, I5, I6, I7, I8, I9, J0, J1, J2, J3, J4, J5, J6, J7, J8, J9, K0, K1, K2, K3, K4, K5, K6, K7, K8, K9, L0, L1;
);
